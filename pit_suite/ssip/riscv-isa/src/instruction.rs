// Copyright James Wainwright
//
// SPDX-License-Identifier: MPL-2.0

/// RISC-V instruction.
///
/// Contains all supported canonical RISC-V instructions. Does not
/// contain pseudo or compressed instructions.
///
/// Instruction arguments are all in their decoded forms, meaning correctly
/// scaled and sign extended. Their names are as they appear in specifications
/// (for the most part), and include:
///
/// * `rd`, `rs1`, `rs2`: destination and source registers.
/// * `frd`, `frs1`, `frs2`, `frs3`: destination and source floating point registers.
/// * `offset`, `imm`, `shamt`: numerical offsets, immediates, and shift amounts.
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Instruction {
    // Unknown:
    UNIMP,
    // RV32I base instruction set:
    LUI { rd: u32, imm: u32 },
    AUIPC { rd: u32, imm: u32 },
    JAL { rd: u32, offset: i32 },
    JALR { rd: u32, rs1: u32, offset: i32 },
    BEQ { rs1: u32, rs2: u32, offset: i32 },
    BNE { rs1: u32, rs2: u32, offset: i32 },
    BLT { rs1: u32, rs2: u32, offset: i32 },
    BGE { rs1: u32, rs2: u32, offset: i32 },
    BLTU { rs1: u32, rs2: u32, offset: i32 },
    BGEU { rs1: u32, rs2: u32, offset: i32 },
    LB { rd: u32, rs1: u32, offset: i32 },
    LH { rd: u32, rs1: u32, offset: i32 },
    LW { rd: u32, rs1: u32, offset: i32 },
    LBU { rd: u32, rs1: u32, offset: i32 },
    LHU { rd: u32, rs1: u32, offset: i32 },
    SB { rs1: u32, rs2: u32, offset: i32 },
    SH { rs1: u32, rs2: u32, offset: i32 },
    SW { rs1: u32, rs2: u32, offset: i32 },
    ADDI { rd: u32, rs1: u32, imm: i32 },
    SLTI { rd: u32, rs1: u32, imm: i32 },
    SLTIU { rd: u32, rs1: u32, imm: i32 },
    XORI { rd: u32, rs1: u32, imm: i32 },
    ORI { rd: u32, rs1: u32, imm: i32 },
    ANDI { rd: u32, rs1: u32, imm: i32 },
    SLLI { rd: u32, rs1: u32, shamt: u32 },
    SRLI { rd: u32, rs1: u32, shamt: u32 },
    SRAI { rd: u32, rs1: u32, shamt: u32 },
    ADD { rd: u32, rs1: u32, rs2: u32 },
    SUB { rd: u32, rs1: u32, rs2: u32 },
    SLL { rd: u32, rs1: u32, rs2: u32 },
    SLT { rd: u32, rs1: u32, rs2: u32 },
    SLTU { rd: u32, rs1: u32, rs2: u32 },
    XOR { rd: u32, rs1: u32, rs2: u32 },
    SRL { rd: u32, rs1: u32, rs2: u32 },
    SRA { rd: u32, rs1: u32, rs2: u32 },
    OR { rd: u32, rs1: u32, rs2: u32 },
    AND { rd: u32, rs1: u32, rs2: u32 },
    FENCE { pred: Iorw, succ: Iorw },
    CBO,
    ECALL,
    EBREAK,
    // S-mode:
    SRET,
    SFENCE_VMA { rs1: u32, rs2: u32 },
    // Privileged:
    MRET,
    WFI,
    // RV64I base instruction set:
    LWU { rd: u32, rs1: u32, offset: i32 },
    LD { rd: u32, rs1: u32, offset: i32 },
    SD { rs1: u32, rs2: u32, offset: i32 },
    ADDIW { rd: u32, rs1: u32, imm: i32 },
    SLLIW { rd: u32, rs1: u32, shamt: u32 },
    SRLIW { rd: u32, rs1: u32, shamt: u32 },
    SRAIW { rd: u32, rs1: u32, shamt: u32 },
    ADDW { rd: u32, rs1: u32, rs2: u32 },
    SUBW { rd: u32, rs1: u32, rs2: u32 },
    SLLW { rd: u32, rs1: u32, rs2: u32 },
    SRLW { rd: u32, rs1: u32, rs2: u32 },
    SRAW { rd: u32, rs1: u32, rs2: u32 },
    // RV32/RV64 Zifencei:
    FENCE_I,
    // RV32/RV64 Zicsr extension:
    CSRRW { rd: u32, csr: u32, rs1: u32 },
    CSRRS { rd: u32, csr: u32, rs1: u32 },
    CSRRC { rd: u32, csr: u32, rs1: u32 },
    CSRRWI { rd: u32, csr: u32, uimm: u32 },
    CSRRSI { rd: u32, csr: u32, uimm: u32 },
    CSRRCI { rd: u32, csr: u32, uimm: u32 },
    // RV32M extension:
    MUL { rd: u32, rs1: u32, rs2: u32 },
    MULH { rd: u32, rs1: u32, rs2: u32 },
    MULHSU { rd: u32, rs1: u32, rs2: u32 },
    MULHU { rd: u32, rs1: u32, rs2: u32 },
    DIV { rd: u32, rs1: u32, rs2: u32 },
    DIVU { rd: u32, rs1: u32, rs2: u32 },
    REM { rd: u32, rs1: u32, rs2: u32 },
    REMU { rd: u32, rs1: u32, rs2: u32 },
    // RV64M extension:
    MULW { rd: u32, rs1: u32, rs2: u32 },
    DIVW { rd: u32, rs1: u32, rs2: u32 },
    DIVUW { rd: u32, rs1: u32, rs2: u32 },
    REMW { rd: u32, rs1: u32, rs2: u32 },
    REMUW { rd: u32, rs1: u32, rs2: u32 },
    // RV32A extension:
    LR_W { rd: u32, rs1: u32, rl: u32, aq: u32 },
    SC_W { rd: u32, rs1: u32, rs2: u32, rl: u32, aq: u32 },
    AMOSWAP_W { rd: u32, rs1: u32, rs2: u32, rl: u32, aq: u32 },
    AMOADD_W { rd: u32, rs1: u32, rs2: u32, rl: u32, aq: u32 },
    AMOXOR_W { rd: u32, rs1: u32, rs2: u32, rl: u32, aq: u32 },
    AMOAND_W { rd: u32, rs1: u32, rs2: u32, rl: u32, aq: u32 },
    AMOOR_W { rd: u32, rs1: u32, rs2: u32, rl: u32, aq: u32 },
    AMOMIN_W { rd: u32, rs1: u32, rs2: u32, rl: u32, aq: u32 },
    AMOMAX_W { rd: u32, rs1: u32, rs2: u32, rl: u32, aq: u32 },
    AMOMINU_W { rd: u32, rs1: u32, rs2: u32, rl: u32, aq: u32 },
    AMOMAXU_W { rd: u32, rs1: u32, rs2: u32, rl: u32, aq: u32 },
    // RV64A extension:
    LR_D { rd: u32, rs1: u32, rl: u32, aq: u32 },
    SC_D { rd: u32, rs1: u32, rs2: u32, rl: u32, aq: u32 },
    AMOSWAP_D { rd: u32, rs1: u32, rs2: u32, rl: u32, aq: u32 },
    AMOADD_D { rd: u32, rs1: u32, rs2: u32, rl: u32, aq: u32 },
    AMOXOR_D { rd: u32, rs1: u32, rs2: u32, rl: u32, aq: u32 },
    AMOAND_D { rd: u32, rs1: u32, rs2: u32, rl: u32, aq: u32 },
    AMOOR_D { rd: u32, rs1: u32, rs2: u32, rl: u32, aq: u32 },
    AMOMIN_D { rd: u32, rs1: u32, rs2: u32, rl: u32, aq: u32 },
    AMOMAX_D { rd: u32, rs1: u32, rs2: u32, rl: u32, aq: u32 },
    AMOMINU_D { rd: u32, rs1: u32, rs2: u32, rl: u32, aq: u32 },
    AMOMAXU_D { rd: u32, rs1: u32, rs2: u32, rl: u32, aq: u32 },

    // RV32F extension:
    FLW { frd: u32, rs1: u32, offset: i32 },
    FSW { rs1: u32, frs2: u32, offset: i32 },
    FMADD_S { frd: u32, rm: u32, frs1: u32, frs2: u32, frs3: u32 },
    FMSUB_S { frd: u32, rm: u32, frs1: u32, frs2: u32, frs3: u32 },
    FNMSUB_S { frd: u32, rm: u32, frs1: u32, frs2: u32, frs3: u32 },
    FNMADD_S { frd: u32, rm: u32, frs1: u32, frs2: u32, frs3: u32 },
    FADD_S { frd: u32, rm: u32, frs1: u32, frs2: u32 },
    FSUB_S { frd: u32, rm: u32, frs1: u32, frs2: u32 },
    FMUL_S { frd: u32, rm: u32, frs1: u32, frs2: u32 },
    FDIV_S { frd: u32, rm: u32, frs1: u32, frs2: u32 },
    FSQRT_S { frd: u32, rm: u32, frs1: u32 },
    FSGNJ_S { frd: u32, frs1: u32, frs2: u32 },
    FSGNJN_S { frd: u32, frs1: u32, frs2: u32 },
    FSGNJX_S { frd: u32, frs1: u32, frs2: u32 },
    FMIN_S { frd: u32, frs1: u32, frs2: u32 },
    FMAX_S { frd: u32, frs1: u32, frs2: u32 },
    FCVT_W_S { rd: u32, rm: u32, frs1: u32 },
    FCVT_WU_S { rd: u32, rm: u32, frs1: u32 },
    FMV_X_W { rd: u32, frs1: u32 },
    FEQ_S { rd: u32, frs1: u32, frs2: u32 },
    FLT_S { rd: u32, frs1: u32, frs2: u32 },
    FLE_S { rd: u32, frs1: u32, frs2: u32 },
    FCLASS_S { rd: u32, frs1: u32 },
    FCVT_S_W { frd: u32, rm: u32, rs1: u32 },
    FCVT_S_WU { frd: u32, rm: u32, rs1: u32 },
    FMV_W_X { frd: u32, rs1: u32 },
    // RV64F extension:
    FCVT_L_S { rd: u32, rm: u32, frs1: u32 },
    FCVT_LU_S { rd: u32, rm: u32, frs1: u32 },
    FCVT_S_L { frd: u32, rm: u32, rs1: u32 },
    FCVT_S_LU { frd: u32, rm: u32, rs1: u32 },
    // RV32D extension:
    FLD { frd: u32, rs1: u32, offset: i32 },
    FSD { rs1: u32, frs2: u32, offset: i32 },
    FMADD_D { frd: u32, rm: u32, frs1: u32, frs2: u32, frs3: u32 },
    FMSUB_D { frd: u32, rm: u32, frs1: u32, frs2: u32, frs3: u32 },
    FNMSUB_D { frd: u32, rm: u32, frs1: u32, frs2: u32, frs3: u32 },
    FNMADD_D { frd: u32, rm: u32, frs1: u32, frs2: u32, frs3: u32 },
    FADD_D { frd: u32, rm: u32, frs1: u32, frs2: u32 },
    FSUB_D { frd: u32, rm: u32, frs1: u32, frs2: u32 },
    FMUL_D { frd: u32, rm: u32, frs1: u32, frs2: u32 },
    FDIV_D { frd: u32, rm: u32, frs1: u32, frs2: u32 },
    FSQRT_D { frd: u32, rm: u32, frs1: u32 },
    FSGNJ_D { frd: u32, frs1: u32, frs2: u32 },
    FSGNJN_D { frd: u32, frs1: u32, frs2: u32 },
    FSGNJX_D { frd: u32, frs1: u32, frs2: u32 },
    FMIN_D { frd: u32, frs1: u32, frs2: u32 },
    FMAX_D { frd: u32, frs1: u32, frs2: u32 },
    FCVT_S_D { frd: u32, rm: u32, frs1: u32 },
    FCVT_D_S { frd: u32, rm: u32, frs1: u32 },
    FEQ_D { rd: u32, frs1: u32, frs2: u32 },
    FLT_D { rd: u32, frs1: u32, frs2: u32 },
    FLE_D { rd: u32, frs1: u32, frs2: u32 },
    FCLASS_D { rd: u32, frs1: u32 },
    FCVT_W_D { rd: u32, rm: u32, frs1: u32 },
    FCVT_WU_D { rd: u32, rm: u32, frs1: u32 },
    FCVT_D_W { frd: u32, rm: u32, rs1: u32 },
    FCVT_D_WU { frd: u32, rm: u32, rs1: u32 },
    // RV64D extension:
    FCVT_L_D { rd: u32, rm: u32, frs1: u32 },
    FCVT_LU_D { rd: u32, rm: u32, frs1: u32 },
    FMV_X_D { rd: u32, frs1: u32 },
    FCVT_D_L { frd: u32, rm: u32, rs1: u32 },
    FCVT_D_LU { frd: u32, rm: u32, rs1: u32 },
    FMV_D_X { frd: u32, rs1: u32 },
    // RV32Q extension:
    FLQ { frd: u32, rs1: u32, offset: i32 },
    FSQ { rs1: u32, frs2: u32, offset: i32 },
    FMADD_Q { frd: u32, rm: u32, frs1: u32, frs2: u32, frs3: u32 },
    FMSUB_Q { frd: u32, rm: u32, frs1: u32, frs2: u32, frs3: u32 },
    FNMSUB_Q { frd: u32, rm: u32, frs1: u32, frs2: u32, frs3: u32 },
    FNMADD_Q { frd: u32, rm: u32, frs1: u32, frs2: u32, frs3: u32 },
    FADD_Q { frd: u32, rm: u32, frs1: u32, frs2: u32 },
    FSUB_Q { frd: u32, rm: u32, frs1: u32, frs2: u32 },
    FMUL_Q { frd: u32, rm: u32, frs1: u32, frs2: u32 },
    FDIV_Q { frd: u32, rm: u32, frs1: u32, frs2: u32 },
    FSQRT_Q { frd: u32, rm: u32, frs1: u32 },
    FSGNJ_Q { frd: u32, frs1: u32, frs2: u32 },
    FSGNJN_Q { frd: u32, frs1: u32, frs2: u32 },
    FSGNJX_Q { frd: u32, frs1: u32, frs2: u32 },
    FMIN_Q { frd: u32, frs1: u32, frs2: u32 },
    FMAX_Q { frd: u32, frs1: u32, frs2: u32 },
    FCVT_S_Q { frd: u32, rm: u32, frs1: u32 },
    FCVT_Q_S { frd: u32, rm: u32, frs1: u32 },
    FCVT_D_Q { frd: u32, rm: u32, frs1: u32 },
    FCVT_Q_D { frd: u32, rm: u32, frs1: u32 },
    FEQ_Q { rd: u32, frs1: u32, frs2: u32 },
    FLT_Q { rd: u32, frs1: u32, frs2: u32 },
    FLE_Q { rd: u32, frs1: u32, frs2: u32 },
    FCLASS_Q { rd: u32, frs1: u32 },
    FCVT_W_Q { rd: u32, rm: u32, frs1: u32 },
    FCVT_WU_Q { rd: u32, rm: u32, frs1: u32 },
    FCVT_Q_W { frd: u32, rm: u32, rs1: u32 },
    FCVT_Q_WU { frd: u32, rm: u32, rs1: u32 },
    // RV64Q extension:
    FCVT_L_Q { rd: u32, rm: u32, frs1: u32 },
    FCVT_LU_Q { rd: u32, rm: u32, frs1: u32 },
    FCVT_Q_L { frd: u32, rm: u32, rs1: u32 },
    FCVT_Q_LU { frd: u32, rm: u32, rs1: u32 },
    // RV32Zfh extension:
    FLH { frd: u32, rs1: u32, offset: i32 },
    FSH { rs1: u32, frs2: u32, offset: i32 },
    FMADD_H { frd: u32, rm: u32, frs1: u32, frs2: u32, frs3: u32 },
    FMSUB_H { frd: u32, rm: u32, frs1: u32, frs2: u32, frs3: u32 },
    FNMSUB_H { frd: u32, rm: u32, frs1: u32, frs2: u32, frs3: u32 },
    FNMADD_H { frd: u32, rm: u32, frs1: u32, frs2: u32, frs3: u32 },
    FADD_H { frd: u32, rm: u32, frs1: u32, frs2: u32 },
    FSUB_H { frd: u32, rm: u32, frs1: u32, frs2: u32 },
    FMUL_H { frd: u32, rm: u32, frs1: u32, frs2: u32 },
    FDIV_H { frd: u32, rm: u32, frs1: u32, frs2: u32 },
    FSQRT_H { frd: u32, rm: u32, frs1: u32 },
    FSGNJ_H { frd: u32, frs1: u32, frs2: u32 },
    FSGNJN_H { frd: u32, frs1: u32, frs2: u32 },
    FSGNJX_H { frd: u32, frs1: u32, frs2: u32 },
    FMIN_H { frd: u32, frs1: u32, frs2: u32 },
    FMAX_H { frd: u32, frs1: u32, frs2: u32 },
    FCVT_S_H { frd: u32, rm: u32, frs1: u32 },
    FCVT_H_S { frd: u32, rm: u32, frs1: u32 },
    FCVT_D_H { frd: u32, rm: u32, frs1: u32 },
    FCVT_H_D { frd: u32, rm: u32, frs1: u32 },
    FCVT_Q_H { frd: u32, rm: u32, frs1: u32 },
    FCVT_H_Q { frd: u32, rm: u32, frs1: u32 },
    FEQ_H { rd: u32, frs1: u32, frs2: u32 },
    FLT_H { rd: u32, frs1: u32, frs2: u32 },
    FLE_H { rd: u32, frs1: u32, frs2: u32 },
    FCLASS_H { rd: u32, frs1: u32 },
    FCVT_W_H { rd: u32, rm: u32, frs1: u32 },
    FCVT_WU_H { rd: u32, rm: u32, frs1: u32 },
    FMV_X_H { frd: u32, rs1: u32 },
    FCVT_H_W { frd: u32, rm: u32, rs1: u32 },
    FCVT_H_WU { frd: u32, rm: u32, rs1: u32 },
    FMV_H_X { frd: u32, rs1: u32 },
    // RV64Zfh extension:
    FCVT_L_H { rd: u32, rm: u32, frs1: u32 },
    FCVT_LU_H { rd: u32, rm: u32, frs1: u32 },
    FCVT_H_L { frd: u32, rm: u32, rs1: u32 },
    FCVT_H_LU { frd: u32, rm: u32, rs1: u32 },

    // Zawrs extension:
    WRS_NTO,
    WRS_STO,
    // RV32Zba extension:
    SH1ADD { rd: u32, rs1: u32, rs2: u32 },
    SH2ADD { rd: u32, rs1: u32, rs2: u32 },
    SH3ADD { rd: u32, rs1: u32, rs2: u32 },
    // RV64Zba extension:
    ADD_UW { rd: u32, rs1: u32, rs2: u32 },
    SH1ADD_UW { rd: u32, rs1: u32, rs2: u32 },
    SH2ADD_UW { rd: u32, rs1: u32, rs2: u32 },
    SH3ADD_UW { rd: u32, rs1: u32, rs2: u32 },
    SLLI_UW { rd: u32, rs1: u32, shamt: u32 },
    // RV32Zbb extension:
    ANDN { rd: u32, rs1: u32, rs2: u32 },
    ORN { rd: u32, rs1: u32, rs2: u32 },
    XNOR { rd: u32, rs1: u32, rs2: u32 },
    CLZ { rd: u32, rs1: u32 },
    CTZ { rd: u32, rs1: u32 },
    CPOP { rd: u32, rs1: u32 },
    MAX { rd: u32, rs1: u32, rs2: u32 },
    MAXU { rd: u32, rs1: u32, rs2: u32 },
    MIN { rd: u32, rs1: u32, rs2: u32 },
    MINU { rd: u32, rs1: u32, rs2: u32 },
    SEXT_B { rd: u32, rs1: u32 },
    SEXT_H { rd: u32, rs1: u32 },
    ZEXT_H { rd: u32, rs1: u32 },
    // RV64Zbb extension:
    CLZW { rd: u32, rs1: u32 },
    CTZW { rd: u32, rs1: u32 },
    CPOPW { rd: u32, rs1: u32 },
    // Bitwise rotations (RV32Zbb AND RV32Zbkb extensions):
    ROL { rd: u32, rs1: u32, rs2: u32 },
    ROR { rd: u32, rs1: u32, rs2: u32 },
    RORI { rd: u32, rs1: u32, shamt: u32 },
    ORC_B { rd: u32, rs1: u32 },
    REV8 { rd: u32, rs1: u32 },
    // Bitwise rotations (RV64Zbb AND RV64Zbkb extensions):
    ROLW { rd: u32, rs1: u32, rs2: u32 },
    RORIW { rd: u32, rs1: u32, shamt: u32 },
    RORW { rd: u32, rs1: u32, rs2: u32 },
    // RV32Zbkb extension:
    PACK { rd: u32, rs1: u32, rs2: u32 },
    PACKH { rd: u32, rs1: u32, rs2: u32 },
    BREV8 { rd: u32, rs1: u32 },
    ZIP { rd: u32, rs1: u32 },
    UNZIP { rd: u32, rs1: u32 },
    // RV64Zbkb extension:
    PACKW { rd: u32, rs1: u32, rs2: u32 },
    // Zbc extension:
    CLMUL { rd: u32, rs1: u32, rs2: u32 },
    CLMULH { rd: u32, rs1: u32, rs2: u32 },
    CLMULR { rd: u32, rs1: u32, rs2: u32 },
    // Zbs extension:
    BCLR { rd: u32, rs1: u32, rs2: u32 },
    BCLRI { rd: u32, rs1: u32, shamt: u32 },
    BEXT { rd: u32, rs1: u32, rs2: u32 },
    BEXTI { rd: u32, rs1: u32, shamt: u32 },
    BINV { rd: u32, rs1: u32, rs2: u32 },
    BINVI { rd: u32, rs1: u32, shamt: u32 },
    BSET { rd: u32, rs1: u32, rs2: u32 },
    BSETI { rd: u32, rs1: u32, shamt: u32 },
}

impl Instruction {
    /// Check whether an instruction could branch or jump.
    pub fn branch(&self) -> bool {
        matches!(
            self,
            Instruction::JAL { .. }
                | Instruction::JALR { .. }
                | Instruction::BEQ { .. }
                | Instruction::BNE { .. }
                | Instruction::BLT { .. }
                | Instruction::BGE { .. }
                | Instruction::BLTU { .. }
                | Instruction::BGEU { .. }
        )
    }

    pub fn mem(&self) -> bool {
        (matches!(
            self,
            Instruction::CBO
        ) || self.load() || self.store())
    }

    pub fn float(&self) -> bool {
        matches!(self,
                 Instruction::FLW { .. } | Instruction::FSW { .. } |
                 Instruction::FMADD_S { .. } | Instruction::FMSUB_S { .. } |
                 Instruction::FNMSUB_S { .. } | Instruction::FNMADD_S { .. } |
                 Instruction::FADD_S { .. } | Instruction::FSUB_S { .. } |
                 Instruction::FMUL_S { .. } | Instruction::FDIV_S { .. } |
                 Instruction::FSQRT_S { .. } | Instruction::FSGNJ_S { .. } |
                 Instruction::FSGNJN_S { .. } | Instruction::FSGNJX_S { .. } |
                 Instruction::FMIN_S { .. } | Instruction::FMAX_S { .. } |
                 Instruction::FCVT_W_S { .. } | Instruction::FCVT_WU_S { .. } |
                 Instruction::FMV_X_W { .. } | Instruction::FEQ_S { .. } |
                 Instruction::FLT_S { .. } | Instruction::FLE_S { .. } |
                 Instruction::FCLASS_S { .. } | Instruction::FCVT_S_W { .. } |
                 Instruction::FCVT_S_WU { .. } | Instruction::FMV_W_X { .. } |
                 Instruction::FCVT_L_S { .. } |
                 Instruction::FCVT_LU_S { .. } | Instruction::FCVT_S_L { .. } |
                 Instruction::FCVT_S_LU { .. } |
                 Instruction::FLD { .. } | Instruction::FSD { .. } |
                 Instruction::FMADD_D { .. } | Instruction::FMSUB_D { .. } |
                 Instruction::FNMSUB_D { .. } | Instruction::FNMADD_D { .. } |
                 Instruction::FADD_D { .. } | Instruction::FSUB_D { .. } |
                 Instruction::FMUL_D { .. } | Instruction::FDIV_D { .. } |
                 Instruction::FSQRT_D { .. } | Instruction::FSGNJ_D { .. } |
                 Instruction::FSGNJN_D { .. } | Instruction::FSGNJX_D { .. } |
                 Instruction::FMIN_D { .. } | Instruction::FMAX_D { .. } |
                 Instruction::FCVT_S_D { .. } | Instruction::FCVT_D_S { .. } |
                 Instruction::FEQ_D { .. } | Instruction::FLT_D { .. } |
                 Instruction::FLE_D { .. } | Instruction::FCLASS_D { .. } |
                 Instruction::FCVT_W_D { .. } | Instruction::FCVT_WU_D { .. } |
                 Instruction::FCVT_D_W { .. } | Instruction::FCVT_D_WU { .. } |
                 Instruction::FCVT_L_D { .. } |
                 Instruction::FCVT_LU_D { .. } | Instruction::FMV_X_D { .. } |
                 Instruction::FCVT_D_L { .. } | Instruction::FCVT_D_LU { .. } |
                 Instruction::FMV_D_X { .. } |
                 Instruction::FLQ { .. } | Instruction::FSQ { .. } |
                 Instruction::FMADD_Q { .. } | Instruction::FMSUB_Q { .. } |
                 Instruction::FNMSUB_Q { .. } | Instruction::FNMADD_Q { .. } |
                 Instruction::FADD_Q { .. } | Instruction::FSUB_Q { .. } |
                 Instruction::FMUL_Q { .. } | Instruction::FDIV_Q { .. } |
                 Instruction::FSQRT_Q { .. } | Instruction::FSGNJ_Q { .. } |
                 Instruction::FSGNJN_Q { .. } | Instruction::FSGNJX_Q { .. } |
                 Instruction::FMIN_Q { .. } | Instruction::FMAX_Q { .. } |
                 Instruction::FCVT_S_Q { .. } | Instruction::FCVT_Q_S { .. } |
                 Instruction::FCVT_D_Q { .. } | Instruction::FCVT_Q_D { .. } |
                 Instruction::FEQ_Q { .. } | Instruction::FLT_Q { .. } |
                 Instruction::FLE_Q { .. } | Instruction::FCLASS_Q { .. } |
                 Instruction::FCVT_W_Q { .. } | Instruction::FCVT_WU_Q { .. } |
                 Instruction::FCVT_Q_W { .. } | Instruction::FCVT_Q_WU { .. } |
                 Instruction::FCVT_L_Q { .. } |
                 Instruction::FCVT_LU_Q { .. } | Instruction::FCVT_Q_L { .. } |
                 Instruction::FCVT_Q_LU { .. } |
                 Instruction::FLH { .. } | Instruction::FSH { .. } | Instruction::FMADD_H { .. } |
                 Instruction::FMSUB_H { .. } | Instruction::FNMSUB_H { .. } |
                 Instruction::FNMADD_H { .. } | Instruction::FADD_H { .. } |
                 Instruction::FSUB_H { .. } | Instruction::FMUL_H { .. } |
                 Instruction::FDIV_H { .. } | Instruction::FSQRT_H { .. } |
                 Instruction::FSGNJ_H { .. } | Instruction::FSGNJN_H { .. } |
                 Instruction::FSGNJX_H { .. } | Instruction::FMIN_H { .. } |
                 Instruction::FMAX_H { .. } | Instruction::FCVT_S_H { .. } |
                 Instruction::FCVT_H_S { .. } | Instruction::FCVT_D_H { .. } |
                 Instruction::FCVT_H_D { .. } | Instruction::FCVT_Q_H { .. } |
                 Instruction::FCVT_H_Q { .. } | Instruction::FEQ_H { .. } |
                 Instruction::FLT_H { .. } | Instruction::FLE_H { .. } |
                 Instruction::FCLASS_H { .. } | Instruction::FCVT_W_H { .. } |
                 Instruction::FCVT_WU_H { .. } | Instruction::FMV_X_H { .. } |
                 Instruction::FCVT_H_W { .. } | Instruction::FCVT_H_WU { .. } |
                 Instruction::FMV_H_X { .. } | Instruction::FCVT_L_H { .. } |
                 Instruction::FCVT_LU_H { .. } | Instruction::FCVT_H_L { .. } |
                 Instruction::FCVT_H_LU { .. })
    }

    /// Check whether an instruction could load from memory.
    pub fn load(&self) -> bool {
        matches!(
            self,
            Instruction::LB { .. }
                | Instruction::LH { .. }
                | Instruction::LW { .. }
                | Instruction::LBU { .. }
                | Instruction::LHU { .. }
                | Instruction::LWU { .. }
                | Instruction::LD { .. }
                | Instruction::LR_W { .. }
                | Instruction::LR_D { .. }
                | Instruction::AMOSWAP_W { .. }
                | Instruction::AMOADD_W { .. }
                | Instruction::AMOXOR_W { .. }
                | Instruction::AMOAND_W { .. }
                | Instruction::AMOOR_W { .. }
                | Instruction::AMOMIN_W { .. }
                | Instruction::AMOMAX_W { .. }
                | Instruction::AMOMINU_W { .. }
                | Instruction::AMOMAXU_W { .. }
                | Instruction::AMOSWAP_D { .. }
                | Instruction::AMOADD_D { .. }
                | Instruction::AMOXOR_D { .. }
                | Instruction::AMOAND_D { .. }
                | Instruction::AMOOR_D { .. }
                | Instruction::AMOMIN_D { .. }
                | Instruction::AMOMAX_D { .. }
                | Instruction::AMOMINU_D { .. }
                | Instruction::AMOMAXU_D { .. }
                | Instruction::FLW { .. }
                | Instruction::FLD { .. }
                | Instruction::FLQ { .. }
                | Instruction::FLH { .. }
        )
    }

    pub fn get_rs1(&self) -> Option<u32> {
        match self {
            Instruction::JALR { rs1, .. }       | Instruction::BEQ { rs1, .. } |
            Instruction::BNE { rs1, .. }        | Instruction::BLT { rs1, .. } |
            Instruction::BGE { rs1, .. }        | Instruction::BLTU { rs1, .. } |
            Instruction::BGEU { rs1, .. }       | Instruction::LB { rs1, .. } |
            Instruction::LH { rs1, .. }         | Instruction::LW { rs1, .. } |
            Instruction::LBU { rs1, .. }        | Instruction::LHU { rs1, .. } |
            Instruction::SB { rs1, .. }         | Instruction::SH { rs1, .. } |
            Instruction::SW { rs1, .. }         | Instruction::ADDI { rs1, .. } |
            Instruction::SLTI { rs1, .. }       | Instruction::SLTIU { rs1, .. } |
            Instruction::XORI { rs1, .. }       | Instruction::ORI { rs1, .. } |
            Instruction::ANDI { rs1, .. }       | Instruction::SLLI { rs1, .. } |
            Instruction::SRLI { rs1, .. }       | Instruction::SRAI { rs1, .. } |
            Instruction::ADD { rs1, .. }        | Instruction::SUB { rs1, .. } |
            Instruction::SLL { rs1, .. }        | Instruction::SLT { rs1, .. } |
            Instruction::SLTU { rs1, .. }       | Instruction::XOR { rs1, .. } |
            Instruction::SRL { rs1, .. }        | Instruction::SRA { rs1, .. } |
            Instruction::OR { rs1, .. }         | Instruction::AND { rs1, .. } |
            Instruction::SFENCE_VMA { rs1, .. } | Instruction::LWU { rs1, .. } |
            Instruction::LD { rs1, .. }         | Instruction::SD { rs1, .. } |
            Instruction::ADDIW { rs1, .. }      | Instruction::SLLIW { rs1, .. } |
            Instruction::SRLIW { rs1, .. }      | Instruction::SRAIW { rs1, .. } |
            Instruction::ADDW { rs1, .. }       | Instruction::SUBW { rs1, .. } |
            Instruction::SLLW { rs1, .. }       | Instruction::SRLW { rs1, .. } |
            Instruction::SRAW { rs1, .. }       | Instruction::CSRRW { rs1, .. } |
            Instruction::CSRRS { rs1, .. }      | Instruction::CSRRC { rs1, .. } |
            Instruction::MUL { rs1, .. }        | Instruction::MULH { rs1, .. } |
            Instruction::MULHSU { rs1, .. }     | Instruction::MULHU { rs1, .. } |
            Instruction::DIV { rs1, .. }        | Instruction::DIVU { rs1, .. } |
            Instruction::REM { rs1, .. }        | Instruction::REMU { rs1, .. } |
            Instruction::MULW { rs1, .. }       | Instruction::DIVW { rs1, .. } |
            Instruction::DIVUW { rs1, .. }      | Instruction::REMW { rs1, .. } |
            Instruction::REMUW { rs1, .. }      | Instruction::LR_W { rs1, .. } |
            Instruction::SC_W { rs1, .. }       | Instruction::AMOSWAP_W { rs1, .. } |
            Instruction::AMOADD_W { rs1, .. }   | Instruction::AMOXOR_W { rs1, .. } |
            Instruction::AMOAND_W { rs1, .. }   | Instruction::AMOOR_W { rs1, .. } |
            Instruction::AMOMIN_W { rs1, .. }   | Instruction::AMOMAX_W { rs1, .. } |
            Instruction::AMOMINU_W { rs1, .. }  | Instruction::AMOMAXU_W { rs1, .. } |
            Instruction::LR_D { rs1, .. }       | Instruction::SC_D { rs1, .. } |
            Instruction::AMOSWAP_D { rs1, .. }  | Instruction::AMOADD_D { rs1, .. } |
            Instruction::AMOXOR_D { rs1, .. }   | Instruction::AMOAND_D { rs1, .. } |
            Instruction::AMOOR_D { rs1, .. }    | Instruction::AMOMIN_D { rs1, .. } |
            Instruction::AMOMAX_D { rs1, .. }   | Instruction::AMOMINU_D { rs1, .. } |
            Instruction::AMOMAXU_D { rs1, .. }  | Instruction::FLW { rs1, .. } |
            Instruction::FSW { rs1, .. }        | Instruction::FCVT_S_W { rs1, .. } |
            Instruction::FCVT_S_WU { rs1, .. }  | Instruction::FMV_W_X { rs1, .. } |
            Instruction::FCVT_S_L { rs1, .. }   | Instruction::FCVT_S_LU { rs1, .. } |
            Instruction::FLD { rs1, .. }        | Instruction::FSD { rs1, .. } |
            Instruction::FCVT_D_W { rs1, .. }   | Instruction::FCVT_D_WU { rs1, .. } |
            Instruction::FCVT_D_L { rs1, .. }   | Instruction::FCVT_D_LU { rs1, .. } |
            Instruction::FMV_D_X { rs1, .. }    | Instruction::FLQ { rs1, .. } |
            Instruction::FSQ { rs1, .. }        | Instruction::FCVT_Q_W { rs1, .. } |
            Instruction::FCVT_Q_WU { rs1, .. }  | Instruction::FCVT_Q_L { rs1, .. } |
            Instruction::FCVT_Q_LU { rs1, .. }  | Instruction::FLH { rs1, .. } |
            Instruction::FSH { rs1, .. }        | Instruction::FMV_X_H { rs1, .. } |
            Instruction::FCVT_H_W { rs1, .. }   | Instruction::FCVT_H_WU { rs1, .. } |
            Instruction::FMV_H_X { rs1, .. }    | Instruction::FCVT_H_L { rs1, .. } |
            Instruction::FCVT_H_LU { rs1, .. }  | Instruction::SH1ADD { rs1, .. } |
            Instruction::SH2ADD { rs1, .. }     | Instruction::SH3ADD { rs1, .. } |
            Instruction::ADD_UW { rs1, .. }     | Instruction::SH1ADD_UW { rs1, .. } |
            Instruction::SH2ADD_UW { rs1, .. }  | Instruction::SH3ADD_UW { rs1, .. } |
            Instruction::SLLI_UW { rs1, .. }    | Instruction::ANDN { rs1, .. } |
            Instruction::ORN { rs1, .. }        | Instruction::XNOR { rs1, .. } |
            Instruction::CLZ { rs1, .. }        | Instruction::CTZ { rs1, .. } |
            Instruction::CPOP { rs1, .. }       | Instruction::MAX { rs1, .. } |
            Instruction::MAXU { rs1, .. }       | Instruction::MIN { rs1, .. } |
            Instruction::MINU { rs1, .. }       | Instruction::SEXT_B { rs1, .. } |
            Instruction::SEXT_H { rs1, .. }     | Instruction::ZEXT_H { rs1, .. } |
            Instruction::CLZW { rs1, .. }       | Instruction::CTZW { rs1, .. } |
            Instruction::CPOPW { rs1, .. }      | Instruction::ROL { rs1, .. } |
            Instruction::ROR { rs1, .. }        | Instruction::RORI { rs1, .. } |
            Instruction::ORC_B { rs1, .. }      | Instruction::REV8 { rs1, .. } |
            Instruction::ROLW { rs1, .. }       | Instruction::RORIW { rs1, .. } |
            Instruction::RORW { rs1, .. }       | Instruction::PACK { rs1, .. } |
            Instruction::PACKH { rs1, .. }      | Instruction::BREV8 { rs1, .. } |
            Instruction::ZIP { rs1, .. }        | Instruction::UNZIP { rs1, .. } |
            Instruction::PACKW { rs1, .. }      | Instruction::CLMUL { rs1, .. } |
            Instruction::CLMULH { rs1, .. }     | Instruction::CLMULR { rs1, .. } |
            Instruction::BCLR { rs1, .. }       | Instruction::BCLRI { rs1, .. } |
            Instruction::BEXT { rs1, .. }       | Instruction::BEXTI { rs1, .. } |
            Instruction::BINV { rs1, .. }       | Instruction::BINVI { rs1, .. } |
            Instruction::BSET { rs1, .. }       | Instruction::BSETI { rs1, .. }
            => Some(*rs1),
            _ => None
        }
    }

    pub fn get_rs2(&self) -> Option<u32> {
        match self {
            Instruction::BEQ { rs2, .. }       | Instruction::BNE { rs2, .. } |
            Instruction::BLT { rs2, .. }       | Instruction::BGE { rs2, .. } |
            Instruction::BLTU { rs2, .. }      | Instruction::BGEU { rs2, .. } |
            Instruction::SB { rs2, .. }        | Instruction::SH { rs2, .. } |
            Instruction::SW { rs2, .. }        | Instruction::ADD { rs2, .. } |
            Instruction::SUB { rs2, .. }       | Instruction::SLL { rs2, .. } |
            Instruction::SLT { rs2, .. }       | Instruction::SLTU { rs2, .. } |
            Instruction::XOR { rs2, .. }       | Instruction::SRL { rs2, .. } |
            Instruction::SRA { rs2, .. }       | Instruction::OR { rs2, .. } |
            Instruction::AND { rs2, .. }       | Instruction::SFENCE_VMA { rs2, .. } |
            Instruction::SD { rs2, .. }        | Instruction::ADDW { rs2, .. } |
            Instruction::SUBW { rs2, .. }      | Instruction::SLLW { rs2, .. } |
            Instruction::SRLW { rs2, .. }      | Instruction::SRAW { rs2, .. } |
            Instruction::MUL { rs2, .. }       | Instruction::MULH { rs2, .. } |
            Instruction::MULHSU { rs2, .. }    | Instruction::MULHU { rs2, .. } |
            Instruction::DIV { rs2, .. }       | Instruction::DIVU { rs2, .. } |
            Instruction::REM { rs2, .. }       | Instruction::REMU { rs2, .. } |
            Instruction::MULW { rs2, .. }      | Instruction::DIVW { rs2, .. } |
            Instruction::DIVUW { rs2, .. }     | Instruction::REMW { rs2, .. } |
            Instruction::REMUW { rs2, .. }     | Instruction::SC_W { rs2, .. } |
            Instruction::AMOSWAP_W { rs2, .. } | Instruction::AMOADD_W { rs2, .. } |
            Instruction::AMOXOR_W { rs2, .. }  | Instruction::AMOAND_W { rs2, .. } |
            Instruction::AMOOR_W { rs2, .. }   | Instruction::AMOMIN_W { rs2, .. } |
            Instruction::AMOMAX_W { rs2, .. }  | Instruction::AMOMINU_W { rs2, .. } |
            Instruction::AMOMAXU_W { rs2, .. } | Instruction::SC_D { rs2, .. } |
            Instruction::AMOSWAP_D { rs2, .. } | Instruction::AMOADD_D { rs2, .. } |
            Instruction::AMOXOR_D { rs2, .. }  | Instruction::AMOAND_D { rs2, .. } |
            Instruction::AMOOR_D { rs2, .. }   | Instruction::AMOMIN_D { rs2, .. } |
            Instruction::AMOMAX_D { rs2, .. }  | Instruction::AMOMINU_D { rs2, .. } |
            Instruction::AMOMAXU_D { rs2, .. } | Instruction::SH1ADD { rs2, .. } |
            Instruction::SH2ADD { rs2, .. }    | Instruction::SH3ADD { rs2, .. } |
            Instruction::ADD_UW { rs2, .. }    | Instruction::SH1ADD_UW { rs2, .. } |
            Instruction::SH2ADD_UW { rs2, .. } | Instruction::SH3ADD_UW { rs2, .. } |
            Instruction::ANDN { rs2, .. }      | Instruction::ORN { rs2, .. } |
            Instruction::XNOR { rs2, .. }      | Instruction::MAX { rs2, .. } |
            Instruction::MAXU { rs2, .. }      | Instruction::MIN { rs2, .. } |
            Instruction::MINU { rs2, .. }      | Instruction::ROL { rs2, .. } |
            Instruction::ROR { rs2, .. }       | Instruction::ROLW { rs2, .. } |
            Instruction::RORW { rs2, .. }      | Instruction::PACK { rs2, .. } |
            Instruction::PACKH { rs2, .. }     | Instruction::PACKW { rs2, .. } |
            Instruction::CLMUL { rs2, .. }     | Instruction::CLMULH { rs2, .. } |
            Instruction::CLMULR { rs2, .. }    | Instruction::BCLR { rs2, .. } |
            Instruction::BEXT { rs2, .. }      | Instruction::BINV { rs2, .. } |
            Instruction::BSET { rs2, .. } => Some(*rs2),
            _ => None
        }
    }

    pub fn get_rd(&self) -> Option<u32> {
        match self {
            Instruction::LUI { rd, .. }       | Instruction::AUIPC { rd, .. } |
            Instruction::JAL { rd, .. }       | Instruction::JALR { rd, .. } |
            Instruction::LB { rd, .. }        | Instruction::LH { rd, .. } |
            Instruction::LW { rd, .. }        | Instruction::LBU { rd, .. } |
            Instruction::LHU { rd, .. }       | Instruction::ADDI { rd, .. } |
            Instruction::SLTI { rd, .. }      | Instruction::SLTIU { rd, .. } |
            Instruction::XORI { rd, .. }      | Instruction::ORI { rd, .. } |
            Instruction::ANDI { rd, .. }      | Instruction::SLLI { rd, .. } |
            Instruction::SRLI { rd, .. }      | Instruction::SRAI { rd, .. } |
            Instruction::ADD { rd, .. }       | Instruction::SUB { rd, .. } |
            Instruction::SLL { rd, .. }       | Instruction::SLT { rd, .. } |
            Instruction::SLTU { rd, .. }      | Instruction::XOR { rd, .. } |
            Instruction::SRL { rd, .. }       | Instruction::SRA { rd, .. } |
            Instruction::OR { rd, .. }        | Instruction::AND { rd, .. } |
            Instruction::LWU { rd, .. }       | Instruction::LD { rd, .. } |
            Instruction::ADDIW { rd, .. }     | Instruction::SLLIW { rd, .. } |
            Instruction::SRLIW { rd, .. }     | Instruction::SRAIW { rd, .. } |
            Instruction::ADDW { rd, .. }      | Instruction::SUBW { rd, .. } |
            Instruction::SLLW { rd, .. }      | Instruction::SRLW { rd, .. } |
            Instruction::SRAW { rd, .. }      | Instruction::CSRRW { rd, .. } |
            Instruction::CSRRS { rd, .. }     | Instruction::CSRRC { rd, .. } |
            Instruction::CSRRWI { rd, .. }    | Instruction::CSRRSI { rd, .. } |
            Instruction::CSRRCI { rd, .. }    | Instruction::MUL { rd, .. } |
            Instruction::MULH { rd, .. }      | Instruction::MULHSU { rd, .. } |
            Instruction::MULHU { rd, .. }     | Instruction::DIV { rd, .. } |
            Instruction::DIVU { rd, .. }      | Instruction::REM { rd, .. } |
            Instruction::REMU { rd, .. }      | Instruction::MULW { rd, .. } |
            Instruction::DIVW { rd, .. }      | Instruction::DIVUW { rd, .. } |
            Instruction::REMW { rd, .. }      | Instruction::REMUW { rd, .. } |
            Instruction::LR_W { rd, .. }      | Instruction::SC_W { rd, .. } |
            Instruction::AMOSWAP_W { rd, .. } | Instruction::AMOADD_W { rd, .. } |
            Instruction::AMOXOR_W { rd, .. }  | Instruction::AMOAND_W { rd, .. } |
            Instruction::AMOOR_W { rd, .. }   | Instruction::AMOMIN_W { rd, .. } |
            Instruction::AMOMAX_W { rd, .. }  | Instruction::AMOMINU_W { rd, .. } |
            Instruction::AMOMAXU_W { rd, .. } | Instruction::LR_D { rd, .. } |
            Instruction::SC_D { rd, .. }      | Instruction::AMOSWAP_D { rd, .. } |
            Instruction::AMOADD_D { rd, .. }  | Instruction::AMOXOR_D { rd, .. } |
            Instruction::AMOAND_D { rd, .. }  | Instruction::AMOOR_D { rd, .. } |
            Instruction::AMOMIN_D { rd, .. }  | Instruction::AMOMAX_D { rd, .. } |
            Instruction::AMOMINU_D { rd, .. } | Instruction::AMOMAXU_D { rd, .. } |
            Instruction::FCVT_W_S { rd, .. }  | Instruction::FCVT_WU_S { rd, .. } |
            Instruction::FMV_X_W { rd, .. }   | Instruction::FEQ_S { rd, .. } |
            Instruction::FLT_S { rd, .. }     | Instruction::FLE_S { rd, .. } |
            Instruction::FCLASS_S { rd, .. }  | Instruction::FCVT_L_S { rd, .. } |
            Instruction::FCVT_LU_S { rd, .. } | Instruction::FEQ_D { rd, .. } |
            Instruction::FLT_D { rd, .. }     | Instruction::FLE_D { rd, .. } |
            Instruction::FCLASS_D { rd, .. }  | Instruction::FCVT_W_D { rd, .. } |
            Instruction::FCVT_WU_D { rd, .. } | Instruction::FCVT_L_D { rd, .. } |
            Instruction::FCVT_LU_D { rd, .. } | Instruction::FMV_X_D { rd, .. } |
            Instruction::FEQ_Q { rd, .. }     | Instruction::FLT_Q { rd, .. } |
            Instruction::FLE_Q { rd, .. }     | Instruction::FCLASS_Q { rd, .. } |
            Instruction::FCVT_W_Q { rd, .. }  | Instruction::FCVT_WU_Q { rd, .. } |
            Instruction::FCVT_L_Q { rd, .. }  | Instruction::FCVT_LU_Q { rd, .. } |
            Instruction::FEQ_H { rd, .. }     | Instruction::FLT_H { rd, .. } |
            Instruction::FLE_H { rd, .. }     | Instruction::FCLASS_H { rd, .. } |
            Instruction::FCVT_W_H { rd, .. }  | Instruction::FCVT_WU_H { rd, .. } |
            Instruction::FCVT_L_H { rd, .. }  | Instruction::FCVT_LU_H { rd, .. } |
            Instruction::SH1ADD { rd, .. }    | Instruction::SH2ADD { rd, .. } |
            Instruction::SH3ADD { rd, .. }    | Instruction::ADD_UW { rd, .. } |
            Instruction::SH1ADD_UW { rd, .. } | Instruction::SH2ADD_UW { rd, .. } |
            Instruction::SH3ADD_UW { rd, .. } | Instruction::SLLI_UW { rd, .. } |
            Instruction::ANDN { rd, .. }      | Instruction::ORN { rd, .. } |
            Instruction::XNOR { rd, .. }      | Instruction::CLZ { rd, .. } |
            Instruction::CTZ { rd, .. }       | Instruction::CPOP { rd, .. } |
            Instruction::MAX { rd, .. }       | Instruction::MAXU { rd, .. } |
            Instruction::MIN { rd, .. }       | Instruction::MINU { rd, .. } |
            Instruction::SEXT_B { rd, .. }    | Instruction::SEXT_H { rd, .. } |
            Instruction::ZEXT_H { rd, .. }    | Instruction::CLZW { rd, .. } |
            Instruction::CTZW { rd, .. }      | Instruction::CPOPW { rd, .. } |
            Instruction::ROL { rd, .. }       | Instruction::ROR { rd, .. } |
            Instruction::RORI { rd, .. }      | Instruction::ORC_B { rd, .. } |
            Instruction::REV8 { rd, .. }      | Instruction::ROLW { rd, .. } |
            Instruction::RORIW { rd, .. }     | Instruction::RORW { rd, .. } |
            Instruction::PACK { rd, .. }      | Instruction::PACKH { rd, .. } |
            Instruction::BREV8 { rd, .. }     | Instruction::ZIP { rd, .. } |
            Instruction::UNZIP { rd, .. }     | Instruction::PACKW { rd, .. } |
            Instruction::CLMUL { rd, .. }     | Instruction::CLMULH { rd, .. } |
            Instruction::CLMULR { rd, .. }    | Instruction::BCLR { rd, .. } |
            Instruction::BCLRI { rd, .. }     | Instruction::BEXT { rd, .. } |
            Instruction::BEXTI { rd, .. }     | Instruction::BINV { rd, .. } |
            Instruction::BINVI { rd, .. }     | Instruction::BSET { rd, .. } |
            Instruction::BSETI { rd, .. } => Some(*rd),
            _ => None
        }
    }

    pub fn get_frs1(&self) -> Option<u32> {
        match self {
            Instruction::FMADD_S { frs1, .. }   | Instruction::FMSUB_S { frs1, .. } |
            Instruction::FNMSUB_S { frs1, .. }  | Instruction::FNMADD_S { frs1, .. } |
            Instruction::FADD_S { frs1, .. }    | Instruction::FSUB_S { frs1, .. } |
            Instruction::FMUL_S { frs1, .. }    | Instruction::FDIV_S { frs1, .. } |
            Instruction::FSQRT_S { frs1, .. }   | Instruction::FSGNJ_S { frs1, .. } |
            Instruction::FSGNJN_S { frs1, .. }  | Instruction::FSGNJX_S { frs1, .. } |
            Instruction::FMIN_S { frs1, .. }    | Instruction::FMAX_S { frs1, .. } |
            Instruction::FCVT_W_S { frs1, .. }  | Instruction::FCVT_WU_S { frs1, .. } |
            Instruction::FMV_X_W { frs1, .. }   | Instruction::FEQ_S { frs1, .. } |
            Instruction::FLT_S { frs1, .. }     | Instruction::FLE_S { frs1, .. } |
            Instruction::FCLASS_S { frs1, .. }  | Instruction::FCVT_L_S { frs1, .. } |
            Instruction::FCVT_LU_S { frs1, .. } | Instruction::FMADD_D { frs1, .. } |
            Instruction::FMSUB_D { frs1, .. }   | Instruction::FNMSUB_D { frs1, .. } |
            Instruction::FNMADD_D { frs1, .. }  | Instruction::FADD_D { frs1, .. } |
            Instruction::FSUB_D { frs1, .. }    | Instruction::FMUL_D { frs1, .. } |
            Instruction::FDIV_D { frs1, .. }    | Instruction::FSQRT_D { frs1, .. } |
            Instruction::FSGNJ_D { frs1, .. }   | Instruction::FSGNJN_D { frs1, .. } |
            Instruction::FSGNJX_D { frs1, .. }  | Instruction::FMIN_D { frs1, .. } |
            Instruction::FMAX_D { frs1, .. }    | Instruction::FCVT_S_D { frs1, .. } |
            Instruction::FCVT_D_S { frs1, .. }  | Instruction::FEQ_D { frs1, .. } |
            Instruction::FLT_D { frs1, .. }     | Instruction::FLE_D { frs1, .. } |
            Instruction::FCLASS_D { frs1, .. }  | Instruction::FCVT_W_D { frs1, .. } |
            Instruction::FCVT_WU_D { frs1, .. } | Instruction::FCVT_L_D { frs1, .. } |
            Instruction::FCVT_LU_D { frs1, .. } | Instruction::FMV_X_D { frs1, .. } |
            Instruction::FMADD_Q { frs1, .. }   | Instruction::FMSUB_Q { frs1, .. } |
            Instruction::FNMSUB_Q { frs1, .. }  | Instruction::FNMADD_Q { frs1, .. } |
            Instruction::FADD_Q { frs1, .. }    | Instruction::FSUB_Q { frs1, .. } |
            Instruction::FMUL_Q { frs1, .. }    | Instruction::FDIV_Q { frs1, .. } |
            Instruction::FSQRT_Q { frs1, .. }   | Instruction::FSGNJ_Q { frs1, .. } |
            Instruction::FSGNJN_Q { frs1, .. }  | Instruction::FSGNJX_Q { frs1, .. } |
            Instruction::FMIN_Q { frs1, .. }    | Instruction::FMAX_Q { frs1, .. } |
            Instruction::FCVT_S_Q { frs1, .. }  | Instruction::FCVT_Q_S { frs1, .. } |
            Instruction::FCVT_D_Q { frs1, .. }  | Instruction::FCVT_Q_D { frs1, .. } |
            Instruction::FEQ_Q { frs1, .. }     | Instruction::FLT_Q { frs1, .. } |
            Instruction::FLE_Q { frs1, .. }     | Instruction::FCLASS_Q { frs1, .. } |
            Instruction::FCVT_W_Q { frs1, .. }  | Instruction::FCVT_WU_Q { frs1, .. } |
            Instruction::FCVT_L_Q { frs1, .. }  | Instruction::FCVT_LU_Q { frs1, .. } |
            Instruction::FMADD_H { frs1, .. }   | Instruction::FMSUB_H { frs1, .. } |
            Instruction::FNMSUB_H { frs1, .. }  | Instruction::FNMADD_H { frs1, .. } |
            Instruction::FADD_H { frs1, .. }    | Instruction::FSUB_H { frs1, .. } |
            Instruction::FMUL_H { frs1, .. }    | Instruction::FDIV_H { frs1, .. } |
            Instruction::FSQRT_H { frs1, .. }   | Instruction::FSGNJ_H { frs1, .. } |
            Instruction::FSGNJN_H { frs1, .. }  | Instruction::FSGNJX_H { frs1, .. } |
            Instruction::FMIN_H { frs1, .. }    | Instruction::FMAX_H { frs1, .. } |
            Instruction::FCVT_S_H { frs1, .. }  | Instruction::FCVT_H_S { frs1, .. } |
            Instruction::FCVT_D_H { frs1, .. }  | Instruction::FCVT_H_D { frs1, .. } |
            Instruction::FCVT_Q_H { frs1, .. }  | Instruction::FCVT_H_Q { frs1, .. } |
            Instruction::FEQ_H { frs1, .. }     | Instruction::FLT_H { frs1, .. } |
            Instruction::FLE_H { frs1, .. }     | Instruction::FCLASS_H { frs1, .. } |
            Instruction::FCVT_W_H { frs1, .. }  | Instruction::FCVT_WU_H { frs1, .. } |
            Instruction::FCVT_L_H { frs1, .. }  | Instruction::FCVT_LU_H { frs1, .. }
            => Some(*frs1),
            _ => None
        }
    }

    pub fn get_frs2(&self) -> Option<u32> {
        match self {
            Instruction::FSW { frs2, .. }      | Instruction::FMADD_S { frs2, .. } |
            Instruction::FMSUB_S { frs2, .. }  | Instruction::FNMSUB_S { frs2, .. } |
            Instruction::FNMADD_S { frs2, .. } | Instruction::FADD_S { frs2, .. } |
            Instruction::FSUB_S { frs2, .. }   | Instruction::FMUL_S { frs2, .. } |
            Instruction::FDIV_S { frs2, .. }   | Instruction::FSGNJ_S { frs2, .. } |
            Instruction::FSGNJN_S { frs2, .. } | Instruction::FSGNJX_S { frs2, .. } |
            Instruction::FMIN_S { frs2, .. }   | Instruction::FMAX_S { frs2, .. } |
            Instruction::FEQ_S { frs2, .. }    | Instruction::FLT_S { frs2, .. } |
            Instruction::FLE_S { frs2, .. }    | Instruction::FSD { frs2, .. } |
            Instruction::FMADD_D { frs2, .. }  | Instruction::FMSUB_D { frs2, .. } |
            Instruction::FNMSUB_D { frs2, .. } | Instruction::FNMADD_D { frs2, .. } |
            Instruction::FADD_D { frs2, .. }   | Instruction::FSUB_D { frs2, .. } |
            Instruction::FMUL_D { frs2, .. }   | Instruction::FDIV_D { frs2, .. } |
            Instruction::FSGNJ_D { frs2, .. }  | Instruction::FSGNJN_D { frs2, .. } |
            Instruction::FSGNJX_D { frs2, .. } | Instruction::FMIN_D { frs2, .. } |
            Instruction::FMAX_D { frs2, .. }   | Instruction::FEQ_D { frs2, .. } |
            Instruction::FLT_D { frs2, .. }    | Instruction::FLE_D { frs2, .. } |
            Instruction::FSQ { frs2, .. }      | Instruction::FMADD_Q { frs2, .. } |
            Instruction::FMSUB_Q { frs2, .. }  | Instruction::FNMSUB_Q { frs2, .. } |
            Instruction::FNMADD_Q { frs2, .. } | Instruction::FADD_Q { frs2, .. } |
            Instruction::FSUB_Q { frs2, .. }   | Instruction::FMUL_Q { frs2, .. } |
            Instruction::FDIV_Q { frs2, .. }   | Instruction::FSGNJ_Q { frs2, .. } |
            Instruction::FSGNJN_Q { frs2, .. } | Instruction::FSGNJX_Q { frs2, .. } |
            Instruction::FMIN_Q { frs2, .. }   | Instruction::FMAX_Q { frs2, .. } |
            Instruction::FEQ_Q { frs2, .. }    | Instruction::FLT_Q { frs2, .. } |
            Instruction::FLE_Q { frs2, .. }    | Instruction::FSH { frs2, .. } |
            Instruction::FMADD_H { frs2, .. }  | Instruction::FMSUB_H { frs2, .. } |
            Instruction::FNMSUB_H { frs2, .. } | Instruction::FNMADD_H { frs2, .. } |
            Instruction::FADD_H { frs2, .. }   | Instruction::FSUB_H { frs2, .. } |
            Instruction::FMUL_H { frs2, .. }   | Instruction::FDIV_H { frs2, .. } |
            Instruction::FSGNJ_H { frs2, .. }  | Instruction::FSGNJN_H { frs2, .. } |
            Instruction::FSGNJX_H { frs2, .. } | Instruction::FMIN_H { frs2, .. } |
            Instruction::FMAX_H { frs2, .. }   | Instruction::FEQ_H { frs2, .. } |
            Instruction::FLT_H { frs2, .. }    | Instruction::FLE_H { frs2, .. }
            => Some(*frs2),
            _ => None
        }
    }

    pub fn get_frs3(&self) -> Option<u32> {
        match self {
            Instruction::FMADD_S { frs3, .. }  | Instruction::FMSUB_S { frs3, .. } |
            Instruction::FNMSUB_S { frs3, .. } | Instruction::FNMADD_S { frs3, .. } |
            Instruction::FMADD_D { frs3, .. }  | Instruction::FMSUB_D { frs3, .. } |
            Instruction::FNMSUB_D { frs3, .. } | Instruction::FNMADD_D { frs3, .. } |
            Instruction::FMADD_Q { frs3, .. }  | Instruction::FMSUB_Q { frs3, .. } |
            Instruction::FNMSUB_Q { frs3, .. } | Instruction::FNMADD_Q { frs3, .. } |
            Instruction::FMADD_H { frs3, .. }  | Instruction::FMSUB_H { frs3, .. } |
            Instruction::FNMSUB_H { frs3, .. } | Instruction::FNMADD_H { frs3, .. }
            => Some(*frs3),
            _ => None
        }
    }

    pub fn get_frd(&self) -> Option<u32> {
        match self {
            Instruction::FLW { frd, .. }       | Instruction::FMADD_S { frd, .. } |
            Instruction::FMSUB_S { frd, .. }   | Instruction::FNMSUB_S { frd, .. } |
            Instruction::FNMADD_S { frd, .. }  | Instruction::FADD_S { frd, .. } |
            Instruction::FSUB_S { frd, .. }    | Instruction::FMUL_S { frd, .. } |
            Instruction::FDIV_S { frd, .. }    | Instruction::FSQRT_S { frd, .. } |
            Instruction::FSGNJ_S { frd, .. }   | Instruction::FSGNJN_S { frd, .. } |
            Instruction::FSGNJX_S { frd, .. }  | Instruction::FMIN_S { frd, .. } |
            Instruction::FMAX_S { frd, .. }    | Instruction::FCVT_S_W { frd, .. } |
            Instruction::FCVT_S_WU { frd, .. } | Instruction::FMV_W_X { frd, .. } |
            Instruction::FCVT_S_L { frd, .. }  | Instruction::FCVT_S_LU { frd, .. } |
            Instruction::FLD { frd, .. }       | Instruction::FMADD_D { frd, .. } |
            Instruction::FMSUB_D { frd, .. }   | Instruction::FNMSUB_D { frd, .. } |
            Instruction::FNMADD_D { frd, .. }  | Instruction::FADD_D { frd, .. } |
            Instruction::FSUB_D { frd, .. }    | Instruction::FMUL_D { frd, .. } |
            Instruction::FDIV_D { frd, .. }    | Instruction::FSQRT_D { frd, .. } |
            Instruction::FSGNJ_D { frd, .. }   | Instruction::FSGNJN_D { frd, .. } |
            Instruction::FSGNJX_D { frd, .. }  | Instruction::FMIN_D { frd, .. } |
            Instruction::FMAX_D { frd, .. }    | Instruction::FCVT_S_D { frd, .. } |
            Instruction::FCVT_D_S { frd, .. }  | Instruction::FCVT_D_W { frd, .. } |
            Instruction::FCVT_D_WU { frd, .. } | Instruction::FCVT_D_L { frd, .. } |
            Instruction::FCVT_D_LU { frd, .. } | Instruction::FMV_D_X { frd, .. } |
            Instruction::FLQ { frd, .. }       | Instruction::FMADD_Q { frd, .. } |
            Instruction::FMSUB_Q { frd, .. }   | Instruction::FNMSUB_Q { frd, .. } |
            Instruction::FNMADD_Q { frd, .. }  | Instruction::FADD_Q { frd, .. } |
            Instruction::FSUB_Q { frd, .. }    | Instruction::FMUL_Q { frd, .. } |
            Instruction::FDIV_Q { frd, .. }    | Instruction::FSQRT_Q { frd, .. } |
            Instruction::FSGNJ_Q { frd, .. }   | Instruction::FSGNJN_Q { frd, .. } |
            Instruction::FSGNJX_Q { frd, .. }  | Instruction::FMIN_Q { frd, .. } |
            Instruction::FMAX_Q { frd, .. }    | Instruction::FCVT_S_Q { frd, .. } |
            Instruction::FCVT_Q_S { frd, .. }  | Instruction::FCVT_D_Q { frd, .. } |
            Instruction::FCVT_Q_D { frd, .. }  | Instruction::FCVT_Q_W { frd, .. } |
            Instruction::FCVT_Q_WU { frd, .. } | Instruction::FCVT_Q_L { frd, .. } |
            Instruction::FCVT_Q_LU { frd, .. } | Instruction::FLH { frd, .. } |
            Instruction::FMADD_H { frd, .. }   | Instruction::FMSUB_H { frd, .. } |
            Instruction::FNMSUB_H { frd, .. }  | Instruction::FNMADD_H { frd, .. } |
            Instruction::FADD_H { frd, .. }    | Instruction::FSUB_H { frd, .. } |
            Instruction::FMUL_H { frd, .. }    | Instruction::FDIV_H { frd, .. } |
            Instruction::FSQRT_H { frd, .. }   | Instruction::FSGNJ_H { frd, .. } |
            Instruction::FSGNJN_H { frd, .. }  | Instruction::FSGNJX_H { frd, .. } |
            Instruction::FMIN_H { frd, .. }    | Instruction::FMAX_H { frd, .. } |
            Instruction::FCVT_S_H { frd, .. }  | Instruction::FCVT_H_S { frd, .. } |
            Instruction::FCVT_D_H { frd, .. }  | Instruction::FCVT_H_D { frd, .. } |
            Instruction::FCVT_Q_H { frd, .. }  | Instruction::FCVT_H_Q { frd, .. } |
            Instruction::FMV_X_H { frd, .. }   | Instruction::FCVT_H_W { frd, .. } |
            Instruction::FCVT_H_WU { frd, .. } | Instruction::FMV_H_X { frd, .. } |
            Instruction::FCVT_H_L { frd, .. }  | Instruction::FCVT_H_LU { frd, .. }
            => Some(*frd),
            _ => None
        }
    }

    /// Check whether an instruction could store to memory.
    pub fn store(&self) -> bool {
        matches!(
            self,
            Instruction::SB { .. }
                | Instruction::SH { .. }
                | Instruction::SW { .. }
                | Instruction::SD { .. }
                | Instruction::SC_W { .. }
                | Instruction::SC_D { .. }
                | Instruction::AMOSWAP_W { .. }
                | Instruction::AMOADD_W { .. }
                | Instruction::AMOXOR_W { .. }
                | Instruction::AMOAND_W { .. }
                | Instruction::AMOOR_W { .. }
                | Instruction::AMOMIN_W { .. }
                | Instruction::AMOMAX_W { .. }
                | Instruction::AMOMINU_W { .. }
                | Instruction::AMOMAXU_W { .. }
                | Instruction::AMOSWAP_D { .. }
                | Instruction::AMOADD_D { .. }
                | Instruction::AMOXOR_D { .. }
                | Instruction::AMOAND_D { .. }
                | Instruction::AMOOR_D { .. }
                | Instruction::AMOMIN_D { .. }
                | Instruction::AMOMAX_D { .. }
                | Instruction::AMOMINU_D { .. }
                | Instruction::AMOMAXU_D { .. }
                | Instruction::FSW { .. }
                | Instruction::FSD { .. }
                | Instruction::FSQ { .. }
                | Instruction::FSH { .. }
        )
    }
}

/// Compressed RISC-V instruction.
///
/// Compressed instructions can be decompressed using [`Instruction::from`].
///
/// Instruction arguments are all in their decoded forms, meaning correctly
/// scaled and sign extended. Registers are normalised. Their names are as they
/// appear in specifications (for the most part), and include:
///
/// * `rd`, `rs1`, `rs2``: destination and source registers.
/// * `frd`, `frs2`: destination and source floating point registers.
/// * `offset`, `imm`, `shamt`: numerical offsets, immediates, and shift amounts.
/// * `pred`, `succ`: predecessor and successor IORW (input, output, read, write) flags.
/// * `rm`: floating point rounding mode.
/// * `rl`, `aq`: atomic release and acquire flags.
///
/// Assumed and redundant flags are not repeated, i.e. `rd` is not stored for
/// `c.swsp` as it's always `sp` (`x2`) and only `rd` is stored for `c.addi` as
/// `rs1` is always the same.
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compressed {
    // Unknown:
    UNIMP,
    // Zca extension:
    // Stack-pointer based loads and stores:
    C_LWSP { rd: u32, offset: i32 },
    C_LDSP { rd: u32, offset: i32 },
    C_LQSP { rd: u32, offset: i32 },
    C_SWSP { rs2: u32, offset: i32 },
    C_SDSP { rs2: u32, offset: i32 },
    C_SQSP { rs2: u32, offset: i32 },
    // Register based loads and stores:
    C_LW { rd: u32, rs1: u32, offset: i32 },
    C_LD { rd: u32, rs1: u32, offset: i32 },
    C_LQ { rd: u32, rs1: u32, offset: i32 },
    C_SW { rs1: u32, rs2: u32, offset: i32 },
    C_SD { rs1: u32, rs2: u32, offset: i32 },
    C_SQ { rs1: u32, rs2: u32, offset: i32 },
    // Control transfer instructions:
    C_J { offset: i32 },
    C_JAL { offset: i32 },
    C_JR { rs1: u32 },
    C_JALR { rs1: u32 },
    C_BEQZ { rs1: u32, offset: i32 },
    C_BNEZ { rs1: u32, offset: i32 },
    // Integer computation instructions:
    C_LI { rd: u32, imm: i32 },
    C_LUI { rd: u32, imm: u32 },
    C_ADDI { rd: u32, imm: i32 },
    C_ADDIW { rd: u32, imm: i32 },
    C_ADDI16SP { imm: i32 },
    C_ADDI4SPN { rd: u32, imm: i32 },
    C_SLLI { rd: u32, shamt: u32 },
    C_SRLI { rd: u32, shamt: u32 },
    C_SRAI { rd: u32, shamt: u32 },
    C_ANDI { rd: u32, imm: i32 },
    C_MV { rd: u32, rs2: u32 },
    C_ADD { rd: u32, rs2: u32 },
    C_AND { rd: u32, rs2: u32 },
    C_OR { rd: u32, rs2: u32 },
    C_XOR { rd: u32, rs2: u32 },
    C_SUB { rd: u32, rs2: u32 },
    C_ADDW { rd: u32, rs2: u32 },
    C_SUBW { rd: u32, rs2: u32 },
    // Misc and system instructions:
    C_NOP,
    C_EBREAK,
    // RV32Zcf extension:
    C_FLW { frd: u32, rs1: u32, offset: i32 },
    C_FLWSP { frd: u32, offset: i32 },
    C_FSW { rs1: u32, frs2: u32, offset: i32 },
    C_FSWSP { frs2: u32, offset: i32 },
    // Zcd extension:
    C_FLD { frd: u32, rs1: u32, offset: i32 },
    C_FLDSP { frd: u32, offset: i32 },
    C_FSD { rs1: u32, frs2: u32, offset: i32 },
    C_FSDSP { frs2: u32, offset: i32 },
}

impl From<Compressed> for Instruction {
    fn from(compressed: Compressed) -> Self {
        match compressed {
            Compressed::UNIMP => Instruction::UNIMP,
            // Zca extension:
            Compressed::C_LWSP { rd, offset } => Instruction::LW { rd, rs1: 2, offset },
            Compressed::C_LDSP { rd, offset } => Instruction::LD { rd, rs1: 2, offset },
            Compressed::C_LQSP { .. } => unimplemented!("rv128"),
            Compressed::C_SWSP { rs2, offset } => Instruction::SW { rs1: 2, rs2, offset },
            Compressed::C_SDSP { rs2, offset } => Instruction::SD { rs1: 2, rs2, offset },
            Compressed::C_SQSP { .. } => unimplemented!("rv128"),
            Compressed::C_LW { rd, rs1, offset } => Instruction::LW { rd, rs1, offset },
            Compressed::C_LD { rd, rs1, offset } => Instruction::LD { rd, rs1, offset },
            Compressed::C_LQ { .. } => unimplemented!("rv128"),
            Compressed::C_SW { rs1, rs2, offset } => Instruction::SW { rs1, rs2, offset },
            Compressed::C_SD { rs1, rs2, offset } => Instruction::SD { rs1, rs2, offset },
            Compressed::C_SQ { .. } => unimplemented!("rv128"),
            Compressed::C_J { offset } => Instruction::JAL { rd: 0, offset },
            Compressed::C_JAL { offset } => Instruction::JAL { rd: 1, offset },
            Compressed::C_JR { rs1 } => Instruction::JALR { rd: 0, rs1, offset: 0 },
            Compressed::C_JALR { rs1 } => Instruction::JALR { rd: 1, rs1, offset: 0 },
            Compressed::C_BEQZ { rs1, offset } => Instruction::BEQ { rs1, rs2: 0, offset },
            Compressed::C_BNEZ { rs1, offset } => Instruction::BNE { rs1, rs2: 0, offset },
            Compressed::C_LI { rd, imm } => Instruction::ADDI { rd, rs1: 0, imm },
            Compressed::C_LUI { rd, imm } => Instruction::LUI { rd, imm },
            Compressed::C_ADDI { rd, imm } => Instruction::ADDI { rd, rs1: rd, imm },
            Compressed::C_ADDIW { rd, imm } => Instruction::ADDIW { rd, rs1: rd, imm },
            Compressed::C_ADDI16SP { imm } => Instruction::ADDI { rd: 2, rs1: 2, imm },
            Compressed::C_ADDI4SPN { rd, imm } => Instruction::ADDI { rd, rs1: 2, imm },
            Compressed::C_SLLI { rd, shamt } => Instruction::SLLI { rd, rs1: rd, shamt },
            Compressed::C_SRLI { rd, shamt } => Instruction::SRLI { rd, rs1: rd, shamt },
            Compressed::C_SRAI { rd, shamt } => Instruction::SRAI { rd, rs1: rd, shamt },
            Compressed::C_ANDI { rd, imm } => Instruction::ANDI { rd, rs1: rd, imm },
            Compressed::C_MV { rd, rs2 } => Instruction::ADD { rd, rs1: 0, rs2 },
            Compressed::C_ADD { rd, rs2 } => Instruction::ADD { rd, rs1: rd, rs2 },
            Compressed::C_AND { rd, rs2 } => Instruction::AND { rd, rs1: rd, rs2 },
            Compressed::C_OR { rd, rs2 } => Instruction::OR { rd, rs1: rd, rs2 },
            Compressed::C_XOR { rd, rs2 } => Instruction::XOR { rd, rs1: rd, rs2 },
            Compressed::C_SUB { rd, rs2 } => Instruction::SUB { rd, rs1: rd, rs2 },
            Compressed::C_ADDW { rd, rs2 } => Instruction::ADDW { rd, rs1: rd, rs2 },
            Compressed::C_SUBW { rd, rs2 } => Instruction::SUBW { rd, rs1: rd, rs2 },
            Compressed::C_NOP => Instruction::ADDI { rd: 0, rs1: 0, imm: 0 },
            Compressed::C_EBREAK => Instruction::EBREAK,
            // Zcf extension:
            Compressed::C_FLW { frd, rs1, offset } => Instruction::FLW { frd, rs1, offset },
            Compressed::C_FLWSP { frd, offset } => Instruction::FLW { frd, rs1: 2, offset },
            Compressed::C_FSW { rs1, frs2, offset } => Instruction::FSW { rs1, frs2, offset },
            Compressed::C_FSWSP { frs2, offset } => Instruction::FSW { rs1: 2, frs2, offset },
            // Zcd extension:
            Compressed::C_FLD { frd, rs1, offset } => Instruction::FLD { frd, rs1, offset },
            Compressed::C_FLDSP { frd, offset } => Instruction::FLD { frd, rs1: 2, offset },
            Compressed::C_FSD { rs1, frs2, offset } => Instruction::FSD { rs1, frs2, offset },
            Compressed::C_FSDSP { frs2, offset } => Instruction::FSD { rs1: 2, frs2, offset },
        }
    }
}

impl Compressed {
    /// Check whether an instruction could branch or jump.
    pub fn branch(&self) -> bool {
        matches!(
            self,
            Compressed::C_J { .. }
                | Compressed::C_JAL { .. }
                | Compressed::C_JR { .. }
                | Compressed::C_JALR { .. }
                | Compressed::C_BEQZ { .. }
                | Compressed::C_BNEZ { .. }
        )
    }
}

/// Bitfield for input/output/read/write fields, e.g. for `FENCE`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Iorw(pub(crate) u8);
