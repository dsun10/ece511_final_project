#include "fusion.hh"
#include "params/BaseO3CPU.hh"

namespace gem5::RiscvISAInst
{
using namespace RiscvISA;
}

#include <fenv.h>

#include "arch/riscv/generated/decoder.hh"
#include "cpu/o3/dyn_inst.hh"

namespace gem5
{

namespace RiscvISA
{
// A x1, x2, x3 + B x1, x1, x4
// => A fuseTmp, x2, x3 + B x1, fuseTmp, x4
// => C x1, (x2, x3), (fuseTmp, x4)

Addr FusionInst::getSecondPC() const { return second->getPC(); }

Addr FusionInst::getSecondFallThruPC() const
{
    return (second->pcState()).getFallThruPC();
}

bool FusionInst::getSecondBranching() const
{
  return (second->pcState()).branching();
}

o3::DynInstPtr FusionInst::getSecondInst() const
{
  return second;
}

class ChainFusionInst : public FusionInst
{
    // only have one dst
    int firstNumSrcs = 0;
  public:
    ChainFusionInst(const char *name, OpClass op, o3::DynInstPtr first, o3::DynInstPtr second)
        : FusionInst(name, op, first, second)
    {
        panic_if(first->destRegIdx(0) != second->destRegIdx(0),
                 "ChainFusionInst: first and second insts must have the same destination register");
        setDestRegIdx(_numDestRegs++, first->destRegIdx(0)); _numTypedDestRegs[first->destRegIdx(0).classValue()]++;
        for (int i = 0; i < first->numSrcRegs(); ++i) {
            setSrcRegIdx(_numSrcRegs++, first->srcRegIdx(i));
        }
        firstNumSrcs = first->numSrcRegs();

        for (int i = 0; i < second->numSrcRegs(); ++i) {
            if (second->srcRegIdx(i) == first->destRegIdx(0)) {
                // if the second inst's src is the first inst's dst, we use a temporary reg
                setSrcRegIdx(_numSrcRegs++, FuseTmpReg);
            } else {
                // otherwise, just use the second inst's src reg
                setSrcRegIdx(_numSrcRegs++, second->srcRegIdx(i));
            }
        }

        flags = first->staticInst->getFlags() | second->staticInst->getFlags();
    }

    Fault execute(ExecContext *xc, Trace::InstRecord *traceData) const override
    {
      assert(fused);

      PhysRegIdPtr fuseTmp = nullptr;
      for (int i = 0; i < fused->numSrcRegs(); ++i) {
        if (fused->srcRegIdx(i) == FuseTmpReg) {
          fuseTmp = fused->renamedSrcIdx(i);
          break;
        }
      }
      if (fuseTmp) {
        first->renameDestReg(0, o3::VirtRegId(fuseTmp), o3::VirtRegId());
      } else {
        first->renameDestReg(0, fused->extRenamedDestIdx(0), o3::VirtRegId());
      }

      second->renameDestReg(0, fused->extRenamedDestIdx(0), o3::VirtRegId());

      // rename
      for (int i = 0; i < numSrcRegs(); i++) {
        if (i < firstNumSrcs) {
          // first inst's src
          first->renameSrcReg(i, fused->extRenamedSrcIdx(i));
        } else {
          // second inst's src
          second->renameSrcReg(i - firstNumSrcs, fused->extRenamedSrcIdx(i));
        }
      }

      // execute
      Fault fault = first->execute();
      if (fault != NoFault)
        return fault;
      fault = second->execute();

      return fault;
    }
};

class AluBrFusionInst : public FusionInst
{
    // only have one dst
    int firstNumSrcs = 0;
  public:
    AluBrFusionInst(const char *name, o3::DynInstPtr first, o3::DynInstPtr second)
    : FusionInst(name, OpClass::IntABr, first, second)
    {
        panic_if(first->destRegIdx(0) != second->srcRegIdx(0) && first->destRegIdx(0) != second->srcRegIdx(1),
                 "AluBR: first inst must pipe a result into second inst");

        // 1. Set Destination (The ALU's output)
        setDestRegIdx(_numDestRegs++, first->destRegIdx(0)); _numTypedDestRegs[first->destRegIdx(0).classValue()]++;
        for (int i = 0; i < first->numSrcRegs(); ++i) {
            setSrcRegIdx(_numSrcRegs++, first->srcRegIdx(i));
        }
        firstNumSrcs = first->numSrcRegs();

        for (int i = 0; i < second->numSrcRegs(); ++i) {
            if (second->srcRegIdx(i) != first->destRegIdx(0)) {
                setSrcRegIdx(_numSrcRegs++, second->srcRegIdx(i));
            }
        }

        flags = first->staticInst->getFlags() | second->staticInst->getFlags();
    }

    Fault execute(ExecContext *xc, Trace::InstRecord *traceData) const override
    {
        assert(fused);

        first->renameDestReg(0, fused->extRenamedDestIdx(0), o3::VirtRegId());
        first->renameSrcReg(0, fused->extRenamedSrcIdx(0));

        if (second->srcRegIdx(0) == first->destRegIdx(0)) {
          second->renameSrcReg(0, fused->extRenamedDestIdx(0));
          second->renameSrcReg(1, fused->extRenamedSrcIdx(1));
        } else {
          second->renameSrcReg(1, fused->extRenamedDestIdx(0));
          second->renameSrcReg(0, fused->extRenamedSrcIdx(1));
        }

        Fault fault = first->execute();
        if (fault != NoFault) { return fault; }

        auto second_fault =  second->execute();

        RiscvISA::PCState thispc;
        thispc.set(first->getPC());
        thispc.setNPC(second->getNPC());

        xc->pcState(thispc);

       return second_fault;
    }
};


// mul rd_mul, rs1, rs2 (first) + add/addw rd_add, rx, ry (second)
// where rd_mul == rx or ry, and rd_add may differ from rd_mul.
// MUL result is passed to ADD via FuseTmpReg; fused dest is ADD's destination.
//
// Source layout: [rs1_mul, rs2_mul, FuseTmpReg, rs_accum]
//   rs_accum is whichever ADD source is NOT the MUL result.
class MaccFusionInst : public FusionInst
{
    int firstNumSrcs = 0;
  public:
    MaccFusionInst(const char *name, OpClass op, o3::DynInstPtr first, o3::DynInstPtr second)
        : FusionInst(name, op, first, second)
    {
        panic_if(first->destRegIdx(0) != second->srcRegIdx(0) &&
                 first->destRegIdx(0) != second->srcRegIdx(1),
                 "MaccFusionInst: MUL result must be consumed by ADD/ADDW");

        // Count how many of ADD's sources are the MUL result — must be exactly one
        // so there is always a distinct accumulator source to track.
        int matchCount = 0;
        for (int i = 0; i < second->numSrcRegs(); ++i) {
            if (second->srcRegIdx(i) == first->destRegIdx(0)) matchCount++;
        }
        panic_if(matchCount == second->numSrcRegs(),
                 "MaccFusionInst: both ADD sources equal the MUL dest — use ChainFusionInst");

        // Fused destination is ADD's output register
        setDestRegIdx(_numDestRegs++, second->destRegIdx(0));
        _numTypedDestRegs[second->destRegIdx(0).classValue()]++;

        // Sources [0..firstNumSrcs-1]: MUL's register inputs (rs1, rs2)
        for (int i = 0; i < first->numSrcRegs(); ++i) {
            setSrcRegIdx(_numSrcRegs++, first->srcRegIdx(i));
        }
        firstNumSrcs = first->numSrcRegs();

        // Source [firstNumSrcs]: FuseTmpReg — placeholder for the MUL result
        setSrcRegIdx(_numSrcRegs++, FuseTmpReg);

        // Source [firstNumSrcs+1]: ADD's accumulator (the source that is NOT the MUL result)
        for (int i = 0; i < second->numSrcRegs(); ++i) {
            if (second->srcRegIdx(i) != first->destRegIdx(0)) {
                setSrcRegIdx(_numSrcRegs++, second->srcRegIdx(i));
                break;
            }
        }

        flags = first->staticInst->getFlags() | second->staticInst->getFlags();
    }

    Fault execute(ExecContext *xc, Trace::InstRecord *traceData) const override
    {
        assert(fused);

        // Locate the physical register backing FuseTmpReg
        PhysRegIdPtr fuseTmp = nullptr;
        for (int i = 0; i < fused->numSrcRegs(); ++i) {
            if (fused->srcRegIdx(i) == FuseTmpReg) {
                fuseTmp = fused->renamedSrcIdx(i);
                break;
            }
        }
        assert(fuseTmp);

        // MUL writes its result to the physical temp
        first->renameDestReg(0, o3::VirtRegId(fuseTmp), o3::VirtRegId());

        // ADD writes to the fused destination
        second->renameDestReg(0, fused->extRenamedDestIdx(0), o3::VirtRegId());

        // Rename MUL's source registers
        for (int i = 0; i < firstNumSrcs; ++i) {
            first->renameSrcReg(i, fused->extRenamedSrcIdx(i));
        }

        // Rename ADD's source registers:
        //   whichever ADD source was rd_mul reads from FuseTmpReg (src[firstNumSrcs])
        //   the other ADD source (accumulator) reads from src[firstNumSrcs+1]
        if (second->srcRegIdx(0) == first->destRegIdx(0)) {
            second->renameSrcReg(0, fused->extRenamedSrcIdx(firstNumSrcs));
            second->renameSrcReg(1, fused->extRenamedSrcIdx(firstNumSrcs + 1));
        } else {
            second->renameSrcReg(1, fused->extRenamedSrcIdx(firstNumSrcs));
            second->renameSrcReg(0, fused->extRenamedSrcIdx(firstNumSrcs + 1));
        }

        Fault fault = first->execute();
        if (fault != NoFault) return fault;
        return second->execute();
    }
};

// add x1, x2, x3 + ld x1, offset(x1)
template<int memsize>
class AluLoadFusionInst : public FusionInst
{
    int firstNumSrcs = 0;

    Request::Flags memAccessFlags;
  public:
    AluLoadFusionInst(const char *name, OpClass op, const o3::DynInstPtr& first, const o3::DynInstPtr& second)
        : FusionInst(name, op, first, second)
    {
        panic_if(first->destRegIdx(0) != second->srcRegIdx(1),
                 "AluLoadFusionInst: first's dst must be the same as second's src");
        panic_if(second->destRegIdx(0) != second->srcRegIdx(0),
                 "AluLoadFusionInst: second's dst must be the same as its src");
        assert(second->operWid() / 8 == memsize);
        memAccessFlags = dynamic_cast<MemInst*>(second->staticInst.get())->getMemAccessFlags();
        setDestRegIdx(_numDestRegs++, second->destRegIdx(0)); _numTypedDestRegs[second->destRegIdx(0).classValue()]++;
        for (int i = 0; i < first->numSrcRegs(); ++i) {
            setSrcRegIdx(_numSrcRegs++, first->srcRegIdx(i));
        }
        firstNumSrcs = first->numSrcRegs();
        setSrcRegIdx(_numSrcRegs++, FuseTmpReg);

        flags = second->staticInst->getFlags();
    }

    Fault execute(ExecContext *xc, Trace::InstRecord *traceData) const override
    {
        panic("Not implemented yet");
    }

    Fault initiateAcc(ExecContext *xc, Trace::InstRecord *traceData) const override
    {
        assert(fused);

        first->renameDestReg(0, fused->extRenamedSrcIdx(firstNumSrcs), o3::VirtRegId());
        second->renameDestReg(0, fused->extRenamedDestIdx(0), o3::VirtRegId());

        // rename srcs
        for (int i = 0; i < numSrcRegs(); i++) {
            if (i < firstNumSrcs) {
                // first inst's src
                first->renameSrcReg(i, fused->extRenamedSrcIdx(i));
            } else {
                // second inst's src
                second->renameSrcReg(i - firstNumSrcs, fused->extRenamedSrcIdx(i));
            }
        }

        // calculate the address
        Fault fault = first->execute();
        if (fault != NoFault)
            return fault;
        uint64_t Rp1 = second->getRegOperand(this, 0);
        Addr EA = Rp1 + second->staticInst->getImm();
        return initiateMemReadSize<ExecContext, memsize>(xc, traceData, EA, memAccessFlags);
    }

    Fault completeAcc(PacketPtr pkt, ExecContext *, Trace::InstRecord *) const override
    {
        Fault fault = second->completeAcc(pkt);
        return fault;
    }
};

// slli rd_s, rs_val, shamt + add rd_a, rd_s, rs_base + ld rd_ld, offset(rd_a)
// EA = (rs_val << shamt) + rs_base + offset
// Sources: [rs_val, rs_base]  Dest: rd_ld
template<int memsize>
class SlliAddLoadFusionInst : public FusionInst
{
    const o3::DynInstPtr third;
    int shamt;
    Request::Flags memAccessFlags;
  public:
    SlliAddLoadFusionInst(const char *name, OpClass op,
                          const o3::DynInstPtr& first,
                          const o3::DynInstPtr& second,
                          const o3::DynInstPtr& third_inst)
        : FusionInst(name, op, first, second), third(third_inst)
    {
        panic_if(first->destRegIdx(0) != second->srcRegIdx(0) &&
                 first->destRegIdx(0) != second->srcRegIdx(1),
                 "SlliAddLoad: SLLI dest must feed ADD");
        panic_if(second->destRegIdx(0) != third->srcRegIdx(1),
                 "SlliAddLoad: ADD dest must be LD base");
        panic_if(third->destRegIdx(0) != third->srcRegIdx(0),
                 "SlliAddLoad: LD dest must equal LD src[0]");

        shamt = first->staticInst->getImm();
        memAccessFlags = dynamic_cast<MemInst*>(third->staticInst.get())->getMemAccessFlags();

        // Destination: LD's rd
        setDestRegIdx(_numDestRegs++, third->destRegIdx(0));
        _numTypedDestRegs[third->destRegIdx(0).classValue()]++;

        // src[0] = rs_val (SLLI's source)
        setSrcRegIdx(_numSrcRegs++, first->srcRegIdx(0));
        // src[1] = rs_base (ADD's non-SLLI source)
        int baseSrcIdx = (second->srcRegIdx(0) == first->destRegIdx(0)) ? 1 : 0;
        setSrcRegIdx(_numSrcRegs++, second->srcRegIdx(baseSrcIdx));

        flags = third->staticInst->getFlags();
    }

    Fault execute(ExecContext *xc, Trace::InstRecord *traceData) const override
    {
        panic("SlliAddLoad: execute() not implemented");
    }

    Fault initiateAcc(ExecContext *xc, Trace::InstRecord *traceData) const override
    {
        assert(fused);

        third->renameDestReg(0, fused->extRenamedDestIdx(0), o3::VirtRegId());

        uint64_t rs_val  = xc->getRegOperand(this, 0);
        uint64_t rs_base = xc->getRegOperand(this, 1);
        Addr EA = (rs_val << shamt) + rs_base + third->staticInst->getImm();

        return initiateMemReadSize<ExecContext, memsize>(xc, traceData, EA, memAccessFlags);
    }

    Fault completeAcc(PacketPtr pkt, ExecContext *, Trace::InstRecord *) const override
    {
        return third->completeAcc(pkt);
    }

    int numFusedParts() const override { return 3; }
};

// ld x1, offset(x2) + ld x3, offset + 8(x2)
template <int memsize>
class SeqLoadFusionInst : public FusionInst
{

    int base_offset = 0;
    int size0 = 0, size1 = 0;

    Request::Flags memAccessFlags;
  public:
    SeqLoadFusionInst(const char *name, OpClass op, const o3::DynInstPtr& first, const o3::DynInstPtr& second)
        : FusionInst(name, op, first, second)
    {
        panic_if(first->srcRegIdx(0) != second->srcRegIdx(0),
                 "SeqLoadFusionInst: first and second insts must have the same source register");
        panic_if(first->staticInst->getImm() + first->operWid() /8 != second->staticInst->getImm(),
                 "SeqLoadFusionInst: second inst's offset must be first inst's offset + load size");
        panic_if(dynamic_cast<MemInst*>(first->staticInst.get())->getMemAccessFlags() !=
                 dynamic_cast<MemInst*>(second->staticInst.get())->getMemAccessFlags(),
                    "SeqLoadFusionInst: first and second insts must have the same memory access flags");
        base_offset = first->staticInst->getImm();
        size0 = first->operWid() / 8;
        size1 = second->operWid() / 8;
        assert(memsize == size0 + size1);
        memAccessFlags = dynamic_cast<MemInst*>(first->staticInst.get())->getMemAccessFlags();

        setDestRegIdx(_numDestRegs++, first->destRegIdx(0)); _numTypedDestRegs[first->destRegIdx(0).classValue()]++;
        setDestRegIdx(_numDestRegs++, second->destRegIdx(0)); _numTypedDestRegs[second->destRegIdx(0).classValue()]++;
        setSrcRegIdx(_numSrcRegs++, first->srcRegIdx(0));

        flags = first->staticInst->getFlags() | second->staticInst->getFlags();
    }

    Fault execute(ExecContext *xc, Trace::InstRecord *traceData) const override
    {
        panic("Not implemented yet");
    }

    Fault initiateAcc(ExecContext *xc, Trace::InstRecord *traceData) const override
    {
        // rename
        first->renameDestReg(0, fused->extRenamedDestIdx(0), o3::VirtRegId());
        second->renameDestReg(0, fused->extRenamedDestIdx(1), o3::VirtRegId());
        // no need to rename src

        uint64_t Rp1 = xc->getRegOperand(this, 0);
        Addr EA = Rp1 + base_offset;
        return initiateMemReadSize<ExecContext, memsize>(xc, traceData, EA, memAccessFlags);
    }

    Fault completeAcc(PacketPtr pkt, ExecContext *, Trace::InstRecord *) const override
    {
        Packet tmp(pkt->getPtr<uint8_t>(), size0);
        Fault fault = first->completeAcc(&tmp);
        if (fault != NoFault)
            return fault;
        tmp.setPtr(pkt->getPtr<uint8_t>() + size0, size1);
        fault = second->completeAcc(&tmp);
        return fault;
    }

    bool correctMisalign(Addr addr) const override
    {
        bool align0 = addr % size0 == 0;
        bool align1 = (addr + size0) % size1 == 0;

        return align0 && align1;
    }
};

StaticInstPtr
chainFuseInsts(const char *name, const std::vector<o3::DynInstPtr> &vec,
               std::function<bool(const std::vector<o3::DynInstPtr> &)> checker)
{
    if (!checker(vec)) {
        return nullptr;  // cannot fuse
    }
    return new ChainFusionInst(name, vec[1]->opClass(), vec[0], vec[1]);
}

StaticInstPtr
alubrFuseInsts(const char *name, const std::vector<o3::DynInstPtr> &vec,
               std::function<bool(const std::vector<o3::DynInstPtr> &)> checker)
{
    if (!checker(vec)) {
        return nullptr;  // cannot fuse
    }
    return new AluBrFusionInst(name, vec[0], vec[1]);
}

StaticInstPtr
maccFuseInsts(const char *name, const std::vector<o3::DynInstPtr> &vec,
              std::function<bool(const std::vector<o3::DynInstPtr> &)> checker)
{
    if (!checker(vec)) {
        return nullptr;  // cannot fuse
    }
    // Use IntMultOp so the fused instruction is routed to multiply-capable FUs
    // (intIQ3/intIQ4) with the correct 3-cycle latency, not a plain 1-cycle ALU.
    return new MaccFusionInst(name, IntMultOp, vec[0], vec[1]);
}

template<int memsize>
StaticInstPtr
aluLoadFuseInsts(const char *name, const std::vector<o3::DynInstPtr> &vec,
               std::function<bool(const std::vector<o3::DynInstPtr> &)> checker)
{
    if (!checker(vec)) {
        return nullptr;  // cannot fuse
    }
    return new AluLoadFusionInst<memsize>(name, vec[1]->opClass(), vec[0], vec[1]);
}

template<int memsize>
StaticInstPtr
slliAddLoadFuseInsts(const char *name, const std::vector<o3::DynInstPtr> &vec,
                     std::function<bool(const std::vector<o3::DynInstPtr> &)> checker)
{
    if (!checker(vec)) return nullptr;
    return new SlliAddLoadFusionInst<memsize>(name, vec[2]->opClass(), vec[0], vec[1], vec[2]);
}

template<int memsize>
StaticInstPtr
seqLoadFuseInsts(const char *name, const std::vector<o3::DynInstPtr> &vec,
               std::function<bool(const std::vector<o3::DynInstPtr> &)> checker)
{
    if (!checker(vec)) {
        return nullptr;  // cannot fuse
    }
    return new SeqLoadFusionInst<memsize>(name, vec[1]->opClass(), vec[0], vec[1]);
}

const std::unordered_map<std::type_index, std::type_index> deCompressMap = {
    {typeid(RiscvISAInst::C_slli), typeid(RiscvISAInst::Slli)},
    {typeid(RiscvISAInst::C_srli), typeid(RiscvISAInst::Srli)},
    {typeid(RiscvISAInst::C_addi), typeid(RiscvISAInst::Addi)},
    {typeid(RiscvISAInst::C_addiw), typeid(RiscvISAInst::Addiw)},
    {typeid(RiscvISAInst::C_add), typeid(RiscvISAInst::Add)},
    {typeid(RiscvISAInst::C_addw), typeid(RiscvISAInst::Addw)},
    {typeid(RiscvISAInst::C_and), typeid(RiscvISAInst::And)},
    {typeid(RiscvISAInst::C_andi), typeid(RiscvISAInst::Andi)},
    {typeid(RiscvISAInst::C_or), typeid(RiscvISAInst::Or)},
    {typeid(RiscvISAInst::C_xor), typeid(RiscvISAInst::Xor)},
    {typeid(RiscvISAInst::C_mul), typeid(RiscvISAInst::Mul)},
    {typeid(RiscvISAInst::C_zext_h), typeid(RiscvISAInst::Zext_h)},
    {typeid(RiscvISAInst::C_sext_h), typeid(RiscvISAInst::Sext_h)},
    {typeid(RiscvISAInst::C_lui), typeid(RiscvISAInst::Lui)},
    {typeid(RiscvISAInst::C_ld), typeid(RiscvISAInst::Ld)},
    {typeid(RiscvISAInst::C_ldsp), typeid(RiscvISAInst::Ld)},
    {typeid(RiscvISAInst::C_lw), typeid(RiscvISAInst::Lw)},
    {typeid(RiscvISAInst::C_lwsp), typeid(RiscvISAInst::Lw)},
    {typeid(RiscvISAInst::C_lh), typeid(RiscvISAInst::Lh)},
    // unsigned load -> signed load
    {typeid(RiscvISAInst::Lwu), typeid(RiscvISAInst::Lw)},
    {typeid(RiscvISAInst::Lhu), typeid(RiscvISAInst::Lh)},
    {typeid(RiscvISAInst::Lbu), typeid(RiscvISAInst::Lb)},
    {typeid(RiscvISAInst::C_lhu), typeid(RiscvISAInst::Lh)},
    {typeid(RiscvISAInst::C_lbu), typeid(RiscvISAInst::Lb)},
};

#define ImmKey(t, i) FusionKey(typeid(RiscvISAInst::t), i)
#define AnyImmKey(t) FusionKey(typeid(RiscvISAInst::t))
#define ChainCreator(n, ...) [](const std::vector<o3::DynInstPtr>& vec) { return chainFuseInsts(n, std::move(vec), [](const std::vector<o3::DynInstPtr>& vec) { return __VA_ARGS__ ;}); }
#define AluBrCreator(n, ...) \
[](const std::vector<o3::DynInstPtr>& vec) { return alubrFuseInsts(n, std::move(vec), \
[](const std::vector<o3::DynInstPtr>& vec) { return __VA_ARGS__ ;}); }
#define MaccCreator(n, ...) [](const std::vector<o3::DynInstPtr>& vec) { return maccFuseInsts(n, std::move(vec), [](const std::vector<o3::DynInstPtr>& vec) { return __VA_ARGS__ ;}); }
#define AluLdCreator(n, s, ...) \
[](const std::vector<o3::DynInstPtr>& vec) { return aluLoadFuseInsts<s>(n, std::move(vec), \
[](const std::vector<o3::DynInstPtr>& vec) { return __VA_ARGS__ ;}); }
#define SeqLdCreator(n, s, ...) \
[](const std::vector<o3::DynInstPtr>& vec) { return seqLoadFuseInsts<s>(n, std::move(vec), \
[](const std::vector<o3::DynInstPtr>& vec) { return __VA_ARGS__ ;}); }
#define SlliAddLdCreator(n, s, ...) \
[](const std::vector<o3::DynInstPtr>& vec) { return slliAddLoadFuseInsts<s>(n, std::move(vec), \
[](const std::vector<o3::DynInstPtr>& vec) { return __VA_ARGS__ ;}); }

#define FirstDest0EqualSecond(a, b) (a->destRegIdx(0) == b->destRegIdx(0))
#define Dest0EqualSrc0(x) (x->destRegIdx(0) == x->srcRegIdx(0))
#define Dest0EqualSrc0or1(x) ((x->destRegIdx(0) == x->srcRegIdx(0)) || (x->destRegIdx(0) == x->srcRegIdx(1)))
#define Dest0EqualSecondSrc0or1(x, y) ((x->destRegIdx(0) == y->srcRegIdx(0)) || (x->destRegIdx(0) == y->srcRegIdx(1)))
#define ImmIs(a, i) (a->staticInst->getImm() == (i))
#define SeqLoadCheck() ((vec[0]->srcRegIdx(0) == vec[1]->srcRegIdx(0)) && (vec[0]->destRegIdx(0) != vec[0]->srcRegIdx(0)))

// make sure do not have the same key on different fusion tags
const FusionTag fusionMap = {
    // shift + alu
    {ImmKey(Slli, 32),
     new FusionTag{
         {ImmKey(Srli, 32), ChainCreator("low32", FirstDest0EqualSecond(vec[0], vec[1]) && Dest0EqualSrc0(vec[1]))},
         {ImmKey(Srli, 31), ChainCreator("sll1zext", FirstDest0EqualSecond(vec[0], vec[1]) && Dest0EqualSrc0(vec[1]))},
         {ImmKey(Srli, 30), ChainCreator("sll2zext", FirstDest0EqualSecond(vec[0], vec[1]) && Dest0EqualSrc0(vec[1]))},
         {ImmKey(Srli, 29), ChainCreator("sll3zext", FirstDest0EqualSecond(vec[0], vec[1]) && Dest0EqualSrc0(vec[1]))},
     }},
    {ImmKey(Slli, 48),
     new FusionTag{
         {ImmKey(Srli, 48), ChainCreator("low16", FirstDest0EqualSecond(vec[0], vec[1]) && Dest0EqualSrc0(vec[1]))},
     }},
    {ImmKey(Slliw, 16),
     new FusionTag{
         {ImmKey(Srliw, 16), ChainCreator("low16w", FirstDest0EqualSecond(vec[0], vec[1]) && Dest0EqualSrc0(vec[1]))},
         {ImmKey(Sraiw, 16), ChainCreator("sext16w", FirstDest0EqualSecond(vec[0], vec[1]) && Dest0EqualSrc0(vec[1]))},
     }},
    {ImmKey(Slli, 1),
     new FusionTag{
         {AnyImmKey(Add), ChainCreator("sll1add", FirstDest0EqualSecond(vec[0], vec[1]) && Dest0EqualSrc0or1(vec[1]))},
     }},
    {ImmKey(Slli, 2),
     new FusionTag{
         {AnyImmKey(Add), ChainCreator("sll2add", FirstDest0EqualSecond(vec[0], vec[1]) && Dest0EqualSrc0or1(vec[1]))},
     }},
    {ImmKey(Slli, 3),
     new FusionTag{
         {AnyImmKey(Add), ChainCreator("sll3add", FirstDest0EqualSecond(vec[0], vec[1]) && Dest0EqualSrc0or1(vec[1]))},
     }},
    {ImmKey(Slli, 4),
     new FusionTag{
         {AnyImmKey(Add), ChainCreator("sll4add", FirstDest0EqualSecond(vec[0], vec[1]) && Dest0EqualSrc0or1(vec[1]))},
     }},
    {ImmKey(Srli, 29),
     new FusionTag{
         {AnyImmKey(Add),
          ChainCreator("srl29add", FirstDest0EqualSecond(vec[0], vec[1]) && Dest0EqualSrc0or1(vec[1]))},
     }},
    {ImmKey(Srli, 30),
     new FusionTag{
         {AnyImmKey(Add),
          ChainCreator("srl30add", FirstDest0EqualSecond(vec[0], vec[1]) && Dest0EqualSrc0or1(vec[1]))},
     }},
    {ImmKey(Srli, 31),
     new FusionTag{
         {AnyImmKey(Add),
          ChainCreator("srl31add", FirstDest0EqualSecond(vec[0], vec[1]) && Dest0EqualSrc0or1(vec[1]))},
     }},
    {ImmKey(Srli, 32),
     new FusionTag{
         {AnyImmKey(Add),
          ChainCreator("srl32add", FirstDest0EqualSecond(vec[0], vec[1]) && Dest0EqualSrc0or1(vec[1]))},
     }},
    {ImmKey(Srli, 8),
     new FusionTag{
         {ImmKey(Andi, 0xff), ChainCreator("byte2", FirstDest0EqualSecond(vec[0], vec[1]) && Dest0EqualSrc0(vec[1]))},
     }},

    // add
    {AnyImmKey(Add),
     new FusionTag{
         {AnyImmKey(Ld), AluLdCreator("farld", 8, FirstDest0EqualSecond(vec[0], vec[1]) && Dest0EqualSrc0(vec[1]))},
         {AnyImmKey(Lw), AluLdCreator("farlw", 4, FirstDest0EqualSecond(vec[0], vec[1]) && Dest0EqualSrc0(vec[1]))},
         {AnyImmKey(Lh), AluLdCreator("farlh", 2, FirstDest0EqualSecond(vec[0], vec[1]) && Dest0EqualSrc0(vec[1]))},
         {AnyImmKey(Lb), AluLdCreator("farlb", 1, FirstDest0EqualSecond(vec[0], vec[1]) && Dest0EqualSrc0(vec[1]))},
     }},
    {AnyImmKey(Addw),
     new FusionTag{
         {ImmKey(Andi, 255),
          ChainCreator("addwbyte", FirstDest0EqualSecond(vec[0], vec[1]) && Dest0EqualSrc0(vec[1]))},
         {ImmKey(Andi, 1), ChainCreator("addwbit", FirstDest0EqualSecond(vec[0], vec[1]) && Dest0EqualSrc0(vec[1]))},
         {AnyImmKey(Zext_h),
          ChainCreator("addwzexth", FirstDest0EqualSecond(vec[0], vec[1]) && Dest0EqualSrc0(vec[1]))},
         {AnyImmKey(Sext_h),
          ChainCreator("addwsexth", FirstDest0EqualSecond(vec[0], vec[1]) && Dest0EqualSrc0(vec[1]))},
     }},
    {AnyImmKey(Addi),
     new FusionTag{
         {AnyImmKey(Ld), AluLdCreator("farld", 8, FirstDest0EqualSecond(vec[0], vec[1]) && Dest0EqualSrc0(vec[1]))},
         {AnyImmKey(Lw), AluLdCreator("farlw", 4, FirstDest0EqualSecond(vec[0], vec[1]) && Dest0EqualSrc0(vec[1]))},
         {AnyImmKey(Lh), AluLdCreator("farlh", 2, FirstDest0EqualSecond(vec[0], vec[1]) && Dest0EqualSrc0(vec[1]))},
         {AnyImmKey(Lb), AluLdCreator("farlb", 1, FirstDest0EqualSecond(vec[0], vec[1]) && Dest0EqualSrc0(vec[1]))},

         {AnyImmKey(Beq), AluBrCreator("addibeq", Dest0EqualSecondSrc0or1(vec[0], vec[1]))},
         {AnyImmKey(Bne), AluBrCreator("addibne", Dest0EqualSecondSrc0or1(vec[0], vec[1]))},
         {AnyImmKey(Blt), AluBrCreator("addiblt", Dest0EqualSecondSrc0or1(vec[0], vec[1]))},
         {AnyImmKey(Bge), AluBrCreator("addibge", Dest0EqualSecondSrc0or1(vec[0], vec[1]))},
         {AnyImmKey(Bltu), AluBrCreator("addibltu", Dest0EqualSecondSrc0or1(vec[0], vec[1]))},
         {AnyImmKey(Bgeu), AluBrCreator("addibgeu", Dest0EqualSecondSrc0or1(vec[0], vec[1]))},
     }},
    {AnyImmKey(Lui),
     new FusionTag{
         {AnyImmKey(Addi), ChainCreator("lui32", FirstDest0EqualSecond(vec[0], vec[1]) && Dest0EqualSrc0(vec[1]))},
         {AnyImmKey(Addiw), ChainCreator("luiw32", FirstDest0EqualSecond(vec[0], vec[1]) && Dest0EqualSrc0(vec[1]))},
     }},
    {AnyImmKey(Auipc),
     new FusionTag{
         {AnyImmKey(Ld), AluLdCreator("farld", 8, FirstDest0EqualSecond(vec[0], vec[1]) && Dest0EqualSrc0(vec[1]))},
         {AnyImmKey(Lw), AluLdCreator("farlw", 4, FirstDest0EqualSecond(vec[0], vec[1]) && Dest0EqualSrc0(vec[1]))},
         {AnyImmKey(Lh), AluLdCreator("farlh", 2, FirstDest0EqualSecond(vec[0], vec[1]) && Dest0EqualSrc0(vec[1]))},
         {AnyImmKey(Lb), AluLdCreator("farlb", 1, FirstDest0EqualSecond(vec[0], vec[1]) && Dest0EqualSrc0(vec[1]))},
     }},

    // multiply-accumulate: MUL/MULW result consumed by ADD/ADDW (any destination)
    {AnyImmKey(Mul),
     new FusionTag{
         {AnyImmKey(Add),  MaccCreator("muladd",  Dest0EqualSecondSrc0or1(vec[0], vec[1]))},
         {AnyImmKey(Addw), MaccCreator("muladdw", Dest0EqualSecondSrc0or1(vec[0], vec[1]))},
     }},
    {AnyImmKey(Mulw),
     new FusionTag{
         {AnyImmKey(Add),  MaccCreator("mulwadd",  Dest0EqualSecondSrc0or1(vec[0], vec[1]))},
         {AnyImmKey(Addw), MaccCreator("mulwaddw", Dest0EqualSecondSrc0or1(vec[0], vec[1]))},
     }},

    // logic
    {AnyImmKey(Andi),
     new FusionTag{
         {AnyImmKey(Add), ChainCreator("oddadd", ImmIs(vec[0], 1) && FirstDest0EqualSecond(vec[0], vec[1]) &&
                                                     Dest0EqualSrc0or1(vec[1]))},
         {AnyImmKey(Addw), ChainCreator("oddaddw", ImmIs(vec[0], 1) && FirstDest0EqualSecond(vec[0], vec[1]) &&
                                                       Dest0EqualSrc0or1(vec[1]))},
         {AnyImmKey(Or), ChainCreator("orh48", ImmIs(vec[0], -256) && FirstDest0EqualSecond(vec[0], vec[1]) &&
                                                   Dest0EqualSrc0or1(vec[1]))},
         {AnyImmKey(Mulw), ChainCreator("mulw7", ImmIs(vec[0], 127) && FirstDest0EqualSecond(vec[0], vec[1]) &&
                                                     Dest0EqualSrc0or1(vec[1]))},

         {ImmKey(Andi, 1), ChainCreator("andi1", FirstDest0EqualSecond(vec[0], vec[1]) && Dest0EqualSrc0(vec[1]))},
         {AnyImmKey(Zext_h), ChainCreator("andi16", FirstDest0EqualSecond(vec[0], vec[1]) && Dest0EqualSrc0(vec[1]))},

         {AnyImmKey(Beq), AluBrCreator("andibeq", Dest0EqualSecondSrc0or1(vec[0], vec[1]))},
         {AnyImmKey(Bne), AluBrCreator("andibne", Dest0EqualSecondSrc0or1(vec[0], vec[1]))},
         {AnyImmKey(Blt), AluBrCreator("andiblt", Dest0EqualSecondSrc0or1(vec[0], vec[1]))},
         {AnyImmKey(Bge), AluBrCreator("andibge", Dest0EqualSecondSrc0or1(vec[0], vec[1]))},
         {AnyImmKey(Bltu), AluBrCreator("andibltu", Dest0EqualSecondSrc0or1(vec[0], vec[1]))},
         {AnyImmKey(Bgeu), AluBrCreator("andibgeu", Dest0EqualSecondSrc0or1(vec[0], vec[1]))},
     }},
    {AnyImmKey(And),
     new FusionTag{
         {ImmKey(Andi, 1), ChainCreator("and1", FirstDest0EqualSecond(vec[0], vec[1]) && Dest0EqualSrc0(vec[1]))},
         {AnyImmKey(Zext_h), ChainCreator("and16", FirstDest0EqualSecond(vec[0], vec[1]) && Dest0EqualSrc0(vec[1]))},
     }},
    {AnyImmKey(Ori),
     new FusionTag{
         {ImmKey(Andi, 1), ChainCreator("ori1", FirstDest0EqualSecond(vec[0], vec[1]) && Dest0EqualSrc0(vec[1]))},
         {AnyImmKey(Zext_h), ChainCreator("ori16", FirstDest0EqualSecond(vec[0], vec[1]) && Dest0EqualSrc0(vec[1]))},
     }},
    {AnyImmKey(Or),
     new FusionTag{
         {ImmKey(Andi, 1), ChainCreator("or1", FirstDest0EqualSecond(vec[0], vec[1]) && Dest0EqualSrc0(vec[1]))},
         {AnyImmKey(Zext_h), ChainCreator("or16", FirstDest0EqualSecond(vec[0], vec[1]) && Dest0EqualSrc0(vec[1]))},
     }},
    {AnyImmKey(Xori),
     new FusionTag{
         {ImmKey(Andi, 1), ChainCreator("xori1", FirstDest0EqualSecond(vec[0], vec[1]) && Dest0EqualSrc0(vec[1]))},
         {AnyImmKey(Zext_h), ChainCreator("xori16", FirstDest0EqualSecond(vec[0], vec[1]) && Dest0EqualSrc0(vec[1]))},
     }},
    {AnyImmKey(Xor),
     new FusionTag{
         {ImmKey(Andi, 1), ChainCreator("xor1", FirstDest0EqualSecond(vec[0], vec[1]) && Dest0EqualSrc0(vec[1]))},
         {AnyImmKey(Zext_h), ChainCreator("xor16", FirstDest0EqualSecond(vec[0], vec[1]) && Dest0EqualSrc0(vec[1]))},
     }},
    {AnyImmKey(Orc_b),
     new FusionTag{
         {ImmKey(Andi, 1), ChainCreator("orcb1", FirstDest0EqualSecond(vec[0], vec[1]) && Dest0EqualSrc0(vec[1]))},
         {AnyImmKey(Zext_h), ChainCreator("orcb16", FirstDest0EqualSecond(vec[0], vec[1]) && Dest0EqualSrc0(vec[1]))},
     }},

    // alu-br creator
    // {AnyImmKey(Addi),
    //  new FusionTag{
    //    {AnyImmKey(Beq), AluBrCreator("addibeq", Dest0EqualSecondSrc0or1(vec[0], vec[1]))},
    //    {AnyImmKey(Bne), AluBrCreator("addibne", Dest0EqualSecondSrc0or1(vec[0], vec[1]))},
    //    {AnyImmKey(Blt), AluBrCreator("addiblt", Dest0EqualSecondSrc0or1(vec[0], vec[1]))},
    //    {AnyImmKey(Bge), AluBrCreator("addibge", Dest0EqualSecondSrc0or1(vec[0], vec[1]))},
    //    {AnyImmKey(Bltu), AluBrCreator("addibltu", Dest0EqualSecondSrc0or1(vec[0], vec[1]))},
    //    {AnyImmKey(Bgeu), AluBrCreator("addibgeu", Dest0EqualSecondSrc0or1(vec[0], vec[1]))},
    //  }},

    // {AnyImmKey(Andi),
    //  new FusionTag{
    //    {AnyImmKey(Beq), AluBrCreator("andibeq", Dest0EqualSecondSrc0or1(vec[0], vec[1]))},
    //    {AnyImmKey(Bne), AluBrCreator("andibne", Dest0EqualSecondSrc0or1(vec[0], vec[1]))},
    //    {AnyImmKey(Blt), AluBrCreator("andiblt", Dest0EqualSecondSrc0or1(vec[0], vec[1]))},
    //    {AnyImmKey(Bge), AluBrCreator("andibge", Dest0EqualSecondSrc0or1(vec[0], vec[1]))},
    //    {AnyImmKey(Bltu), AluBrCreator("andibltu", Dest0EqualSecondSrc0or1(vec[0], vec[1]))},
    //    {AnyImmKey(Bgeu), AluBrCreator("andibgeu", Dest0EqualSecondSrc0or1(vec[0], vec[1]))},
    //  }},

    {AnyImmKey(Srai),
     new FusionTag{
       {AnyImmKey(Beq), AluBrCreator("sraibeq", Dest0EqualSecondSrc0or1(vec[0], vec[1]))},
       {AnyImmKey(Bne), AluBrCreator("sraibne", Dest0EqualSecondSrc0or1(vec[0], vec[1]))},
       {AnyImmKey(Blt), AluBrCreator("sraiblt", Dest0EqualSecondSrc0or1(vec[0], vec[1]))},
       {AnyImmKey(Bge), AluBrCreator("sraibge", Dest0EqualSecondSrc0or1(vec[0], vec[1]))},
       {AnyImmKey(Bltu), AluBrCreator("sraibltu", Dest0EqualSecondSrc0or1(vec[0], vec[1]))},
       {AnyImmKey(Bgeu), AluBrCreator("sraibgeu", Dest0EqualSecondSrc0or1(vec[0], vec[1]))},
     }},

    {AnyImmKey(Slti),
     new FusionTag{
       {AnyImmKey(Beq), AluBrCreator("sltibeq", Dest0EqualSecondSrc0or1(vec[0], vec[1]))},
       {AnyImmKey(Bne), AluBrCreator("sltibne", Dest0EqualSecondSrc0or1(vec[0], vec[1]))},
       {AnyImmKey(Blt), AluBrCreator("sltiblt", Dest0EqualSecondSrc0or1(vec[0], vec[1]))},
       {AnyImmKey(Bge), AluBrCreator("sltibge", Dest0EqualSecondSrc0or1(vec[0], vec[1]))},
       {AnyImmKey(Bltu), AluBrCreator("sltibltu", Dest0EqualSecondSrc0or1(vec[0], vec[1]))},
       {AnyImmKey(Bgeu), AluBrCreator("sltibgeu", Dest0EqualSecondSrc0or1(vec[0], vec[1]))},
     }},

    // load
    {AnyImmKey(Ld), new FusionTag{
        {AnyImmKey(Ld), SeqLdCreator("ldld", 16, SeqLoadCheck() && ImmIs(vec[1], vec[0]->staticInst->getImm() + 8) )},
        {AnyImmKey(Lw), SeqLdCreator("ldlw", 12, SeqLoadCheck() && ImmIs(vec[1], vec[0]->staticInst->getImm() + 8) )},
        {AnyImmKey(Lh), SeqLdCreator("ldlh", 10, SeqLoadCheck() && ImmIs(vec[1], vec[0]->staticInst->getImm() + 8) )},
        {AnyImmKey(Lb), SeqLdCreator("ldlb", 9, SeqLoadCheck() && ImmIs(vec[1], vec[0]->staticInst->getImm() + 8) )},
    }},
    {AnyImmKey(Lw), new FusionTag{
        {AnyImmKey(Ld), SeqLdCreator("lwld", 12, SeqLoadCheck() && ImmIs(vec[1], vec[0]->staticInst->getImm() + 4) )},
        {AnyImmKey(Lw), SeqLdCreator("lwlw", 8, SeqLoadCheck() && ImmIs(vec[1], vec[0]->staticInst->getImm() + 4) )},
        {AnyImmKey(Lh), SeqLdCreator("lwlh", 6, SeqLoadCheck() && ImmIs(vec[1], vec[0]->staticInst->getImm() + 4) )},
        {AnyImmKey(Lb), SeqLdCreator("lwlb", 5, SeqLoadCheck() && ImmIs(vec[1], vec[0]->staticInst->getImm() + 4) )},
    }},
    {AnyImmKey(Lh), new FusionTag{
        {AnyImmKey(Ld), SeqLdCreator("lhld", 10, SeqLoadCheck() && ImmIs(vec[1], vec[0]->staticInst->getImm() + 2) )},
        {AnyImmKey(Lw), SeqLdCreator("lhlw", 6, SeqLoadCheck() && ImmIs(vec[1], vec[0]->staticInst->getImm() + 2) )},
        {AnyImmKey(Lh), SeqLdCreator("lhlh", 4, SeqLoadCheck() && ImmIs(vec[1], vec[0]->staticInst->getImm() + 2) )},
        {AnyImmKey(Lb), SeqLdCreator("lhlb", 3, SeqLoadCheck() && ImmIs(vec[1], vec[0]->staticInst->getImm() + 2) )},
    }},
    {AnyImmKey(Lb), new FusionTag{
        {AnyImmKey(Ld), SeqLdCreator("lbld", 9, SeqLoadCheck() && ImmIs(vec[1], vec[0]->staticInst->getImm() + 1) )},
        {AnyImmKey(Lw), SeqLdCreator("lblw", 5, SeqLoadCheck() && ImmIs(vec[1], vec[0]->staticInst->getImm() + 1) )},
        {AnyImmKey(Lh), SeqLdCreator("lblh", 3, SeqLoadCheck() && ImmIs(vec[1], vec[0]->staticInst->getImm() + 1) )},
        {AnyImmKey(Lb), SeqLdCreator("lblb", 2, SeqLoadCheck() && ImmIs(vec[1], vec[0]->staticInst->getImm() + 1) )},
    }},
};


// SLLI+ADD+LD/ST triple fusion: slli rd, rs, N + add rd2, rd, rb + ld rd3, off(rd2)
// Condition: SLLI dest feeds ADD, ADD dest is LD base register
#define SlliAddLdCheck() \
    ((vec[0]->destRegIdx(0) == vec[1]->srcRegIdx(0) || \
      vec[0]->destRegIdx(0) == vec[1]->srcRegIdx(1)) && \
     vec[1]->destRegIdx(0) == vec[2]->srcRegIdx(1) && \
     vec[2]->destRegIdx(0) == vec[2]->srcRegIdx(0))

#define SlliAddLdEntry(shamt) \
    {ImmKey(Slli, shamt), new FusionTag{ \
        {AnyImmKey(Add), new FusionTag{ \
            {AnyImmKey(Ld), SlliAddLdCreator("sll" #shamt "addld", 8, SlliAddLdCheck())}, \
            {AnyImmKey(Lw), SlliAddLdCreator("sll" #shamt "addlw", 4, SlliAddLdCheck())}, \
            {AnyImmKey(Lh), SlliAddLdCreator("sll" #shamt "addlh", 2, SlliAddLdCheck())}, \
            {AnyImmKey(Lb), SlliAddLdCreator("sll" #shamt "addlb", 1, SlliAddLdCheck())}, \
        }}, \
    }}

const FusionTag fusionMap3 = {
    SlliAddLdEntry(1),
    SlliAddLdEntry(2),
    SlliAddLdEntry(3),
    SlliAddLdEntry(4),
    SlliAddLdEntry(5),
    SlliAddLdEntry(6),
    SlliAddLdEntry(7),
    SlliAddLdEntry(8),
};

}
}
