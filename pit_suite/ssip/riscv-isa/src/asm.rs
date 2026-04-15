// Copyright James Wainwright
//
// SPDX-License-Identifier: MPL-2.0

//! Support for serializing instructions in assembly format.

use std::fmt::{self, Display};

use crate::instruction::{Compressed, Instruction, Iorw};

impl Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::UNIMP => f.write_str("unimp"),
            Instruction::LUI { rd, imm } => write!(f, "lui x{rd}, {imm}"),
            Instruction::AUIPC { rd, imm } => write!(f, "auipc x{rd}, {imm}"),
            Instruction::JAL { rd, offset } => write!(f, "jal x{rd}, {offset}"),
            Instruction::JALR { rd, rs1, offset } => write!(f, "jalr x{rd}, {offset}(x{rs1})"),
            Instruction::BEQ { rs1, rs2, offset } => write!(f, "beq x{rs1}, x{rs2}, {offset}"),
            Instruction::BNE { rs1, rs2, offset } => write!(f, "bne x{rs1}, x{rs2}, {offset}"),
            Instruction::BLT { rs1, rs2, offset } => write!(f, "blt x{rs1}, x{rs2}, {offset}"),
            Instruction::BGE { rs1, rs2, offset } => write!(f, "bge x{rs1}, x{rs2}, {offset}"),
            Instruction::BLTU { rs1, rs2, offset } => write!(f, "bltu x{rs1}, x{rs2}, {offset}"),
            Instruction::BGEU { rs1, rs2, offset } => write!(f, "bgeu x{rs1}, x{rs2}, {offset}"),
            Instruction::LB { rd, rs1, offset } => write!(f, "lb x{rd}, {offset}(x{rs1})"),
            Instruction::LH { rd, rs1, offset } => write!(f, "lh x{rd}, {offset}(x{rs1})"),
            Instruction::LW { rd, rs1, offset } => write!(f, "lw x{rd}, {offset}(x{rs1})"),
            Instruction::LBU { rd, rs1, offset } => write!(f, "lbu x{rd}, {offset}(x{rs1})"),
            Instruction::LHU { rd, rs1, offset } => write!(f, "lhu x{rd}, {offset}(x{rs1})"),
            Instruction::SB { rs1, rs2, offset } => write!(f, "sb x{rs2}, {offset}(x{rs1})"),
            Instruction::SH { rs1, rs2, offset } => write!(f, "sh x{rs2}, {offset}(x{rs1})"),
            Instruction::SW { rs1, rs2, offset } => write!(f, "sw x{rs2}, {offset}(x{rs1})"),
            Instruction::ADDI { rd, rs1, imm } => write!(f, "addi x{rd}, x{rs1}, {imm}"),
            Instruction::SLTI { rd, rs1, imm } => write!(f, "slti x{rd}, x{rs1}, {imm}"),
            Instruction::SLTIU { rd, rs1, imm } => write!(f, "sltiu x{rd}, x{rs1}, {imm}"),
            Instruction::XORI { rd, rs1, imm } => write!(f, "xori x{rd}, x{rs1}, {imm}"),
            Instruction::ORI { rd, rs1, imm } => write!(f, "ori x{rd}, x{rs1}, {imm}"),
            Instruction::ANDI { rd, rs1, imm } => write!(f, "andi x{rd}, x{rs1}, {imm}"),
            Instruction::SLLI { rd, rs1, shamt } => write!(f, "slli x{rd}, x{rs1}, {shamt}"),
            Instruction::SRLI { rd, rs1, shamt } => write!(f, "srli x{rd}, x{rs1}, {shamt}"),
            Instruction::SRAI { rd, rs1, shamt } => write!(f, "srai x{rd}, x{rs1}, {shamt}"),
            Instruction::ADD { rd, rs1, rs2 } => write!(f, "add x{rd}, x{rs1}, x{rs2}"),
            Instruction::SUB { rd, rs1, rs2 } => write!(f, "sub x{rd}, x{rs1}, x{rs2}"),
            Instruction::SLL { rd, rs1, rs2 } => write!(f, "sll x{rd}, x{rs1}, x{rs2}"),
            Instruction::SLT { rd, rs1, rs2 } => write!(f, "slt x{rd}, x{rs1}, x{rs2}"),
            Instruction::SLTU { rd, rs1, rs2 } => write!(f, "sltu x{rd}, x{rs1}, x{rs2}"),
            Instruction::XOR { rd, rs1, rs2 } => write!(f, "xor x{rd}, x{rs1}, x{rs2}"),
            Instruction::SRL { rd, rs1, rs2 } => write!(f, "srl x{rd}, x{rs1}, x{rs2}"),
            Instruction::SRA { rd, rs1, rs2 } => write!(f, "sra x{rd}, x{rs1}, x{rs2}"),
            Instruction::OR { rd, rs1, rs2 } => write!(f, "or x{rd}, x{rs1}, x{rs2}"),
            Instruction::AND { rd, rs1, rs2 } => write!(f, "and x{rd}, x{rs1}, x{rs2}"),
            Instruction::FENCE { pred, succ } => {
                write!(f, "fence {pred}, {succ}")
            }
            Instruction::ECALL => f.write_str("ecall"),
            Instruction::EBREAK => f.write_str("ebreak"),
            Instruction::SRET => f.write_str("sret"),
            Instruction::SFENCE_VMA { rs1, rs2 } => write!(f, "sfence.vma x{rs1}, x{rs2}"),
            Instruction::CBO => write!(f, "cbo"),
            Instruction::MRET => f.write_str("mret"),
            Instruction::WFI => f.write_str("wfi"),
            Instruction::LWU { rd, rs1, offset } => write!(f, "lwu x{rd}, {offset}(x{rs1})"),
            Instruction::LD { rd, rs1, offset } => write!(f, "ld x{rd}, {offset}(x{rs1})"),
            Instruction::SD { rs1, rs2, offset } => write!(f, "sd x{rs2}, {offset}(x{rs1})"),
            Instruction::ADDIW { rd, rs1, imm } => write!(f, "addiw x{rd}, x{rs1}, {imm}"),
            Instruction::SLLIW { rd, rs1, shamt } => write!(f, "slliw x{rd}, x{rs1}, {shamt}"),
            Instruction::SRLIW { rd, rs1, shamt } => write!(f, "srliw x{rd}, x{rs1}, {shamt}"),
            Instruction::SRAIW { rd, rs1, shamt } => write!(f, "sraiw x{rd}, x{rs1}, {shamt}"),
            Instruction::ADDW { rd, rs1, rs2 } => write!(f, "addw x{rd}, x{rs1}, x{rs2}"),
            Instruction::SUBW { rd, rs1, rs2 } => write!(f, "subw x{rd}, x{rs1}, x{rs2}"),
            Instruction::SLLW { rd, rs1, rs2 } => write!(f, "sllw x{rd}, x{rs1}, x{rs2}"),
            Instruction::SRLW { rd, rs1, rs2 } => write!(f, "srlw x{rd}, x{rs1}, x{rs2}"),
            Instruction::SRAW { rd, rs1, rs2 } => write!(f, "sraw x{rd}, x{rs1}, x{rs2}"),
            Instruction::FENCE_I => f.write_str("fence.i"),
            Instruction::CSRRW { rd, csr, rs1 } => write!(f, "csrrw x{rd}, {csr}, x{rs1}"),
            Instruction::CSRRS { rd, csr, rs1 } => write!(f, "csrrs x{rd}, {csr}, x{rs1}"),
            Instruction::CSRRC { rd, csr, rs1 } => write!(f, "csrrc x{rd}, {csr}, x{rs1}"),
            Instruction::CSRRWI { rd, csr, uimm } => write!(f, "csrrwi x{rd}, {csr}, {uimm}"),
            Instruction::CSRRSI { rd, csr, uimm } => write!(f, "csrrsi x{rd}, {csr}, {uimm}"),
            Instruction::CSRRCI { rd, csr, uimm } => write!(f, "csrrci x{rd}, {csr}, {uimm}"),
            Instruction::MUL { rd, rs1, rs2 } => write!(f, "mul x{rd}, x{rs1}, x{rs2}"),
            Instruction::MULH { rd, rs1, rs2 } => write!(f, "mulh x{rd}, x{rs1}, x{rs2}"),
            Instruction::MULHSU { rd, rs1, rs2 } => write!(f, "mulhsu x{rd}, x{rs1}, x{rs2}"),
            Instruction::MULHU { rd, rs1, rs2 } => write!(f, "mulhu x{rd}, x{rs1}, x{rs2}"),
            Instruction::DIV { rd, rs1, rs2 } => write!(f, "div x{rd}, x{rs1}, x{rs2}"),
            Instruction::DIVU { rd, rs1, rs2 } => write!(f, "divu x{rd}, x{rs1}, x{rs2}"),
            Instruction::REM { rd, rs1, rs2 } => write!(f, "rem x{rd}, x{rs1}, x{rs2}"),
            Instruction::REMU { rd, rs1, rs2 } => write!(f, "remu x{rd}, x{rs1}, x{rs2}"),
            Instruction::MULW { rd, rs1, rs2 } => write!(f, "mulw x{rd}, x{rs1}, x{rs2}"),
            Instruction::DIVW { rd, rs1, rs2 } => write!(f, "divw x{rd}, x{rs1}, x{rs2}"),
            Instruction::DIVUW { rd, rs1, rs2 } => write!(f, "divuw x{rd}, x{rs1}, x{rs2}"),
            Instruction::REMW { rd, rs1, rs2 } => write!(f, "remw x{rd}, x{rs1}, x{rs2}"),
            Instruction::REMUW { rd, rs1, rs2 } => write!(f, "remuw x{rd}, x{rs1}, x{rs2}"),
            Instruction::LR_W { rd, rs1, .. } => write!(f, "lr.w x{rd}, (x{rs1})"),
            Instruction::SC_W { rd, rs1, rs2, .. } => write!(f, "sc.w x{rd}, x{rs2}, (x{rs1})"),
            Instruction::AMOSWAP_W { rd, rs1, rs2, .. } => {
                write!(f, "amoswap.w x{rd}, x{rs2}, (x{rs1})")
            }
            Instruction::AMOADD_W { rd, rs1, rs2, .. } => {
                write!(f, "amoadd.w x{rd}, x{rs2}, (x{rs1})")
            }
            Instruction::AMOXOR_W { rd, rs1, rs2, .. } => {
                write!(f, "amoxor.w x{rd}, x{rs2}, (x{rs1})")
            }
            Instruction::AMOAND_W { rd, rs1, rs2, .. } => {
                write!(f, "amoand.w x{rd}, x{rs2}, (x{rs1})")
            }
            Instruction::AMOOR_W { rd, rs1, rs2, .. } => {
                write!(f, "amoor.w x{rd}, x{rs2}, (x{rs1})")
            }
            Instruction::AMOMIN_W { rd, rs1, rs2, .. } => {
                write!(f, "amomin.w x{rd}, x{rs2}, (x{rs1})")
            }
            Instruction::AMOMAX_W { rd, rs1, rs2, .. } => {
                write!(f, "amomax.w x{rd}, x{rs2}, (x{rs1})")
            }
            Instruction::AMOMINU_W { rd, rs1, rs2, .. } => {
                write!(f, "amominu.w x{rd}, x{rs2}, (x{rs1})")
            }
            Instruction::AMOMAXU_W { rd, rs1, rs2, .. } => {
                write!(f, "amomaxu.w x{rd}, x{rs2}, (x{rs1})")
            }
            Instruction::LR_D { rd, rs1, .. } => write!(f, "lr.d x{rd}, (x{rs1})"),
            Instruction::SC_D { rd, rs1, rs2, .. } => write!(f, "sc.d x{rd}, x{rs2}, (x{rs1})"),
            Instruction::AMOSWAP_D { rd, rs1, rs2, .. } => {
                write!(f, "amoswap.d x{rd}, x{rs2}, (x{rs1})")
            }
            Instruction::AMOADD_D { rd, rs1, rs2, .. } => {
                write!(f, "amoadd.d x{rd}, x{rs2}, (x{rs1})")
            }
            Instruction::AMOXOR_D { rd, rs1, rs2, .. } => {
                write!(f, "amoxor.d x{rd}, x{rs2}, (x{rs1})")
            }
            Instruction::AMOAND_D { rd, rs1, rs2, .. } => {
                write!(f, "amoand.d x{rd}, x{rs2}, (x{rs1})")
            }
            Instruction::AMOOR_D { rd, rs1, rs2, .. } => {
                write!(f, "amoor.d x{rd}, x{rs2}, (x{rs1})")
            }
            Instruction::AMOMIN_D { rd, rs1, rs2, .. } => {
                write!(f, "amomin.d x{rd}, x{rs2}, (x{rs1})")
            }
            Instruction::AMOMAX_D { rd, rs1, rs2, .. } => {
                write!(f, "amomax.d x{rd}, x{rs2}, (x{rs1})")
            }
            Instruction::AMOMINU_D { rd, rs1, rs2, .. } => {
                write!(f, "amominu.d x{rd}, x{rs2}, (x{rs1})")
            }
            Instruction::AMOMAXU_D { rd, rs1, rs2, .. } => {
                write!(f, "amomaxu.d x{rd}, x{rs2}, (x{rs1})")
            }
            Instruction::FLW { frd, rs1, offset } => write!(f, "flw f{frd}, {offset}(x{rs1})"),
            Instruction::FSW { rs1, frs2, offset } => write!(f, "fsw f{frs2}, {offset}(x{rs1})"),
            Instruction::FMADD_S { frd, frs1, frs2, frs3, .. } => {
                write!(f, "fmadd.s f{frd}, f{frs1}, f{frs2}, f{frs3}")
            }
            Instruction::FMSUB_S { frd, frs1, frs2, frs3, .. } => {
                write!(f, "fmsub.s f{frd}, f{frs1}, f{frs2}, f{frs3}")
            }
            Instruction::FNMSUB_S { frd, frs1, frs2, frs3, .. } => {
                write!(f, "fnmsub.s f{frd}, f{frs1}, f{frs2}, f{frs3}")
            }
            Instruction::FNMADD_S { frd, frs1, frs2, frs3, .. } => {
                write!(f, "fnmadd.s f{frd}, f{frs1}, f{frs2}, f{frs3}")
            }
            Instruction::FADD_S { frd, frs1, frs2, .. } => {
                write!(f, "fadd.s f{frd}, f{frs1}, f{frs2}")
            }
            Instruction::FSUB_S { frd, frs1, frs2, .. } => {
                write!(f, "fsub.s f{frd}, f{frs1}, f{frs2}")
            }
            Instruction::FMUL_S { frd, frs1, frs2, .. } => {
                write!(f, "fmul.s f{frd}, f{frs1}, f{frs2}")
            }
            Instruction::FDIV_S { frd, frs1, frs2, .. } => {
                write!(f, "fdiv.s f{frd}, f{frs1}, f{frs2}")
            }
            Instruction::FSQRT_S { frd, frs1, .. } => write!(f, "fsqrt.s f{frd}, f{frs1}"),
            Instruction::FSGNJ_S { frd, frs1, frs2 } => {
                write!(f, "fsgnj.s f{frd}, f{frs1}, f{frs2}")
            }
            Instruction::FSGNJN_S { frd, frs1, frs2 } => {
                write!(f, "fsgnjn.s f{frd}, f{frs1}, f{frs2}")
            }
            Instruction::FSGNJX_S { frd, frs1, frs2 } => {
                write!(f, "fsgnjx.s f{frd}, f{frs1}, f{frs2}")
            }
            Instruction::FMIN_S { frd, frs1, frs2 } => write!(f, "fmin.s f{frd}, f{frs1}, f{frs2}"),
            Instruction::FMAX_S { frd, frs1, frs2 } => write!(f, "fmax.s f{frd}, f{frs1}, f{frs2}"),
            Instruction::FCVT_W_S { rd, frs1, .. } => write!(f, "fcvt.w.s x{rd}, f{frs1}"),
            Instruction::FCVT_WU_S { rd, frs1, .. } => write!(f, "fcvt.wu.s x{rd}, f{frs1}"),
            Instruction::FMV_X_W { rd, frs1 } => write!(f, "fmv.x.w x{rd}, f{frs1}"),
            Instruction::FEQ_S { rd, frs1, frs2 } => write!(f, "feq.s x{rd}, f{frs1}, f{frs2}"),
            Instruction::FLT_S { rd, frs1, frs2 } => write!(f, "flt.s x{rd}, f{frs1}, f{frs2}"),
            Instruction::FLE_S { rd, frs1, frs2 } => write!(f, "fle.s x{rd}, f{frs1}, f{frs2}"),
            Instruction::FCLASS_S { rd, frs1 } => write!(f, "fclass.s x{rd}, f{frs1}"),
            Instruction::FCVT_S_W { frd, rs1, .. } => write!(f, "fcvt.s.w f{frd}, x{rs1}"),
            Instruction::FCVT_S_WU { frd, rs1, .. } => write!(f, "fcvt.s.wu f{frd}, x{rs1}"),
            Instruction::FMV_W_X { frd, rs1 } => write!(f, "fmv.w.x f{frd}, x{rs1}"),
            Instruction::FCVT_L_S { rd, frs1, .. } => write!(f, "fcvt.l.s x{rd}, f{frs1}"),
            Instruction::FCVT_LU_S { rd, frs1, .. } => write!(f, "fcvt.lu.s x{rd}, f{frs1}"),
            Instruction::FCVT_S_L { frd, rs1, .. } => write!(f, "fcvt.s.l f{frd}, x{rs1}"),
            Instruction::FCVT_S_LU { frd, rs1, .. } => write!(f, "fcvt.s.lu f{frd}, x{rs1}"),
            Instruction::FLD { frd, rs1, offset } => write!(f, "fld f{frd}, {offset}(x{rs1})"),
            Instruction::FSD { rs1, frs2, offset } => write!(f, "fsd f{frs2}, {offset}(x{rs1})"),
            Instruction::FMADD_D { frd, frs1, frs2, frs3, .. } => {
                write!(f, "fmadd.d f{frd}, f{frs1}, f{frs2}, f{frs3}")
            }
            Instruction::FMSUB_D { frd, frs1, frs2, frs3, .. } => {
                write!(f, "fmsub.d f{frd}, f{frs1}, f{frs2}, f{frs3}")
            }
            Instruction::FNMSUB_D { frd, frs1, frs2, frs3, .. } => {
                write!(f, "fnmsub.d f{frd}, f{frs1}, f{frs2}, f{frs3}")
            }
            Instruction::FNMADD_D { frd, frs1, frs2, frs3, .. } => {
                write!(f, "fnmadd.d f{frd}, f{frs1}, f{frs2}, f{frs3}")
            }
            Instruction::FADD_D { frd, frs1, frs2, .. } => {
                write!(f, "fadd.d f{frd}, f{frs1}, f{frs2}")
            }
            Instruction::FSUB_D { frd, frs1, frs2, .. } => {
                write!(f, "fsub.d f{frd}, f{frs1}, f{frs2}")
            }
            Instruction::FMUL_D { frd, frs1, frs2, .. } => {
                write!(f, "fmul.d f{frd}, f{frs1}, f{frs2}")
            }
            Instruction::FDIV_D { frd, frs1, frs2, .. } => {
                write!(f, "fdiv.d f{frd}, f{frs1}, f{frs2}")
            }
            Instruction::FSQRT_D { frd, frs1, .. } => write!(f, "fsqrt.d f{frd}, f{frs1}"),
            Instruction::FSGNJ_D { frd, frs1, frs2 } => {
                write!(f, "fsgnj.d f{frd}, f{frs1}, f{frs2}")
            }
            Instruction::FSGNJN_D { frd, frs1, frs2 } => {
                write!(f, "fsgnjn.d f{frd}, f{frs1}, f{frs2}")
            }
            Instruction::FSGNJX_D { frd, frs1, frs2 } => {
                write!(f, "fsgnjx.d f{frd}, f{frs1}, f{frs2}")
            }
            Instruction::FMIN_D { frd, frs1, frs2 } => write!(f, "fmin.d f{frd}, f{frs1}, f{frs2}"),
            Instruction::FMAX_D { frd, frs1, frs2 } => write!(f, "fmax.d f{frd}, f{frs1}, f{frs2}"),
            Instruction::FCVT_S_D { frd, frs1, .. } => write!(f, "fcvt.s.d f{frd}, f{frs1}"),
            Instruction::FCVT_D_S { frd, frs1, .. } => write!(f, "fcvt.d.s f{frd}, f{frs1}"),
            Instruction::FEQ_D { rd, frs1, frs2 } => write!(f, "feq.d x{rd}, f{frs1}, f{frs2}"),
            Instruction::FLT_D { rd, frs1, frs2 } => write!(f, "flt.d x{rd}, f{frs1}, f{frs2}"),
            Instruction::FLE_D { rd, frs1, frs2 } => write!(f, "fle.d x{rd}, f{frs1}, f{frs2}"),
            Instruction::FCLASS_D { rd, frs1 } => write!(f, "fclass.d x{rd}, f{frs1}"),
            Instruction::FCVT_W_D { rd, frs1, .. } => write!(f, "fcvt.w.d x{rd}, f{frs1}"),
            Instruction::FCVT_WU_D { rd, frs1, .. } => write!(f, "fcvt.wu.d x{rd}, f{frs1}"),
            Instruction::FCVT_D_W { frd, rs1, .. } => write!(f, "fcvt.d.w f{frd}, x{rs1}"),
            Instruction::FCVT_D_WU { frd, rs1, .. } => write!(f, "fcvt.d.wu f{frd}, x{rs1}"),
            Instruction::FCVT_L_D { rd, frs1, .. } => write!(f, "fcvt.l.d x{rd}, f{frs1}"),
            Instruction::FCVT_LU_D { rd, frs1, .. } => write!(f, "fcvt.lu.d x{rd}, f{frs1}"),
            Instruction::FMV_X_D { rd, frs1 } => write!(f, "fmv.x.d x{rd}, f{frs1}"),
            Instruction::FCVT_D_L { frd, rs1, .. } => write!(f, "fcvt.d.l f{frd}, x{rs1}"),
            Instruction::FCVT_D_LU { frd, rs1, .. } => write!(f, "fcvt.d.lu f{frd}, x{rs1}"),
            Instruction::FMV_D_X { frd, rs1 } => write!(f, "fmv.d.x f{frd}, x{rs1}"),
            Instruction::FLQ { frd, rs1, offset } => write!(f, "flq f{frd}, {offset}(x{rs1})"),
            Instruction::FSQ { rs1, frs2, offset } => write!(f, "fsq f{frs2}, {offset}(x{rs1})"),
            Instruction::FMADD_Q { frd, frs1, frs2, frs3, .. } => {
                write!(f, "fmadd.q f{frd}, f{frs1}, f{frs2}, f{frs3}")
            }
            Instruction::FMSUB_Q { frd, frs1, frs2, frs3, .. } => {
                write!(f, "fmsub.q f{frd}, f{frs1}, f{frs2}, f{frs3}")
            }
            Instruction::FNMSUB_Q { frd, frs1, frs2, frs3, .. } => {
                write!(f, "fnmsub.q f{frd}, f{frs1}, f{frs2}, f{frs3}")
            }
            Instruction::FNMADD_Q { frd, frs1, frs2, frs3, .. } => {
                write!(f, "fnmadd.q f{frd}, f{frs1}, f{frs2}, f{frs3}")
            }
            Instruction::FADD_Q { frd, frs1, frs2, .. } => {
                write!(f, "fadd.q f{frd}, f{frs1}, f{frs2}")
            }
            Instruction::FSUB_Q { frd, frs1, frs2, .. } => {
                write!(f, "fsub.q f{frd}, f{frs1}, f{frs2}")
            }
            Instruction::FMUL_Q { frd, frs1, frs2, .. } => {
                write!(f, "fmul.q f{frd}, f{frs1}, f{frs2}")
            }
            Instruction::FDIV_Q { frd, frs1, frs2, .. } => {
                write!(f, "fdiv.q f{frd}, f{frs1}, f{frs2}")
            }
            Instruction::FSQRT_Q { frd, frs1, .. } => write!(f, "fsqrt.q f{frd}, f{frs1}"),
            Instruction::FSGNJ_Q { frd, frs1, frs2 } => {
                write!(f, "fsgnj.q f{frd}, f{frs1}, f{frs2}")
            }
            Instruction::FSGNJN_Q { frd, frs1, frs2 } => {
                write!(f, "fsgnjn.q f{frd}, f{frs1}, f{frs2}")
            }
            Instruction::FSGNJX_Q { frd, frs1, frs2 } => {
                write!(f, "fsgnjx.q f{frd}, f{frs1}, f{frs2}")
            }
            Instruction::FMIN_Q { frd, frs1, frs2 } => write!(f, "fmin.q f{frd}, f{frs1}, f{frs2}"),
            Instruction::FMAX_Q { frd, frs1, frs2 } => write!(f, "fmax.q f{frd}, f{frs1}, f{frs2}"),
            Instruction::FCVT_S_Q { frd, frs1, .. } => write!(f, "fcvt.s.q f{frd}, f{frs1}"),
            Instruction::FCVT_Q_S { frd, frs1, .. } => write!(f, "fcvt.q.s f{frd}, f{frs1}"),
            Instruction::FCVT_D_Q { frd, frs1, .. } => write!(f, "fcvt.d.q f{frd}, f{frs1}"),
            Instruction::FCVT_Q_D { frd, frs1, .. } => write!(f, "fcvt.q.d f{frd}, f{frs1}"),
            Instruction::FEQ_Q { rd, frs1, frs2 } => write!(f, "feq.q x{rd}, f{frs1}, f{frs2}"),
            Instruction::FLT_Q { rd, frs1, frs2 } => write!(f, "flt.q x{rd}, f{frs1}, f{frs2}"),
            Instruction::FLE_Q { rd, frs1, frs2 } => write!(f, "fle.q x{rd}, f{frs1}, f{frs2}"),
            Instruction::FCLASS_Q { rd, frs1 } => write!(f, "fclass.q x{rd}, f{frs1}"),
            Instruction::FCVT_W_Q { rd, frs1, .. } => write!(f, "fcvt.w.q x{rd}, f{frs1}"),
            Instruction::FCVT_WU_Q { rd, frs1, .. } => write!(f, "fcvt.wu.q x{rd}, f{frs1}"),
            Instruction::FCVT_Q_W { frd, rs1, .. } => write!(f, "fcvt.q.w f{frd}, x{rs1}"),
            Instruction::FCVT_Q_WU { frd, rs1, .. } => write!(f, "fcvt.q.wu f{frd}, x{rs1}"),
            Instruction::FCVT_L_Q { rd, frs1, .. } => write!(f, "fcvt.l.q x{rd}, f{frs1}"),
            Instruction::FCVT_LU_Q { rd, frs1, .. } => write!(f, "fcvt.lu.q x{rd}, f{frs1}"),
            Instruction::FCVT_Q_L { frd, rs1, .. } => write!(f, "fcvt.q.l f{frd}, x{rs1}"),
            Instruction::FCVT_Q_LU { frd, rs1, .. } => write!(f, "fcvt.q.lu f{frd}, x{rs1}"),
            Instruction::FLH { frd, rs1, offset } => write!(f, "flh f{frd}, {offset}(x{rs1})"),
            Instruction::FSH { rs1, frs2, offset } => write!(f, "fsh f{frs2}, {offset}(x{rs1})"),
            Instruction::FMADD_H { frd, frs1, frs2, frs3, .. } => {
                write!(f, "fmadd.h f{frd}, f{frs1}, f{frs2}, f{frs3}")
            }
            Instruction::FMSUB_H { frd, frs1, frs2, frs3, .. } => {
                write!(f, "fmsub.h f{frd}, f{frs1}, f{frs2}, f{frs3}")
            }
            Instruction::FNMSUB_H { frd, frs1, frs2, frs3, .. } => {
                write!(f, "fnmsub.h f{frd}, f{frs1}, f{frs2}, f{frs3}")
            }
            Instruction::FNMADD_H { frd, frs1, frs2, frs3, .. } => {
                write!(f, "fnmadd.h f{frd}, f{frs1}, f{frs2}, f{frs3}")
            }
            Instruction::FADD_H { frd, frs1, frs2, .. } => {
                write!(f, "fadd.h f{frd}, f{frs1}, f{frs2}")
            }
            Instruction::FSUB_H { frd, frs1, frs2, .. } => {
                write!(f, "fsub.h f{frd}, f{frs1}, f{frs2}")
            }
            Instruction::FMUL_H { frd, frs1, frs2, .. } => {
                write!(f, "fmul.h f{frd}, f{frs1}, f{frs2}")
            }
            Instruction::FDIV_H { frd, frs1, frs2, .. } => {
                write!(f, "fdiv.h f{frd}, f{frs1}, f{frs2}")
            }
            Instruction::FSQRT_H { frd, frs1, .. } => write!(f, "fsqrt.h f{frd}, f{frs1}"),
            Instruction::FSGNJ_H { frd, frs1, frs2 } => {
                write!(f, "fsgnj.h f{frd}, f{frs1}, f{frs2}")
            }
            Instruction::FSGNJN_H { frd, frs1, frs2 } => {
                write!(f, "fsgnjn.h f{frd}, f{frs1}, f{frs2}")
            }
            Instruction::FSGNJX_H { frd, frs1, frs2 } => {
                write!(f, "fsgnjx.h f{frd}, f{frs1}, f{frs2}")
            }
            Instruction::FMIN_H { frd, frs1, frs2 } => write!(f, "fmin.h f{frd}, f{frs1}, f{frs2}"),
            Instruction::FMAX_H { frd, frs1, frs2 } => write!(f, "fmax.h f{frd}, f{frs1}, f{frs2}"),
            Instruction::FCVT_S_H { frd, frs1, .. } => write!(f, "fcvt.s.h f{frd}, f{frs1}"),
            Instruction::FCVT_H_S { frd, frs1, .. } => write!(f, "fcvt.h.s f{frd}, f{frs1}"),
            Instruction::FCVT_D_H { frd, frs1, .. } => write!(f, "fcvt.d.h f{frd}, f{frs1}"),
            Instruction::FCVT_H_D { frd, frs1, .. } => write!(f, "fcvt.h.d f{frd}, f{frs1}"),
            Instruction::FCVT_Q_H { frd, frs1, .. } => write!(f, "fcvt.q.h f{frd}, f{frs1}"),
            Instruction::FCVT_H_Q { frd, frs1, .. } => write!(f, "fcvt.h.q f{frd}, f{frs1}"),
            Instruction::FEQ_H { rd, frs1, frs2 } => write!(f, "feq.h x{rd}, f{frs1}, f{frs2}"),
            Instruction::FLT_H { rd, frs1, frs2 } => write!(f, "flt.h x{rd}, f{frs1}, f{frs2}"),
            Instruction::FLE_H { rd, frs1, frs2 } => write!(f, "fle.h x{rd}, f{frs1}, f{frs2}"),
            Instruction::FCLASS_H { rd, frs1 } => write!(f, "fclass.h x{rd}, f{frs1}"),
            Instruction::FCVT_W_H { rd, frs1, .. } => write!(f, "fcvt.w.h x{rd}, f{frs1}"),
            Instruction::FCVT_WU_H { rd, frs1, .. } => write!(f, "fcvt.wu.h x{rd}, f{frs1}"),
            Instruction::FMV_X_H { frd, rs1 } => write!(f, "fmv.x.h f{frd}, x{rs1}"),
            Instruction::FCVT_H_W { frd, rs1, .. } => write!(f, "fcvt.h.w f{frd}, x{rs1}"),
            Instruction::FCVT_H_WU { frd, rs1, .. } => write!(f, "fcvt.h.wu f{frd}, x{rs1}"),
            Instruction::FMV_H_X { frd, rs1 } => write!(f, "fmv.h.x f{frd}, x{rs1}"),
            Instruction::FCVT_L_H { rd, frs1, .. } => write!(f, "fcvt.l.h x{rd}, f{frs1}"),
            Instruction::FCVT_LU_H { rd, frs1, .. } => {
                write!(f, "fcvt.lu.h x{rd}, f{frs1}")
            }
            Instruction::FCVT_H_L { frd, rs1, .. } => write!(f, "fcvt.h.l f{frd}, x{rs1}"),
            Instruction::FCVT_H_LU { frd, rs1, .. } => {
                write!(f, "fcvt.h.lu f{frd}, x{rs1}")
            }
            Instruction::WRS_NTO => f.write_str("wrs.nto"),
            Instruction::WRS_STO => f.write_str("wrs.sto"),
            Instruction::SH1ADD { rd, rs1, rs2 } => write!(f, "sh1add x{rd}, x{rs1}, x{rs2}"),
            Instruction::SH2ADD { rd, rs1, rs2 } => write!(f, "sh2add x{rd}, x{rs1}, x{rs2}"),
            Instruction::SH3ADD { rd, rs1, rs2 } => write!(f, "sh3add x{rd}, x{rs1}, x{rs2}"),
            Instruction::ADD_UW { rd, rs1, rs2 } => write!(f, "add.uw x{rd}, x{rs1}, x{rs2}"),
            Instruction::SH1ADD_UW { rd, rs1, rs2 } => write!(f, "sh1add.uw x{rd}, x{rs1}, x{rs2}"),
            Instruction::SH2ADD_UW { rd, rs1, rs2 } => write!(f, "sh2add.uw x{rd}, x{rs1}, x{rs2}"),
            Instruction::SH3ADD_UW { rd, rs1, rs2 } => write!(f, "sh3add.uw x{rd}, x{rs1}, x{rs2}"),
            Instruction::SLLI_UW { rd, rs1, shamt } => write!(f, "slli.uw x{rd}, x{rs1}, {shamt}"),
            Instruction::ANDN { rd, rs1, rs2 } => write!(f, "andn x{rd}, x{rs1}, x{rs2}"),
            Instruction::ORN { rd, rs1, rs2 } => write!(f, "orn x{rd}, x{rs1}, x{rs2}"),
            Instruction::XNOR { rd, rs1, rs2 } => write!(f, "xnor x{rd}, x{rs1}, x{rs2}"),
            Instruction::CLZ { rd, rs1 } => write!(f, "clz x{rd}, x{rs1}"),
            Instruction::CTZ { rd, rs1 } => write!(f, "ctz x{rd}, x{rs1}"),
            Instruction::CPOP { rd, rs1 } => write!(f, "cpop x{rd}, x{rs1}"),
            Instruction::MAX { rd, rs1, rs2 } => write!(f, "max x{rd}, x{rs1}, x{rs2}"),
            Instruction::MAXU { rd, rs1, rs2 } => write!(f, "maxu x{rd}, x{rs1}, x{rs2}"),
            Instruction::MIN { rd, rs1, rs2 } => write!(f, "min x{rd}, x{rs1}, x{rs2}"),
            Instruction::MINU { rd, rs1, rs2 } => write!(f, "minu x{rd}, x{rs1}, x{rs2}"),
            Instruction::SEXT_B { rd, rs1 } => write!(f, "sext.b x{rd}, x{rs1}"),
            Instruction::SEXT_H { rd, rs1 } => write!(f, "sext.h x{rd}, x{rs1}"),
            Instruction::ZEXT_H { rd, rs1 } => write!(f, "zext.h x{rd}, x{rs1}"),
            Instruction::CLZW { rd, rs1 } => write!(f, "clzw x{rd}, x{rs1}"),
            Instruction::CTZW { rd, rs1 } => write!(f, "ctzw x{rd}, x{rs1}"),
            Instruction::CPOPW { rd, rs1 } => write!(f, "cpopw x{rd}, x{rs1}"),
            Instruction::ROL { rd, rs1, rs2 } => write!(f, "rol x{rd}, x{rs1}, x{rs2}"),
            Instruction::ROR { rd, rs1, rs2 } => write!(f, "ror x{rd}, x{rs1}, x{rs2}"),
            Instruction::RORI { rd, rs1, shamt } => write!(f, "rori x{rd}, x{rs1}, {shamt}"),
            Instruction::ORC_B { rd, rs1 } => write!(f, "orc.b x{rd}, x{rs1}"),
            Instruction::REV8 { rd, rs1 } => write!(f, "rev8 x{rd}, x{rs1}"),
            Instruction::ROLW { rd, rs1, rs2 } => write!(f, "rolw x{rd}, x{rs1}, x{rs2}"),
            Instruction::RORIW { rd, rs1, shamt } => write!(f, "roriw x{rd}, x{rs1}, {shamt}"),
            Instruction::RORW { rd, rs1, rs2 } => write!(f, "rorw x{rd}, x{rs1}, x{rs2}"),
            Instruction::PACK { rd, rs1, rs2 } => write!(f, "pack x{rd}, x{rs1}, x{rs2}"),
            Instruction::PACKH { rd, rs1, rs2 } => write!(f, "packh x{rd}, x{rs1}, x{rs2}"),
            Instruction::BREV8 { rd, rs1 } => write!(f, "brev8 x{rd}, x{rs1}"),
            Instruction::ZIP { rd, rs1 } => write!(f, "zip x{rd}, x{rs1}"),
            Instruction::UNZIP { rd, rs1 } => write!(f, "unzip x{rd}, x{rs1}"),
            Instruction::PACKW { rd, rs1, rs2 } => write!(f, "packw x{rd}, x{rs1}, x{rs2}"),
            Instruction::CLMUL { rd, rs1, rs2 } => write!(f, "clmul x{rd}, x{rs1}, x{rs2}"),
            Instruction::CLMULH { rd, rs1, rs2 } => write!(f, "clmulh x{rd}, x{rs1}, x{rs2}"),
            Instruction::CLMULR { rd, rs1, rs2 } => write!(f, "clmulr x{rd}, x{rs1}, x{rs2}"),
            Instruction::BCLR { rd, rs1, rs2 } => write!(f, "bclr x{rd}, x{rs1}, x{rs2}"),
            Instruction::BCLRI { rd, rs1, shamt } => write!(f, "bclri x{rd}, x{rs1}, {shamt}"),
            Instruction::BEXT { rd, rs1, rs2 } => write!(f, "bext x{rd}, x{rs1}, x{rs2}"),
            Instruction::BEXTI { rd, rs1, shamt } => write!(f, "bexti x{rd}, x{rs1}, {shamt}"),
            Instruction::BINV { rd, rs1, rs2 } => write!(f, "binv x{rd}, x{rs1}, x{rs2}"),
            Instruction::BINVI { rd, rs1, shamt } => write!(f, "binvi x{rd}, x{rs1}, {shamt}"),
            Instruction::BSET { rd, rs1, rs2 } => write!(f, "bset x{rd}, x{rs1}, x{rs2}"),
            Instruction::BSETI { rd, rs1, shamt } => write!(f, "bseti x{rd}, x{rs1}, {shamt}"),
        }
    }
}

impl Display for Compressed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Compressed::UNIMP => f.write_str("unimp"),
            Compressed::C_LWSP { rd, offset } => write!(f, "c.lwsp x{rd}, {offset}(x2)"),
            Compressed::C_LDSP { rd, offset } => write!(f, "c.ldsp x{rd}, {offset}(x2)"),
            Compressed::C_LQSP { rd, offset } => write!(f, "c.lqsp x{rd}, {offset}(x2)"),
            Compressed::C_FLWSP { frd, offset } => write!(f, "c.flwsp f{frd}, {offset}(x2)"),
            Compressed::C_FLDSP { frd, offset } => write!(f, "c.fldsp f{frd}, {offset}(x2)"),
            Compressed::C_SWSP { rs2, offset } => write!(f, "c.swsp x{rs2}, {offset}(x2)"),
            Compressed::C_SDSP { rs2, offset } => write!(f, "c.sdsp x{rs2}, {offset}(x2)"),
            Compressed::C_SQSP { rs2, offset } => write!(f, "c.sqsp x{rs2}, {offset}(x2)"),
            Compressed::C_FSWSP { frs2, offset } => write!(f, "c.fswsp f{frs2}, {offset}(x2)"),
            Compressed::C_FSDSP { frs2, offset } => write!(f, "c.fsdsp f{frs2}, {offset}(x2)"),
            Compressed::C_LW { rd, rs1, offset } => write!(f, "c.lw x{rd}, {offset}(x{rs1})"),
            Compressed::C_LD { rd, rs1, offset } => write!(f, "c.ld x{rd}, {offset}(x{rs1})"),
            Compressed::C_LQ { rd, rs1, offset } => write!(f, "c.lq x{rd}, {offset}(x{rs1})"),
            Compressed::C_FLW { frd, rs1, offset } => write!(f, "c.flw f{frd}, {offset}(x{rs1})"),
            Compressed::C_FLD { frd, rs1, offset } => write!(f, "c.fld f{frd}, {offset}(x{rs1})"),
            Compressed::C_SW { rs1, rs2, offset } => write!(f, "c.sw x{rs2}, {offset}(x{rs1})"),
            Compressed::C_SD { rs1, rs2, offset } => write!(f, "c.sd x{rs2}, {offset}(x{rs1})"),
            Compressed::C_SQ { rs1, rs2, offset } => write!(f, "c.sq x{rs2}, {offset}(x{rs1})"),
            Compressed::C_FSW { rs1, frs2, offset } => write!(f, "c.fsw f{frs2}, {offset}(x{rs1})"),
            Compressed::C_FSD { rs1, frs2, offset } => write!(f, "c.fsd f{frs2}, {offset}(x{rs1})"),
            Compressed::C_J { offset } => write!(f, "c.j {offset}"),
            Compressed::C_JAL { offset } => write!(f, "c.jal {offset}"),
            Compressed::C_JR { rs1 } => write!(f, "c.jr x{rs1}"),
            Compressed::C_JALR { rs1 } => write!(f, "c.jalr x{rs1}"),
            Compressed::C_BEQZ { rs1, offset } => write!(f, "c.beqz x{rs1}, {offset}"),
            Compressed::C_BNEZ { rs1, offset } => write!(f, "c.bnez x{rs1}, {offset}"),
            Compressed::C_LI { rd, imm } => write!(f, "c.li x{rd}, {imm}"),
            Compressed::C_LUI { rd, imm } => write!(f, "c.lui x{rd}, {imm}"),
            Compressed::C_ADDI { rd, imm } => write!(f, "c.addi x{rd}, {imm}"),
            Compressed::C_ADDIW { rd, imm } => write!(f, "c.addiw x{rd}, {imm}"),
            Compressed::C_ADDI16SP { imm } => write!(f, "c.addi16sp x2, {imm}"),
            Compressed::C_ADDI4SPN { rd, imm } => write!(f, "c.addi4spn x{rd}, x2, {imm}"),
            Compressed::C_SLLI { rd, shamt } => write!(f, "c.slli x{rd}, {shamt}"),
            Compressed::C_SRLI { rd, shamt } => write!(f, "c.srli x{rd}, {shamt}"),
            Compressed::C_SRAI { rd, shamt } => write!(f, "c.srai x{rd}, {shamt}"),
            Compressed::C_ANDI { rd, imm } => write!(f, "c.andi x{rd}, {imm}"),
            Compressed::C_MV { rd, rs2 } => write!(f, "c.mv x{rd}, x{rs2}"),
            Compressed::C_ADD { rd, rs2 } => write!(f, "c.add x{rd}, x{rs2}"),
            Compressed::C_AND { rd, rs2 } => write!(f, "c.and x{rd}, x{rs2}"),
            Compressed::C_OR { rd, rs2 } => write!(f, "c.or x{rd}, x{rs2}"),
            Compressed::C_XOR { rd, rs2 } => write!(f, "c.xor x{rd}, x{rs2}"),
            Compressed::C_SUB { rd, rs2 } => write!(f, "c.sub x{rd}, x{rs2}"),
            Compressed::C_ADDW { rd, rs2 } => write!(f, "c.addw x{rd}, x{rs2}"),
            Compressed::C_SUBW { rd, rs2 } => write!(f, "c.subw x{rd}, x{rs2}"),
            Compressed::C_NOP => f.write_str("c.nop"),
            Compressed::C_EBREAK => f.write_str("c.ebreak"),
        }
    }
}

impl Display for Iorw {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if *self & Iorw::I == Iorw::I {
            f.write_str("i")?;
        }
        if *self & Iorw::O == Iorw::O {
            f.write_str("o")?;
        }
        if *self & Iorw::R == Iorw::R {
            f.write_str("r")?;
        }
        if *self & Iorw::W == Iorw::W {
            f.write_str("w")?;
        }

        Ok(())
    }
}
