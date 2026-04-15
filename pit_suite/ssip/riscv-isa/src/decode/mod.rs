// Copyright James Wainwright
//
// SPDX-License-Identifier: MPL-2.0

//! RISC-V instruction decoding.

use crate::instruction::Instruction;
use crate::target::Target;

pub(crate) mod compressed;
pub(crate) mod full;

/// Decode one instruction from the start of some little-endian bytes. The
/// decoded instruction and its length in bytes are returned if successful.
///
/// Unsupported and unknown instructions are decoded as [`Instruction::UNIMP`].
/// Compressed instructions are decompressed.
///
/// `None` is returned if there are not enough bytes given to determine the
/// instruction.
pub fn decode_le_bytes(bytes: &[u8], target: &Target) -> Option<(Instruction, usize)> {
    // Compressed instructions:
    if target.compressed() && bytes.first()? & 0b11 != 0b11 {
        let (code, _) = bytes.split_at_checked(2)?;

        let code: [u8; 2] = code.try_into().ok()?;
        let code = u16::from_le_bytes(code);

        let instruction = compressed::decode(code, target);

        return Some((instruction.into(), 2));
    }

    let (code, _) = bytes.split_at_checked(4)?;

    let code: [u8; 4] = code.try_into().ok()?;
    let code = u32::from_le_bytes(code);

    let instruction = full::decode(code, target);
    Some((instruction, 4))
}

/// RISC-V instruction decoder.
///
/// Allows for iterating over decoded instructions supported by the given
/// [`Target`].
#[derive(Clone, Debug)]
pub struct Decoder<'a> {
    /// Target configuration to decode for.
    target: Target,
    bytes: &'a [u8],
}

impl<'a> Decoder<'a> {
    /// Create a new decoder for a RISC-V [`Target`] configuration.
    pub fn from_le_bytes(target: Target, bytes: &'a [u8]) -> Decoder<'a> {
        Decoder { target, bytes }
    }
}

impl Iterator for Decoder<'_> {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        let (insn, len) = decode_le_bytes(self.bytes, &self.target)?;
        self.bytes = &self.bytes[len..];

        Some(insn)
    }
}
