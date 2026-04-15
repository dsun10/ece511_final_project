// Copyright James Wainwright
//
// SPDX-License-Identifier: MPL-2.0

use std::fs;
use std::str::FromStr;

use riscv_isa::Target;

#[test]
fn main() {
    let cases = [
        ("rv32i.s", Target::from_str("RV32I").unwrap()),
        ("rv64i.s", Target::from_str("RV64I").unwrap()),
        ("s_mode.s", Target::default().with_s_mode(true)),
        ("priv.s", Target::default().with_privileged(true)),
        ("zifencei.s", Target::from_str("RV32IZifencei").unwrap()),
        ("zicsr.s", Target::from_str("RV32IZicsr").unwrap()),
        ("rv32m.s", Target::from_str("RV32IM").unwrap()),
        ("rv64m.s", Target::from_str("RV64IM").unwrap()),
        ("rv32a.s", Target::from_str("RV32IA").unwrap()),
        ("rv64a.s", Target::from_str("RV64IA").unwrap()),
        ("rv32f.s", Target::from_str("RV32IF").unwrap()),
        ("rv64f.s", Target::from_str("RV64IF").unwrap()),
        ("rv32d.s", Target::from_str("RV32ID").unwrap()),
        ("rv64d.s", Target::from_str("RV64ID").unwrap()),
        ("rv32c.s", Target::from_str("RV32IC").unwrap()),
        ("rv64c.s", Target::from_str("RV64IC").unwrap()),
        ("rv32fc.s", Target::from_str("RV32IFC").unwrap()),
        ("rv32dc.s", Target::from_str("RV32IDC").unwrap()),
        ("rv32q.s", Target::from_str("RV32IQ").unwrap()),
        ("rv64q.s", Target::from_str("RV64IQ").unwrap()),
        ("rv32zfh.s", Target::from_str("RV32IZfh").unwrap()),
        ("rv64zfh.s", Target::from_str("RV64IZfh").unwrap()),
        ("zawrs.s", Target::from_str("RV32IZawrs").unwrap()),
        ("rv32zba.s", Target::from_str("RV32IZba").unwrap()),
        ("rv64zba.s", Target::from_str("RV64IZba").unwrap()),
        ("rv32zbb.s", Target::from_str("RV32IZbb").unwrap()),
        ("rv64zbb.s", Target::from_str("RV64IZbb").unwrap()),
        ("zbc.s", Target::from_str("RV32IZbc").unwrap()),
        ("rv32zbs.s", Target::from_str("RV32IZbs").unwrap()),
        ("rv64zbs.s", Target::from_str("RV64IZbs").unwrap()),
        ("rv32zbkb.s", Target::from_str("RV32IZbkb").unwrap()),
        ("rv64zbkb.s", Target::from_str("RV64IZbkb").unwrap()),
    ];

    for (corpus, target) in cases {
        decode(corpus, &target);
    }
}

fn decode(corpus: &str, target: &Target) {
    let file = format!("tests/corpus/{corpus}");
    let file = fs::read_to_string(file).unwrap();

    for (i, line) in file.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }

        let (asm, code) = line.split_once('#').unwrap();
        let (asm, code) = (asm.trim(), code.trim());

        if asm.is_empty() {
            continue;
        }

        let got = if target.compressed() {
            let code = u16::from_str_radix(code, 16).unwrap();
            riscv_isa::decode_compressed(code, target).to_string()
        } else {
            let code = u32::from_str_radix(code, 16).unwrap();
            riscv_isa::decode_full(code, target).to_string()
        };

        assert_eq!(
            asm,
            got,
            "{corpus}:{line_num} - expected '{asm}', got '{got}'",
            line_num = i + 1
        );
    }
}
