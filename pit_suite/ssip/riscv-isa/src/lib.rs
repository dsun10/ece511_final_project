// Copyright James Wainwright
//
// SPDX-License-Identifier: MPL-2.0

//! RISC-V instruction set architecture library.
//!
//! Supports decoding RV32 and RV64 from the following specs and extensions:
//!
//! * `I`/`E`, `M`, `A`, `F`, `D`, `G`, `Q`, `C`, `B`
//! * `Zicsr`, `Zifencei`
//! * `Zawrs`
//! * `Zfh`
//! * `Zca`, `Zcf`, `Zcd`
//! * `Zba`, `Zbb`, `Zbs`, `Zbkb`, `Zbc`
//!
//! # Example
//!
//! ```rust
//! use std::str::FromStr;
//! use riscv_isa::{Decoder, Instruction, Target};
//!
//! let target = Target::from_str("RV32IMACZifencei_Zicsr").unwrap();
//! let instructions = [
//!     0x83, 0xa2, 0xad, 0x00, // lw x5, 10(x27)
//!     0x33, 0x82, 0x78, 0x03, // mul x4, x17, x23
//! ];
//!
//! let mut decoder = Decoder::from_le_bytes(target, &instructions[..]);
//!
//! assert_eq!(decoder.next(), Some(Instruction::LW { rd: 5, rs1: 27, offset: 10 }));
//! assert_eq!(decoder.next(), Some(Instruction::MUL { rd: 4, rs1: 17, rs2: 23 }));
//! assert_eq!(decoder.next(), None);
//! ```

mod asm;
mod csr;
mod decode;
mod instruction;
mod target;

pub use csr::Csr;
pub use decode::compressed::decode as decode_compressed;
pub use decode::full::decode as decode_full;
pub use decode::{decode_le_bytes, Decoder};
pub use instruction::{Compressed, Instruction};
pub use target::{Target, Xlen};

/// Privilege mode, e.g. `User` or `Machine`.
///
/// Privileges can be compared, for example:
///
/// ```rust
/// # use riscv_isa::Privilege;
/// assert!(Privilege::User < Privilege::Machine);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Privilege {
    User,
    Supervisor,
    Hypervisor,
    Machine,
    Debug,
}
