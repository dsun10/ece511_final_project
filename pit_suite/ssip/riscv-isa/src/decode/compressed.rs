// SPDX-FileCopyrightText: James Wainwright
//
// SPDX-License-Identifier: MPL-2.0

//! Handling of compressed instructions from the C extension.
//!
//! This module contains a decoder for compressed instructions and functions for
//! handling their encodings.

use crate::instruction::Compressed;
use crate::{Target, Xlen};

/// Decode a 16-bit compressed instruction.
///
/// The lowest two bits must not be `0b11` which would indicate the instruction
/// is not compressed.
///
/// Compressed instructions can be decompressed using [`Instruction::from`].
///
/// Instructions not supported by the [`Target`] configuration are decoded as
/// [`Compressed::UNIMP`].
///
/// [`Instruction::from`]: crate::instruction::Instruction::from
pub fn decode(code: u16, target: &Target) -> Compressed {
    let rv32 = target.xlen == Xlen::Rv32;
    let rv64 = target.xlen == Xlen::Rv64;

    let ext_zca = target.zca;
    let ext_zcf = target.zcf;
    let ext_zcd = target.zcd;

    let opcode = code & 0b11;
    let funct3 = (code >> 13) & 0b111;

    use Compressed::*;
    match opcode {
        0b00 => {
            let rd = ((code as u32 >> 2) & 0b111) + 8;
            let rs1 = ((code as u32 >> 7) & 0b111) + 8;
            let rs2 = ((code as u32 >> 2) & 0b111) + 8;
            match funct3 {
                _ if code == 0 => UNIMP,
                0b000 if ext_zca => C_ADDI4SPN { rd, imm: imm_addi4spn(code) },
                0b001 if ext_zcd => C_FLD { frd: rd, rs1, offset: offset_lsd(code) },
                0b010 if ext_zca => C_LW { rd, rs1, offset: offset_lsw(code) },
                0b011 if ext_zcf && rv32 => C_FLW { frd: rd, rs1, offset: offset_lsw(code) },
                0b011 if ext_zca && rv64 => C_LD { rd, rs1, offset: offset_lsd(code) },
                0b101 if ext_zcd => C_FSD { rs1, frs2: rs2, offset: offset_lsd(code) },
                0b110 if ext_zca => C_SW { rs1, rs2, offset: offset_lsw(code) },
                0b111 if ext_zcf && rv32 => C_FSW { rs1, frs2: rs2, offset: offset_lsw(code) },
                0b111 if ext_zca && rv64 => C_SD { rs1, rs2, offset: offset_lsd(code) },
                _ => UNIMP,
            }
        }
        0b01 => {
            let rd = (code as u32 >> 7) & 0b11111;
            let rd_p = ((code as u32 >> 7) & 0b111) + 8;
            let rs1 = ((code as u32 >> 7) & 0b111) + 8;
            let rs2 = ((code as u32 >> 2) & 0b111) + 8;

            // These three sets of bits help us disambiguate instructions.
            let x = (code >> 2) & 0b11111;
            let y = (code >> 7) & 0b11111;
            let z = (code >> 12) & 0b1;
            match (funct3, z, y, x) {
                (0b000, _, 0, _) if ext_zca => C_NOP,
                (0b000, _, _, _) if ext_zca => C_ADDI { rd, imm: imm_li(code) },
                (0b001, _, _, _) if ext_zca && rv32 => C_JAL { offset: offset_jump(code) },
                (0b001, _, _, _) if ext_zca && rv64 => C_ADDIW { rd, imm: imm_li(code) },
                (0b010, _, _, _) if ext_zca => C_LI { rd, imm: imm_li(code) },
                (0b011, _, 2, _) if ext_zca => C_ADDI16SP { imm: imm_addi16sp(code) },
                (0b011, _, _, _) if ext_zca && rd != 0 => C_LUI { rd, imm: imm_lui(code) },
                (0b100, _, y, _) if ext_zca && y >> 3 == 0b00 && (rv64 || z==0) => {
                    C_SRLI { rd: rd_p, shamt: imm_li(code) as u32 }
                }
                (0b100, _, y, _) if ext_zca && y >> 3 == 0b01 && (rv64 || z==0) => {
                    C_SRAI { rd: rd_p, shamt: imm_li(code) as u32 }
                }
                (0b100, _, y, _) if ext_zca && y >> 3 == 0b10 => {
                    C_ANDI { rd: rd_p, imm: imm_li(code) }
                }
                (0b100, 0, y, z) if ext_zca && y >> 3 == 0b11 => match z >> 3 {
                    0b00 => C_SUB { rd: rd_p, rs2 },
                    0b01 => C_XOR { rd: rd_p, rs2 },
                    0b10 => C_OR { rd: rd_p, rs2 },
                    0b11 => C_AND { rd: rd_p, rs2 },
                    _ => UNIMP,
                },
                (0b100, 1, y, z) if ext_zca && y >> 3 == 0b11 => match z >> 3 {
                    0b00 => C_SUBW { rd: rd_p, rs2 },
                    0b01 => C_ADDW { rd: rd_p, rs2 },
                    _ => UNIMP,
                },
                (0b101, _, _, _) if ext_zca => C_J { offset: offset_jump(code) },
                (0b110, _, _, _) if ext_zca => C_BEQZ { rs1, offset: offset_branch(code) },
                (0b111, _, _, _) if ext_zca => C_BNEZ { rs1, offset: offset_branch(code) },
                _ => UNIMP,
            }
        }
        0b10 => {
            let rd = (code as u32 >> 7) & 0b11111;
            let rs1 = (code as u32 >> 7) & 0b11111;
            let rs2 = (code as u32 >> 2) & 0b11111;
            let frd = rd;
            let frs2 = rs2;

            // As above, these three sets of bits help us disambiguate instructions.
            let x = (code >> 2) & 0b11111;
            let y = (code >> 7) & 0b11111;
            let z = (code >> 12) & 0b1;
            match (funct3, z, y, x) {
                (0b000, _, _, _) if ext_zca && (rv64 || z==0)  =>
                    C_SLLI { rd, shamt: imm_li(code) as u32 },
                (0b001, _, _, _) if ext_zcd => C_FLDSP { frd, offset: offset_ldsp(code) },
                (0b010, _, _, _) if ext_zca => C_LWSP { rd, offset: offset_lwsp(code) },
                (0b011, _, _, _) if ext_zcf && rv32 => C_FLWSP { frd, offset: offset_lwsp(code) },
                (0b011, _, _, _) if ext_zca && rv64 => C_LDSP { rd, offset: offset_ldsp(code) },
                (0b100, 0, _, 0) if ext_zca && rs1!=0 => C_JR { rs1 },
                (0b100, 0, _, _) if ext_zca => C_MV { rd, rs2 },
                (0b100, 1, 0, 0) if ext_zca => C_EBREAK,
                (0b100, 1, _, 0) if ext_zca && rs1!=0 => C_JALR { rs1 },
                (0b100, 1, _, _) if ext_zca => C_ADD { rd, rs2 },
                (0b101, _, _, _) if ext_zcd => C_FSDSP { frs2, offset: offset_sdsp(code) },
                (0b110, _, _, _) if ext_zca => C_SWSP { rs2, offset: offset_swsp(code) },
                (0b111, _, _, _) if ext_zcf && rv32 => C_FSWSP { frs2, offset: offset_swsp(code) },
                (0b111, _, _, _) if ext_zca && rv64 => C_SDSP { rs2, offset: offset_sdsp(code) },
                _ => UNIMP,
            }
        }
        _ => UNIMP,
    }
}

fn offset_lwsp(code: u16) -> i32 {
    let code = code as i32;
    (((code >> 12) & 0b1) << 5) // Zero extended.
        | (((code >> 4) & 0b111) << 2)
        | (((code >> 2) & 0b11) << 6)
}

fn offset_ldsp(code: u16) -> i32 {
    let code = code as i32;
    (((code >> 12) & 0b1) << 5) // Zero extended.
        | (((code >> 5) & 0b11) << 3)
        | (((code >> 2) & 0b111) << 6)
}

fn offset_swsp(code: u16) -> i32 {
    let code = code as i32;
    (((code >> 9) & 0b1111) << 2) // Zero extended.
        | (((code >> 7) & 0b11) << 6)
}

fn offset_sdsp(code: u16) -> i32 {
    let code = code as i32;
    (((code >> 10) & 0b111) << 3) // Zero extended.
        | (((code >> 7) & 0b111) << 6)
}

fn offset_lsw(code: u16) -> i32 {
    let code = code as i32;
    (((code >> 10) & 0b111) << 3) // Zero extended.
        | (((code >> 6) & 0b1) << 2)
        | (((code >> 5) & 0b1) << 6)
}

fn offset_lsd(code: u16) -> i32 {
    let code = code as i32;
    (((code >> 10) & 0b111) << 3) // Zero extended.
        | (((code >> 5) & 0b11) << 6)
}

fn offset_jump(code: u16) -> i32 {
    let code = code as i32;
    (code << (31 - 12) >> (31 - 12) /* <- sign extension */ >> 12 << 11) // 1 bit
        | (((code >> 11) & 0b1) << 4)
        | (((code >> 9) & 0b11) << 8)
        | (((code >> 8) & 0b1) << 10)
        | (((code >> 7) & 0b1) << 6)
        | (((code >> 6) & 0b1) << 7)
        | (((code >> 3) & 0b111) << 1)
        | (((code >> 2) & 0b1) << 5)
}

fn offset_branch(code: u16) -> i32 {
    let code = code as i32;
    (code << (31 - 12) >> (31 - 12) /* <- sign extension */ >> 12 << 8) // 1 bit
        | (((code >> 10) & 0b11) << 3)
        | (((code >> 5) & 0b11) << 6)
        | (((code >> 3) & 0b11) << 1)
        | (((code >> 2) & 0b1) << 5)
}

fn imm_li(code: u16) -> i32 {
    let code = code as i32;
    (code << (31 - 12) >> (31 - 12) /* <- sign extension */ >> 12 << 5) // 1 bit
        | (code >> 2) & 0b11111
}

fn imm_lui(code: u16) -> u32 {
    let code = code as i32;
    let imm = (code << (31 - 12) >> (31 - 12) /* <- sign extension */ >> 12 << 5) // 1 bit
        | (code >> 2) & 0b11111;
    // We only want to sign extend up to the 20th bit, cut the upper 12.
    (imm as u32) << (31 - 19) >> (31 - 19)
}

fn imm_addi16sp(code: u16) -> i32 {
    let code = code as i32;
    (code << (31 - 12) >> (31 - 12) /* <- sign extension */ >> 12 << 9) // 1 bit
        | (((code >> 6) & 0b1) << 4)
        | (((code >> 5) & 0b1) << 6)
        | (((code >> 3) & 0b11) << 7)
        | (((code >> 2) & 0b1) << 5)
}

fn imm_addi4spn(code: u16) -> i32 {
    let code = code as i32;
    (((code >> 11) & 0b11) << 4) // Zero extended.
        | (((code >> 7) & 0b1111) << 6)
        | (((code >> 6) & 0b1) << 2)
        | (((code >> 5) & 0b1) << 3)
}
