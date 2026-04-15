<!--
SPDX-FileCopyrightText: James Wainwright

SPDX-License-Identifier: MPL-2.0
-->

# RISC-V ISA library

[![builds.sr.ht status](https://builds.sr.ht/~jwnrt/riscv-isa/commits/main.svg)](https://builds.sr.ht/~jwnrt/riscv-isa/commits/main?)

Rust crate for working with the RISC-V instruction set architecture.

This library currently supports decoding RISC-V instructions, finding CSRs, and
has partial support for producing disassembly.

Instructions from the following specs and extensions are supported:

* `RV32` and `RV64`
* `I`/`E`, `M`, `A`, `F`, `D`, `G`, `Q`, `C`, `B`
* `Zicsr`, `Zifencei`
* `Zawrs`
* `Zfh`
* `Zca`, `Zcf`, `Zcd`
* `Zba`, `Zbb`, `Zbs`, `Zbc`, `Zbkb`

## Usage

Decoding instructions:

```rust
use std::str::FromStr;
use riscv_isa::{Decoder, Instruction, Target};

let target = Target::from_str("RV32IMACZifencei_Zicsr").unwrap();
let instructions = [
    0x83, 0xa2, 0xad, 0x00, // lw x5, 10(x27)
    0x33, 0x82, 0x78, 0x03, // mul x4, x17, x23
];

let mut decoder = Decoder::from_le_bytes(target, &instructions[..]);

assert_eq!(decoder.next(), Some(Instruction::LW { rd: 5, rs1: 27, offset: 10 }));
assert_eq!(decoder.next(), Some(Instruction::MUL { rd: 4, rs1: 17, rs2: 23 }));
assert_eq!(decoder.next(), None);
```

## License

This work is distributed under the terms of the MPL-2.0 license. See LICENSES
for details. This project follows the [REUSE specification, version 3.3][reuse-3.3].

[reuse-3.3]: https://reuse.software/spec-3.3/
