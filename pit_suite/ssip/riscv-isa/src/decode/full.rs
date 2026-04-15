// Copyright James Wainwright
//
// SPDX-License-Identifier: MPL-2.0

//! Decoding of full (not compressed) 32-bit intsructions.

use crate::instruction::{Instruction, Iorw};
use crate::target::{Target, Xlen};

/// Decode a 32-bit instruction.
///
/// Unsupported and unknown instructions are decoded to [`Instruction::UNIMP`].
pub fn decode(code: u32, target: &Target) -> Instruction {
    // Extract target features for shorter `if`s later on.
    let rv32 = target.xlen == Xlen::Rv32;
    let rv64 = target.xlen == Xlen::Rv64;
    let s_mode = target.supervisor_mode;
    let ext_zifencei = target.zifencei;
    let ext_zicsr = target.zicsr;
    let ext_m = target.m;
    let ext_a = target.a;
    let ext_f = target.f;
    let ext_d = target.d;
    let ext_q = target.q;
    let ext_zfh = target.zfh;
    let ext_zawrs = target.zawrs;
    let ext_zba = target.zba;
    let ext_zbb = target.zbb;
    let ext_zbc = target.zbc;
    let ext_zbs = target.zbs;
    let ext_zbkb = target.zbkb;

    // Pre-compute the components of the encoded instruction.
    let Encoding {
        opcode,
        funct3,
        funct7,
        rd,
        rs1,
        rs2,
        rs3,
        rm,
        uimm,
        rl,
        aq,
        csr,
        succ,
        pred,
        i_imm,
        s_imm,
        b_imm,
        u_imm,
        j_imm,
        shamt32,
        shamt64,
    } = Encoding::from(code);

    // Aliases:
    let frd = rd;
    let frs1 = rs1;
    let frs2 = rs2;
    let frs3 = rs3;
    let shamt = match target.xlen {
        Xlen::Rv32 => shamt32,
        Xlen::Rv64 => shamt64,
    };

    // Mega decoding table!
    //
    // Most instructions can be identified by the `opcode`, `funct3` and `funct7` bits
    // depending on their encodings. Some require extra bits to be fixed.
    //
    // This decoding method gives good performance and flexibility. Comparing the code
    // to a huge set of masks gives a more readable table but is not as fast.
    use Instruction::*;
    match (opcode, funct3, funct7) {
        // Base instruction set:
        (Op::LUI, _, _) => LUI { rd, imm: u_imm },
        (Op::AUIPC, _, _) => AUIPC { rd, imm: u_imm },
        (Op::JAL, _, _) => JAL { rd, offset: j_imm },
        (Op::JALR, 0b000, _) => JALR { rd, rs1, offset: i_imm },
        (Op::BRANCH, 0b000, _) => BEQ { rs1, rs2, offset: b_imm },
        (Op::BRANCH, 0b001, _) => BNE { rs1, rs2, offset: b_imm },
        (Op::BRANCH, 0b100, _) => BLT { rs1, rs2, offset: b_imm },
        (Op::BRANCH, 0b101, _) => BGE { rs1, rs2, offset: b_imm },
        (Op::BRANCH, 0b110, _) => BLTU { rs1, rs2, offset: b_imm },
        (Op::BRANCH, 0b111, _) => BGEU { rs1, rs2, offset: b_imm },
        (Op::LOAD, 0b000, _) => LB { rd, rs1, offset: i_imm },
        (Op::LOAD, 0b001, _) => LH { rd, rs1, offset: i_imm },
        (Op::LOAD, 0b010, _) => LW { rd, rs1, offset: i_imm },
        (Op::LOAD, 0b100, _) => LBU { rd, rs1, offset: i_imm },
        (Op::LOAD, 0b101, _) => LHU { rd, rs1, offset: i_imm },
        (Op::STORE, 0b000, _) => SB { rs1, rs2, offset: s_imm },
        (Op::STORE, 0b001, _) => SH { rs1, rs2, offset: s_imm },
        (Op::STORE, 0b010, _) => SW { rs1, rs2, offset: s_imm },
        (Op::OP_IMM, 0b000, _) => ADDI { rd, rs1, imm: i_imm },
        (Op::OP_IMM, 0b010, _) => SLTI { rd, rs1, imm: i_imm },
        (Op::OP_IMM, 0b011, _) => SLTIU { rd, rs1, imm: i_imm },
        (Op::OP_IMM, 0b100, _) => XORI { rd, rs1, imm: i_imm },
        (Op::OP_IMM, 0b110, _) => ORI { rd, rs1, imm: i_imm },
        (Op::OP_IMM, 0b111, _) => ANDI { rd, rs1, imm: i_imm },
        (Op::OP, 0b000, 0b0000000) => ADD { rd, rs1, rs2 },
        (Op::OP, 0b000, 0b0100000) => SUB { rd, rs1, rs2 },
        (Op::OP, 0b001, 0b0000000) => SLL { rd, rs1, rs2 },
        (Op::OP, 0b010, 0b0000000) => SLT { rd, rs1, rs2 },
        (Op::OP, 0b011, 0b0000000) => SLTU { rd, rs1, rs2 },
        (Op::OP, 0b100, 0b0000000) => XOR { rd, rs1, rs2 },
        (Op::OP, 0b101, 0b0000000) => SRL { rd, rs1, rs2 },
        (Op::OP, 0b101, 0b0100000) => SRA { rd, rs1, rs2 },
        (Op::OP, 0b110, 0b0000000) => OR { rd, rs1, rs2 },
        (Op::OP, 0b111, 0b0000000) => AND { rd, rs1, rs2 },
        (Op::MISC_MEM, 0b000, _) if (code >> 28) & 0b1111 == 0b000 => FENCE { succ, pred },
        (Op::MISC_MEM, 0b010, _) => CBO, // general bucket, no validation enforced
        (Op::SYSTEM, 0b000, 0b0000000) if rd == 0b0 && rs1 == 0b0 && rs2 == 0b0 => ECALL,
        (Op::SYSTEM, 0b000, 0b0000000) if rd == 0b0 && rs1 == 0b0 && rs2 == 0b1 => EBREAK,
        // RV32 base instruction set:
        (Op::OP_IMM, 0b001, 0b0000000) if rv32 => SLLI { rd, rs1, shamt },
        (Op::OP_IMM, 0b101, 0b0000000) if rv32 => SRLI { rd, rs1, shamt },
        (Op::OP_IMM, 0b101, 0b0100000) if rv32 => SRAI { rd, rs1, shamt },
        // RV64 base instruction set:
        (Op::LOAD, 0b110, _) if rv64 => LWU { rd, rs1, offset: i_imm },
        (Op::LOAD, 0b011, _) if rv64 => LD { rd, rs1, offset: i_imm },
        (Op::STORE, 0b011, _) if rv64 => SD { rs1, rs2, offset: s_imm },
        (Op::OP_IMM, 0b001, f7) if rv64 && f7 >> 1 == 0b000000 => SLLI { rd, rs1, shamt },
        (Op::OP_IMM, 0b101, f7) if rv64 && f7 >> 1 == 0b000000 => SRLI { rd, rs1, shamt },
        (Op::OP_IMM, 0b101, f7) if rv64 && f7 >> 1 == 0b010000 => SRAI { rd, rs1, shamt },
        (Op::OP_IMM_32, 0b000, _) if rv64 => ADDIW { rd, rs1, imm: i_imm },
        (Op::OP_IMM_32, 0b001, 0b0000000) if rv64 => SLLIW { rd, rs1, shamt: shamt32 },
        (Op::OP_IMM_32, 0b101, 0b0000000) if rv64 => SRLIW { rd, rs1, shamt: shamt32 },
        (Op::OP_IMM_32, 0b101, 0b0100000) if rv64 => SRAIW { rd, rs1, shamt: shamt32 },
        (Op::OP_32, 0b000, 0b0000000) if rv64 => ADDW { rd, rs1, rs2 },
        (Op::OP_32, 0b000, 0b0100000) if rv64 => SUBW { rd, rs1, rs2 },
        (Op::OP_32, 0b001, 0b0000000) if rv64 => SLLW { rd, rs1, rs2 },
        (Op::OP_32, 0b101, 0b0000000) if rv64 => SRLW { rd, rs1, rs2 },
        (Op::OP_32, 0b101, 0b0100000) if rv64 => SRAW { rd, rs1, rs2 },
        // Privileged instructions:
        (Op::SYSTEM, 0b000, 0b0011000) if rd == 0b0 && rs1 == 0b0 && rs2 == 0b010 => MRET,
        (Op::SYSTEM, 0b000, 0b0001000) if rd == 0b0 && rs1 == 0b0 && rs2 == 0b101 => WFI,
        // S-mode instructions:
        (Op::SYSTEM, 0b000, 0b0001000) if s_mode && rd == 0b0 && rs1 == 0b0 && rs2 == 0b10 => SRET,
        (Op::SYSTEM, 0b000, 0b0001001) if s_mode && rd == 0b0 => SFENCE_VMA { rs1, rs2 },
        // Zifencei extension:
        (Op::MISC_MEM, 0b001, _) if ext_zifencei => FENCE_I,
        // Zicsr extension:
        (Op::SYSTEM, 0b001, _) if ext_zicsr => CSRRW { rd, csr, rs1 },
        (Op::SYSTEM, 0b010, _) if ext_zicsr => CSRRS { rd, csr, rs1 },
        (Op::SYSTEM, 0b011, _) if ext_zicsr => CSRRC { rd, csr, rs1 },
        (Op::SYSTEM, 0b101, _) if ext_zicsr => CSRRWI { rd, csr, uimm },
        (Op::SYSTEM, 0b110, _) if ext_zicsr => CSRRSI { rd, csr, uimm },
        (Op::SYSTEM, 0b111, _) if ext_zicsr => CSRRCI { rd, csr, uimm },
        // M extension:
        (Op::OP, 0b000, 0b0000001) if ext_m => MUL { rd, rs1, rs2 },
        (Op::OP, 0b001, 0b0000001) if ext_m => MULH { rd, rs1, rs2 },
        (Op::OP, 0b010, 0b0000001) if ext_m => MULHSU { rd, rs1, rs2 },
        (Op::OP, 0b011, 0b0000001) if ext_m => MULHU { rd, rs1, rs2 },
        (Op::OP, 0b100, 0b0000001) if ext_m => DIV { rd, rs1, rs2 },
        (Op::OP, 0b101, 0b0000001) if ext_m => DIVU { rd, rs1, rs2 },
        (Op::OP, 0b110, 0b0000001) if ext_m => REM { rd, rs1, rs2 },
        (Op::OP, 0b111, 0b0000001) if ext_m => REMU { rd, rs1, rs2 },
        // RV64M extension:
        (Op::OP_32, 0b000, 0b0000001) if rv64 && ext_m => MULW { rd, rs1, rs2 },
        (Op::OP_32, 0b100, 0b0000001) if rv64 && ext_m => DIVW { rd, rs1, rs2 },
        (Op::OP_32, 0b101, 0b0000001) if rv64 && ext_m => DIVUW { rd, rs1, rs2 },
        (Op::OP_32, 0b110, 0b0000001) if rv64 && ext_m => REMW { rd, rs1, rs2 },
        (Op::OP_32, 0b111, 0b0000001) if rv64 && ext_m => REMUW { rd, rs1, rs2 },
        // A extension:
        (Op::AMO, 0b010, f7) if ext_a && f7 >> 2 == 0b00010 && rs2 == 0b00000 => {
            LR_W { rd, rs1, rl, aq }
        }
        (Op::AMO, 0b010, f7) if ext_a && f7 >> 2 == 0b00011 => SC_W { rd, rs1, rs2, rl, aq },
        (Op::AMO, 0b010, f7) if ext_a && f7 >> 2 == 0b00001 => AMOSWAP_W { rd, rs1, rs2, rl, aq },
        (Op::AMO, 0b010, f7) if ext_a && f7 >> 2 == 0b00000 => AMOADD_W { rd, rs1, rs2, rl, aq },
        (Op::AMO, 0b010, f7) if ext_a && f7 >> 2 == 0b00100 => AMOXOR_W { rd, rs1, rs2, rl, aq },
        (Op::AMO, 0b010, f7) if ext_a && f7 >> 2 == 0b01100 => AMOAND_W { rd, rs1, rs2, rl, aq },
        (Op::AMO, 0b010, f7) if ext_a && f7 >> 2 == 0b01000 => AMOOR_W { rd, rs1, rs2, rl, aq },
        (Op::AMO, 0b010, f7) if ext_a && f7 >> 2 == 0b10000 => AMOMIN_W { rd, rs1, rs2, rl, aq },
        (Op::AMO, 0b010, f7) if ext_a && f7 >> 2 == 0b10100 => AMOMAX_W { rd, rs1, rs2, rl, aq },
        (Op::AMO, 0b010, f7) if ext_a && f7 >> 2 == 0b11000 => AMOMINU_W { rd, rs1, rs2, rl, aq },
        (Op::AMO, 0b010, f7) if ext_a && f7 >> 2 == 0b11100 => AMOMAXU_W { rd, rs1, rs2, rl, aq },
        // RV64A extension:
        (Op::AMO, 0b011, f7) if rv64 && ext_a && f7 >> 2 == 0b00010 && rs2 == 0b00000 => {
            LR_D { rd, rs1, rl, aq }
        }
        (Op::AMO, 0b011, f7) if rv64 && ext_a && f7 >> 2 == 0b00011 => {
            SC_D { rd, rs1, rs2, rl, aq }
        }
        (Op::AMO, 0b011, f7) if rv64 && ext_a && f7 >> 2 == 0b00001 => {
            AMOSWAP_D { rd, rs1, rs2, rl, aq }
        }
        (Op::AMO, 0b011, f7) if rv64 && ext_a && f7 >> 2 == 0b00000 => {
            AMOADD_D { rd, rs1, rs2, rl, aq }
        }
        (Op::AMO, 0b011, f7) if rv64 && ext_a && f7 >> 2 == 0b00100 => {
            AMOXOR_D { rd, rs1, rs2, rl, aq }
        }
        (Op::AMO, 0b011, f7) if rv64 && ext_a && f7 >> 2 == 0b01100 => {
            AMOAND_D { rd, rs1, rs2, rl, aq }
        }
        (Op::AMO, 0b011, f7) if rv64 && ext_a && f7 >> 2 == 0b01000 => {
            AMOOR_D { rd, rs1, rs2, rl, aq }
        }
        (Op::AMO, 0b011, f7) if rv64 && ext_a && f7 >> 2 == 0b10000 => {
            AMOMIN_D { rd, rs1, rs2, rl, aq }
        }
        (Op::AMO, 0b011, f7) if rv64 && ext_a && f7 >> 2 == 0b10100 => {
            AMOMAX_D { rd, rs1, rs2, rl, aq }
        }
        (Op::AMO, 0b011, f7) if rv64 && ext_a && f7 >> 2 == 0b11000 => {
            AMOMINU_D { rd, rs1, rs2, rl, aq }
        }
        (Op::AMO, 0b011, f7) if rv64 && ext_a && f7 >> 2 == 0b11100 => {
            AMOMAXU_D { rd, rs1, rs2, rl, aq }
        }
        // F extension:
        (Op::LOAD_FP, 0b010, _) if ext_f => FLW { frd, rs1, offset: i_imm },
        (Op::STORE_FP, 0b010, _) if ext_f => FSW { rs1, frs2, offset: s_imm },
        (Op::MADD, _, f7) if ext_f && f7 & 0b11 == 0b00 => FMADD_S { frd, rm, frs1, frs2, frs3 },
        (Op::MSUB, _, f7) if ext_f && f7 & 0b11 == 0b00 => FMSUB_S { frd, rm, frs1, frs2, frs3 },
        (Op::NMSUB, _, f7) if ext_f && f7 & 0b11 == 0b00 => FNMSUB_S { frd, rm, frs1, frs2, frs3 },
        (Op::NMADD, _, f7) if ext_f && f7 & 0b11 == 0b00 => FNMADD_S { frd, rm, frs1, frs2, frs3 },
        (Op::OP_FP, _, 0b0000000) if ext_f => FADD_S { frd, rm, frs1, frs2 },
        (Op::OP_FP, 0b001, 0b1110000) if ext_f && rs2 == 0b00000 => FCLASS_S { rd, frs1 },
        (Op::OP_FP, _, 0b1101000) if ext_f && rs2 == 0b00000 => FCVT_S_W { frd, rm, rs1 },
        (Op::OP_FP, _, 0b1101000) if ext_f && rs2 == 0b00001 => FCVT_S_WU { frd, rm, rs1 },
        (Op::OP_FP, _, 0b1100000) if ext_f && rs2 == 0b00000 => FCVT_W_S { rd, rm, frs1 },
        (Op::OP_FP, _, 0b1100000) if ext_f && rs2 == 0b00001 => FCVT_WU_S { rd, rm, frs1 },
        (Op::OP_FP, _, 0b0001100) if ext_f => FDIV_S { frd, rm, frs1, frs2 },
        (Op::OP_FP, _, 0b0000100) if ext_f => FSUB_S { frd, rm, frs1, frs2 },
        (Op::OP_FP, _, 0b0001000) if ext_f => FMUL_S { frd, rm, frs1, frs2 },
        (Op::OP_FP, _, 0b0101100) if ext_f && rs2 == 0b00000 => FSQRT_S { frd, rm, frs1 },
        (Op::OP_FP, 0b000, 0b0010000) if ext_f => FSGNJ_S { frd, frs1, frs2 },
        (Op::OP_FP, 0b001, 0b0010000) if ext_f => FSGNJN_S { frd, frs1, frs2 },
        (Op::OP_FP, 0b010, 0b0010000) if ext_f => FSGNJX_S { frd, frs1, frs2 },
        (Op::OP_FP, 0b000, 0b0010100) if ext_f => FMIN_S { frd, frs1, frs2 },
        (Op::OP_FP, 0b001, 0b0010100) if ext_f => FMAX_S { frd, frs1, frs2 },
        (Op::OP_FP, 0b000, 0b1110000) if ext_f && rs2 == 0b00000 => FMV_X_W { rd, frs1 },
        (Op::OP_FP, 0b010, 0b1010000) if ext_f => FEQ_S { rd, frs1, frs2 },
        (Op::OP_FP, 0b001, 0b1010000) if ext_f => FLT_S { rd, frs1, frs2 },
        (Op::OP_FP, 0b000, 0b1010000) if ext_f => FLE_S { rd, frs1, frs2 },
        (Op::OP_FP, 0b000, 0b1111000) if ext_f && rs2 == 0b00000 => FMV_W_X { frd, rs1 },
        // RV64F extension:
        (Op::OP_FP, _, 0b1100000) if rv64 && ext_f && rs2 == 0b00010 => FCVT_L_S { rd, rm, frs1 },
        (Op::OP_FP, _, 0b1100000) if rv64 && ext_f && rs2 == 0b00011 => FCVT_LU_S { rd, rm, frs1 },
        (Op::OP_FP, _, 0b1101000) if rv64 && ext_f && rs2 == 0b00010 => FCVT_S_L { frd, rm, rs1 },
        (Op::OP_FP, _, 0b1101000) if rv64 && ext_f && rs2 == 0b00011 => FCVT_S_LU { frd, rm, rs1 },
        // D extension:
        (Op::LOAD_FP, 0b011, _) if ext_d => FLD { frd, rs1, offset: i_imm },
        (Op::STORE_FP, 0b011, _) if ext_d => FSD { rs1, frs2, offset: s_imm },
        (Op::MADD, _, f7) if ext_d && f7 & 0b11 == 0b01 => FMADD_D { frd, rm, frs1, frs2, frs3 },
        (Op::MSUB, _, f7) if ext_d && f7 & 0b11 == 0b01 => FMSUB_D { frd, rm, frs1, frs2, frs3 },
        (Op::NMSUB, _, f7) if ext_d && f7 & 0b11 == 0b01 => FNMSUB_D { frd, rm, frs1, frs2, frs3 },
        (Op::NMADD, _, f7) if ext_d && f7 & 0b11 == 0b01 => FNMADD_D { frd, rm, frs1, frs2, frs3 },
        (Op::OP_FP, _, 0b0000001) if ext_d => FADD_D { frd, rm, frs1, frs2 },
        (Op::OP_FP, _, 0b0000101) if ext_d => FSUB_D { frd, rm, frs1, frs2 },
        (Op::OP_FP, _, 0b0001001) if ext_d => FMUL_D { frd, rm, frs1, frs2 },
        (Op::OP_FP, _, 0b0001101) if ext_d => FDIV_D { frd, rm, frs1, frs2 },
        (Op::OP_FP, _, 0b0101101) if ext_d && rs2 == 0b00000 => FSQRT_D { frd, rm, frs1 },
        (Op::OP_FP, 0b000, 0b0010001) if ext_d => FSGNJ_D { frd, frs1, frs2 },
        (Op::OP_FP, 0b001, 0b0010001) if ext_d => FSGNJN_D { frd, frs1, frs2 },
        (Op::OP_FP, 0b010, 0b0010001) if ext_d => FSGNJX_D { frd, frs1, frs2 },
        (Op::OP_FP, 0b000, 0b0010101) if ext_d => FMIN_D { frd, frs1, frs2 },
        (Op::OP_FP, 0b001, 0b0010101) if ext_d => FMAX_D { frd, frs1, frs2 },
        (Op::OP_FP, _, 0b0100000) if ext_d && rs2 == 0b00001 => FCVT_S_D { frd, rm, frs1 },
        (Op::OP_FP, _, 0b0100001) if ext_d && rs2 == 0b00000 => FCVT_D_S { frd, rm, frs1 },
        (Op::OP_FP, 0b010, 0b1010001) if ext_d => FEQ_D { rd, frs1, frs2 },
        (Op::OP_FP, 0b001, 0b1010001) if ext_d => FLT_D { rd, frs1, frs2 },
        (Op::OP_FP, 0b000, 0b1010001) if ext_d => FLE_D { rd, frs1, frs2 },
        (Op::OP_FP, 0b001, 0b1110001) if ext_d && rs2 == 0b00000 => FCLASS_D { rd, frs1 },
        (Op::OP_FP, _, 0b1100001) if ext_d && rs2 == 0b00000 => FCVT_W_D { rd, rm, frs1 },
        (Op::OP_FP, _, 0b1100001) if ext_d && rs2 == 0b00001 => FCVT_WU_D { rd, rm, frs1 },
        (Op::OP_FP, _, 0b1101001) if ext_d && rs2 == 0b00000 => FCVT_D_W { frd, rm, rs1 },
        (Op::OP_FP, _, 0b1101001) if ext_d && rs2 == 0b00001 => FCVT_D_WU { frd, rm, rs1 },
        // RV64D extension:
        (Op::OP_FP, _, 0b1100001) if rv64 && ext_d && rs2 == 0b00010 => FCVT_L_D { rd, rm, frs1 },
        (Op::OP_FP, _, 0b1100001) if rv64 && ext_d && rs2 == 0b00011 => FCVT_LU_D { rd, rm, frs1 },
        (Op::OP_FP, 0b000, 0b1110001) if rv64 && ext_d && rs2 == 0b00000 => FMV_X_D { rd, frs1 },
        (Op::OP_FP, _, 0b1101001) if rv64 && ext_d && rs2 == 0b00010 => FCVT_D_L { frd, rm, rs1 },
        (Op::OP_FP, _, 0b1101001) if rv64 && ext_d && rs2 == 0b00011 => FCVT_D_LU { frd, rm, rs1 },
        (Op::OP_FP, 0b000, 0b1111001) if rv64 && ext_d && rs2 == 0b00000 => FMV_D_X { frd, rs1 },
        // Q extension:
        (Op::LOAD_FP, 0b100, _) if ext_q => FLQ { frd, rs1, offset: i_imm },
        (Op::STORE_FP, 0b100, _) if ext_q => FSQ { rs1, frs2, offset: s_imm },
        (Op::MADD, _, f7) if ext_q && f7 & 0b11 == 0b11 => FMADD_Q { frd, rm, frs1, frs2, frs3 },
        (Op::MSUB, _, f7) if ext_q && f7 & 0b11 == 0b11 => FMSUB_Q { frd, rm, frs1, frs2, frs3 },
        (Op::NMSUB, _, f7) if ext_q && f7 & 0b11 == 0b11 => FNMSUB_Q { frd, rm, frs1, frs2, frs3 },
        (Op::NMADD, _, f7) if ext_q && f7 & 0b11 == 0b11 => FNMADD_Q { frd, rm, frs1, frs2, frs3 },
        (Op::OP_FP, _, 0b0000011) if ext_q => FADD_Q { frd, rm, frs1, frs2 },
        (Op::OP_FP, _, 0b0000111) if ext_q => FSUB_Q { frd, rm, frs1, frs2 },
        (Op::OP_FP, _, 0b0001011) if ext_q => FMUL_Q { frd, rm, frs1, frs2 },
        (Op::OP_FP, _, 0b0001111) if ext_q => FDIV_Q { frd, rm, frs1, frs2 },
        (Op::OP_FP, _, 0b0101111) if ext_q && frs2 == 0b00000 => FSQRT_Q { frd, rm, frs1 },
        (Op::OP_FP, 0b000, 0b0010011) if ext_q => FSGNJ_Q { frd, frs1, frs2 },
        (Op::OP_FP, 0b001, 0b0010011) if ext_q => FSGNJN_Q { frd, frs1, frs2 },
        (Op::OP_FP, 0b010, 0b0010011) if ext_q => FSGNJX_Q { frd, frs1, frs2 },
        (Op::OP_FP, 0b000, 0b0010111) if ext_q => FMIN_Q { frd, frs1, frs2 },
        (Op::OP_FP, 0b001, 0b0010111) if ext_q => FMAX_Q { frd, frs1, frs2 },
        (Op::OP_FP, _, 0b0100000) if ext_q && frs2 == 0b000011 => FCVT_S_Q { frd, rm, frs1 },
        (Op::OP_FP, _, 0b0100011) if ext_q && frs2 == 0b000000 => FCVT_Q_S { frd, rm, frs1 },
        (Op::OP_FP, _, 0b0100001) if ext_q && frs2 == 0b000011 => FCVT_D_Q { frd, rm, frs1 },
        (Op::OP_FP, _, 0b0100011) if ext_q && frs2 == 0b000001 => FCVT_Q_D { frd, rm, frs1 },
        (Op::OP_FP, 0b010, 0b1010011) if ext_q => FEQ_Q { rd, frs1, frs2 },
        (Op::OP_FP, 0b001, 0b1010011) if ext_q => FLT_Q { rd, frs1, frs2 },
        (Op::OP_FP, 0b000, 0b1010011) if ext_q => FLE_Q { rd, frs1, frs2 },
        (Op::OP_FP, 0b001, 0b1110011) if ext_q && rs2 == 0b00000 => FCLASS_Q { rd, frs1 },
        (Op::OP_FP, _, 0b1100011) if ext_q && rs2 == 0b00000 => FCVT_W_Q { rd, rm, frs1 },
        (Op::OP_FP, _, 0b1100011) if ext_q && rs2 == 0b00001 => FCVT_WU_Q { rd, rm, frs1 },
        (Op::OP_FP, _, 0b1101011) if ext_q && rs2 == 0b00000 => FCVT_Q_W { frd, rm, rs1 },
        (Op::OP_FP, _, 0b1101011) if ext_q && rs2 == 0b00001 => FCVT_Q_WU { frd, rm, rs1 },
        // RV64Q extension:
        (Op::OP_FP, _, 0b1100011) if rv64 && ext_q && rs2 == 0b00010 => FCVT_L_Q { rd, rm, frs1 },
        (Op::OP_FP, _, 0b1100011) if rv64 && ext_q && rs2 == 0b00011 => FCVT_LU_Q { rd, rm, frs1 },
        (Op::OP_FP, _, 0b1101011) if rv64 && ext_q && rs2 == 0b00010 => FCVT_Q_L { frd, rm, rs1 },
        (Op::OP_FP, _, 0b1101011) if rv64 && ext_q && rs2 == 0b00011 => FCVT_Q_LU { frd, rm, rs1 },
        // Zfh extension:
        (Op::LOAD_FP, 0b001, _) if ext_zfh => FLH { frd, rs1, offset: i_imm },
        (Op::STORE_FP, 0b001, _) if ext_zfh => FSH { rs1, frs2, offset: s_imm },
        (Op::MADD, _, f7) if ext_zfh && f7 & 0b11 == 0b10 => FMADD_H { frd, rm, frs1, frs2, frs3 },
        (Op::MSUB, _, f7) if ext_zfh && f7 & 0b11 == 0b10 => FMSUB_H { frd, rm, frs1, frs2, frs3 },
        (Op::NMSUB, _, f7) if ext_zfh && f7 & 0b11 == 0b10 => {
            FNMSUB_H { frd, rm, frs1, frs2, frs3 }
        }
        (Op::NMADD, _, f7) if ext_zfh && f7 & 0b11 == 0b10 => {
            FNMADD_H { frd, rm, frs1, frs2, frs3 }
        }
        (Op::OP_FP, _, 0b0000010) if ext_zfh => FADD_H { frd, rm, frs1, frs2 },
        (Op::OP_FP, _, 0b0000110) if ext_zfh => FSUB_H { frd, rm, frs1, frs2 },
        (Op::OP_FP, _, 0b0001010) if ext_zfh => FMUL_H { frd, rm, frs1, frs2 },
        (Op::OP_FP, _, 0b0001110) if ext_zfh => FDIV_H { frd, rm, frs1, frs2 },
        (Op::OP_FP, _, 0b0101110) if ext_zfh && rs2 == 0b00000 => FSQRT_H { frd, rm, frs1 },
        (Op::OP_FP, 0b000, 0b0010010) if ext_zfh => FSGNJ_H { frd, frs1, frs2 },
        (Op::OP_FP, 0b001, 0b0010010) if ext_zfh => FSGNJN_H { frd, frs1, frs2 },
        (Op::OP_FP, 0b010, 0b0010010) if ext_zfh => FSGNJX_H { frd, frs1, frs2 },
        (Op::OP_FP, 0b000, 0b0010110) if ext_zfh => FMIN_H { frd, frs1, frs2 },
        (Op::OP_FP, 0b001, 0b0010110) if ext_zfh => FMAX_H { frd, frs1, frs2 },
        (Op::OP_FP, _, 0b0100000) if ext_zfh && rs2 == 0b000010 => FCVT_S_H { frd, rm, frs1 },
        (Op::OP_FP, _, 0b0100010) if ext_zfh && rs2 == 0b000000 => FCVT_H_S { frd, rm, frs1 },
        (Op::OP_FP, _, 0b0100001) if ext_zfh && rs2 == 0b000010 => FCVT_D_H { frd, rm, frs1 },
        (Op::OP_FP, _, 0b0100010) if ext_zfh && rs2 == 0b000001 => FCVT_H_D { frd, rm, frs1 },
        (Op::OP_FP, _, 0b0100011) if ext_zfh && rs2 == 0b000010 => FCVT_Q_H { frd, rm, frs1 },
        (Op::OP_FP, _, 0b0100010) if ext_zfh && rs2 == 0b000011 => FCVT_H_Q { frd, rm, frs1 },
        (Op::OP_FP, 0b010, 0b1010010) if ext_zfh => FEQ_H { rd, frs1, frs2 },
        (Op::OP_FP, 0b001, 0b1010010) if ext_zfh => FLT_H { rd, frs1, frs2 },
        (Op::OP_FP, 0b000, 0b1010010) if ext_zfh => FLE_H { rd, frs1, frs2 },
        (Op::OP_FP, 0b001, 0b1110010) if ext_zfh && rs2 == 0b00000 => FCLASS_H { rd, frs1 },
        (Op::OP_FP, _, 0b1100010) if ext_zfh && rs2 == 0b00000 => FCVT_W_H { rd, rm, frs1 },
        (Op::OP_FP, _, 0b1100010) if ext_zfh && rs2 == 0b00001 => FCVT_WU_H { rd, rm, frs1 },
        (Op::OP_FP, _, 0b1110010) if ext_zfh && rs2 == 0b00000 => FMV_X_H { frd, rs1 },
        (Op::OP_FP, _, 0b1101010) if ext_zfh && rs2 == 0b00000 => FCVT_H_W { frd, rm, rs1 },
        (Op::OP_FP, _, 0b1101010) if ext_zfh && rs2 == 0b00001 => FCVT_H_WU { frd, rm, rs1 },
        (Op::OP_FP, _, 0b1111010) if ext_zfh && rs2 == 0b00000 => FMV_H_X { frd, rs1 },
        // RV64Zfh extension:
        (Op::OP_FP, _, 0b1100010) if rv64 && ext_zfh && rs2 == 0b00010 => FCVT_L_H { rd, rm, frs1 },
        (Op::OP_FP, _, 0b1100010) if rv64 && ext_zfh && rs2 == 0b00011 => {
            FCVT_LU_H { rd, rm, frs1 }
        }
        (Op::OP_FP, _, 0b1101010) if rv64 && ext_zfh && rs2 == 0b00010 => FCVT_H_L { frd, rm, rs1 },
        (Op::OP_FP, _, 0b1101010) if rv64 && ext_zfh && rs2 == 0b00011 => {
            FCVT_H_LU { frd, rm, rs1 }
        }
        // Zawrs extension:
        (Op::SYSTEM, 0b000, 0b0000000)
            if ext_zawrs && rd == 0b00000 && rs1 == 0b00000 && rs2 == 0b01101 =>
        {
            WRS_NTO
        }
        (Op::SYSTEM, 0b000, 0b0000000)
            if ext_zawrs && rd == 0b00000 && rs1 == 0b00000 && rs2 == 0b11101 =>
        {
            WRS_STO
        }
        // Zba extension:
        (Op::OP, 0b010, 0b0010000) if ext_zba => SH1ADD { rd, rs1, rs2 },
        (Op::OP, 0b100, 0b0010000) if ext_zba => SH2ADD { rd, rs1, rs2 },
        (Op::OP, 0b110, 0b0010000) if ext_zba => SH3ADD { rd, rs1, rs2 },
        // RV64Zba extension:
        (Op::OP_32, 0b000, 0b0000100) if rv64 && ext_zba => ADD_UW { rd, rs1, rs2 },
        (Op::OP_32, 0b010, 0b0010000) if rv64 && ext_zba => SH1ADD_UW { rd, rs1, rs2 },
        (Op::OP_32, 0b100, 0b0010000) if rv64 && ext_zba => SH2ADD_UW { rd, rs1, rs2 },
        (Op::OP_32, 0b110, 0b0010000) if rv64 && ext_zba => SH3ADD_UW { rd, rs1, rs2 },
        (Op::OP_IMM_32, 0b001, f7) if rv64 && ext_zba && f7 >> 1 == 0b000010 => {
            SLLI_UW { rd, rs1, shamt }
        }
        // Zbb extension:
        (Op::OP_IMM, 0b001, 0b0110000) if ext_zbb && rs2 == 0b00000 => CLZ { rd, rs1 },
        (Op::OP_IMM, 0b001, 0b0110000) if ext_zbb && rs2 == 0b00001 => CTZ { rd, rs1 },
        (Op::OP_IMM, 0b001, 0b0110000) if ext_zbb && rs2 == 0b00010 => CPOP { rd, rs1 },
        (Op::OP, 0b110, 0b0000101) if ext_zbb => MAX { rd, rs1, rs2 },
        (Op::OP, 0b111, 0b0000101) if ext_zbb => MAXU { rd, rs1, rs2 },
        (Op::OP, 0b100, 0b0000101) if ext_zbb => MIN { rd, rs1, rs2 },
        (Op::OP, 0b101, 0b0000101) if ext_zbb => MINU { rd, rs1, rs2 },
        (Op::OP_IMM, 0b001, 0b0110000) if ext_zbb && rs2 == 0b00100 => SEXT_B { rd, rs1 },
        (Op::OP_IMM, 0b001, 0b0110000) if ext_zbb && rs2 == 0b00101 => SEXT_H { rd, rs1 },
        // RV32Zbb extension:
        (Op::OP, 0b100, 0b0000100) if rv32 && ext_zbb && rs2 == 0b00000 => ZEXT_H { rd, rs1 },
        // RV64Zbb extension:
        (Op::OP_IMM_32, 0b001, 0b0110000) if rv64 && ext_zbb && rs2 == 0b00000 => CLZW { rd, rs1 },
        (Op::OP_IMM_32, 0b001, 0b0110000) if rv64 && ext_zbb && rs2 == 0b00001 => CTZW { rd, rs1 },
        (Op::OP_IMM_32, 0b001, 0b0110000) if rv64 && ext_zbb && rs2 == 0b00010 => CPOPW { rd, rs1 },
        (Op::OP_32, 0b100, 0b0000100) if rv64 && ext_zbb && rs2 == 0b00000 => ZEXT_H { rd, rs1 },
        // Zbb and Zbkb extensions:
        (Op::OP, 0b111, 0b0100000) if ext_zbb || ext_zbkb => ANDN { rd, rs1, rs2 },
        (Op::OP, 0b110, 0b0100000) if ext_zbb || ext_zbkb => ORN { rd, rs1, rs2 },
        (Op::OP, 0b100, 0b0100000) if ext_zbb || ext_zbkb => XNOR { rd, rs1, rs2 },
        (Op::OP, 0b001, 0b0110000) if ext_zbb || ext_zbkb => ROL { rd, rs1, rs2 },
        (Op::OP, 0b101, 0b0110000) if ext_zbb || ext_zbkb => ROR { rd, rs1, rs2 },
        (Op::OP_IMM, 0b101, 0b0010100) if (ext_zbb || ext_zbkb) && rs2 == 0b00111 => {
            ORC_B { rd, rs1 }
        }
        // RV32Zbb and RV32Zbkb extensions:
        (Op::OP_IMM, 0b101, 0b0110000) if rv32 && (ext_zbb || ext_zbkb) => RORI { rd, rs1, shamt },
        (Op::OP_IMM, 0b101, 0b0110100) if rv32 && (ext_zbb || ext_zbkb) && rs2 == 0b11000 => {
            REV8 { rd, rs1 }
        }
        // RV64Zbb AND RV64Zbkb extensions:
        (Op::OP_IMM, 0b101, f7) if rv64 && (ext_zbb || ext_zbkb) && f7 >> 1 == 0b011000 => {
            RORI { rd, rs1, shamt }
        }
        (Op::OP_32, 0b001, 0b0110000) if rv64 && (ext_zbb || ext_zbkb) => ROLW { rd, rs1, rs2 },
        (Op::OP_IMM_32, 0b101, 0b0110000) if rv64 && (ext_zbb || ext_zbkb) => {
            RORIW { rd, rs1, shamt }
        }
        (Op::OP_32, 0b101, 0b0110000) if rv64 && (ext_zbb || ext_zbkb) => RORW { rd, rs1, rs2 },
        // Zbkb extension:
        (Op::OP, 0b100, 0b0000100) if ext_zbkb => PACK { rd, rs1, rs2 },
        (Op::OP, 0b111, 0b0000100) if ext_zbkb => PACKH { rd, rs1, rs2 },
        (Op::OP_IMM, 0b101, 0b0110100) if ext_zbkb && rs2 == 0b00111 => BREV8 { rd, rs1 },
        // RV32Zbkb extension:
        (Op::OP_IMM, 0b001, 0b0000100) if rv32 && ext_zbkb && rs2 == 0b01111 => ZIP { rd, rs1 },
        (Op::OP_IMM, 0b101, 0b0000100) if rv32 && ext_zbkb && rs2 == 0b01111 => UNZIP { rd, rs1 },
        // RV64Zbkb extension:
        (Op::OP_32, 0b100, 0b0000100) if rv64 && ext_zbkb => PACKW { rd, rs1, rs2 },
        (Op::OP_IMM, 0b101, 0b0110101) if rv64 && ext_zbkb && rs2 == 0b11000 => REV8 { rd, rs1 },
        // Zbc extension:
        (Op::OP, 0b001, 0b0000101) if ext_zbc => CLMUL { rd, rs1, rs2 },
        (Op::OP, 0b011, 0b0000101) if ext_zbc => CLMULH { rd, rs1, rs2 },
        (Op::OP, 0b010, 0b0000101) if ext_zbc => CLMULR { rd, rs1, rs2 },
        // RV32Zbs extension:
        (Op::OP, 0b001, 0b0100100) if ext_zbs => BCLR { rd, rs1, rs2 },
        (Op::OP, 0b101, 0b0100100) if ext_zbs => BEXT { rd, rs1, rs2 },
        (Op::OP, 0b001, 0b0110100) if ext_zbs => BINV { rd, rs1, rs2 },
        (Op::OP, 0b001, 0b0010100) if ext_zbs => BSET { rd, rs1, rs2 },
        // RV64Zbs extension:
        (Op::OP_IMM, 0b001, 0b0100100) if rv32 && ext_zbs => BCLRI { rd, rs1, shamt },
        (Op::OP_IMM, 0b101, 0b0100100) if rv32 && ext_zbs => BEXTI { rd, rs1, shamt },
        (Op::OP_IMM, 0b001, 0b0110100) if rv32 && ext_zbs => BINVI { rd, rs1, shamt },
        (Op::OP_IMM, 0b001, 0b0010100) if rv32 && ext_zbs => BSETI { rd, rs1, shamt },
        // RV64Zbs extension:
        (Op::OP_IMM, 0b001, f7) if rv64 && ext_zbs && f7 >> 1 == 0b010010 => {
            BCLRI { rd, rs1, shamt }
        }
        (Op::OP_IMM, 0b101, f7) if rv64 && ext_zbs && f7 >> 1 == 0b010010 => {
            BEXTI { rd, rs1, shamt }
        }
        (Op::OP_IMM, 0b001, f7) if rv64 && ext_zbs && f7 >> 1 == 0b011010 => {
            BINVI { rd, rs1, shamt }
        }
        (Op::OP_IMM, 0b001, f7) if rv64 && ext_zbs && f7 >> 1 == 0b001010 => {
            BSETI { rd, rs1, shamt }
        }
        // Unknown:
        _ => UNIMP,
    }
}

/// Instruction opcodes (the lower seven bits of an instruction).
pub struct Op;
impl Op {
    pub const LOAD: u32 = 0x03;
    pub const LOAD_FP: u32 = 0x07;
    // custom-0
    pub const MISC_MEM: u32 = 0x0f;
    pub const OP_IMM: u32 = 0x13;
    pub const AUIPC: u32 = 0x17;
    pub const OP_IMM_32: u32 = 0x1b;
    // 48b
    pub const STORE: u32 = 0x23;
    pub const STORE_FP: u32 = 0x27;
    // custom-1
    pub const AMO: u32 = 0x2f;
    pub const OP: u32 = 0x33;
    pub const LUI: u32 = 0x37;
    pub const OP_32: u32 = 0x3b;
    // 64b
    pub const MADD: u32 = 0x43;
    pub const MSUB: u32 = 0x47;
    pub const NMSUB: u32 = 0x4b;
    pub const NMADD: u32 = 0x4f;
    pub const OP_FP: u32 = 0x53;
    // reserved
    // custom-2/rv128
    // 48b
    pub const BRANCH: u32 = 0x63;
    pub const JALR: u32 = 0x67;
    // reserved
    pub const JAL: u32 = 0x6f;
    pub const SYSTEM: u32 = 0x73;
    // reserved
    // custom-3/rv128
    // >= 80b
}

/// Possible fields of an encoded instruction.
///
/// Some of these fields are overlapping / mutually exclusive and
/// are specific to an instruction's encoding.
pub struct Encoding {
    /// 7-bit opcode (see [`Op`]).
    pub opcode: u32,
    /// 3-bit opcode extension present in R-, I-, S-, and B-type instructions.
    pub funct3: u32,
    /// 7-bit opcode extension present in R-type instructions.
    pub funct7: u32,
    /// Destination register.
    pub rd: u32,
    /// Source register 1.
    pub rs1: u32,
    /// Source register 2.
    pub rs2: u32,
    /// Source register 3 for R4-type instructions.
    pub rs3: u32,
    /// 3-bit rounding mode for floating-point instructions.
    pub rm: u32,
    /// 5-bit unsigned immediate for CSR instructions.
    pub uimm: u32,
    /// 1-bit release mode for atomic instructions.
    pub rl: u32,
    /// 1-bit acquire mode for atomic instructions.
    pub aq: u32,
    /// Source / destination CSR identifier for CSR instructions.
    pub csr: u32,
    /// 5-bit "shift amount" for bit-shift instructions on 32-bit targets.
    pub shamt32: u32,
    /// 6-bit "shift amount" for bit-shift instructions on 64-bit targets.
    pub shamt64: u32,
    /// Successor for the fence instruction.
    pub succ: Iorw,
    /// Predecessor for the fence instruction.
    pub pred: Iorw,
    /// 12-bit sign-extended immediate for I-type instructions.
    pub i_imm: i32,
    /// 7-bit sign-extended immediate for S-type instructions.
    pub s_imm: i32,
    /// 13-bit sign-extended immediate for B-type instructions.
    ///
    /// Only the upper 12 bits are specified, the lowest is always zero.
    pub b_imm: i32,
    /// 20-bit immediate for U-type instructions.
    ///
    /// This value is _not_ already shifted into the upper 20 bits.
    pub u_imm: u32,
    /// 21-bit sign-extended immediate for J-type instructions.
    ///
    /// Only the upper 20 bits are specified, the lowest is always zero.
    pub j_imm: i32,
}

impl From<u32> for Encoding {
    /// Extract encoded fields from an instruction.
    fn from(code: u32) -> Self {
        Encoding {
            opcode: code & 0b1111111,
            rd: (code >> 7) & 0b11111,
            funct3: (code >> 12) & 0b111,
            rs1: (code >> 15) & 0b11111,
            rs2: (code >> 20) & 0b11111,
            rs3: (code >> 27) & 0b11111,
            rm: (code >> 12) & 0b111,
            uimm: (code >> 15) & 0b11111,
            rl: (code >> 25) & 0b1,
            aq: (code >> 26) & 0b1,
            csr: (code >> 20) & 0b1111_1111_1111,
            funct7: code >> 25,
            shamt32: (code >> 20) & 0b11111,
            shamt64: (code >> 20) & 0b111111,
            succ: Iorw::from((code >> 20) & 0b1111),
            pred: Iorw::from((code >> 24) & 0b1111),
            i_imm: code as i32 >> 20,
            s_imm: ((code as i32 >> 25) << 5) | (code as i32 >> 7) & 0b11111,
            b_imm: ((code as i32 >> 31) << 12)
                | (((code as i32 >> 25) & 0b111111) << 5)
                | (((code as i32 >> 8) & 0b1111) << 1)
                | (((code as i32 >> 7) & 0b1) << 11),
            u_imm: (code >> 12),
            j_imm: ((code as i32 >> 31) << 20)
                | (((code as i32 >> 21) & 0b11_1111_1111) << 1)
                | (((code as i32 >> 20) & 0b1) << 11)
                | (((code as i32 >> 12) & 0b1111_1111) << 12),
        }
    }
}

impl Iorw {
    pub const I: Iorw = Iorw(0b1000);
    pub const O: Iorw = Iorw(0b0100);
    pub const R: Iorw = Iorw(0b0010);
    pub const W: Iorw = Iorw(0b0001);
}

impl std::ops::BitOr for Iorw {
    type Output = Iorw;

    fn bitor(self, rhs: Self) -> Self::Output {
        Iorw(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for Iorw {
    type Output = Iorw;

    fn bitand(self, rhs: Self) -> Self::Output {
        Iorw(self.0 & rhs.0)
    }
}

impl From<u8> for Iorw {
    fn from(value: u8) -> Self {
        Iorw(value)
    }
}

impl From<u32> for Iorw {
    fn from(value: u32) -> Self {
        Iorw(value as u8)
    }
}
