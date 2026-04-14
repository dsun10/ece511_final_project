#ifndef __CPU_PRED_BTB_TEST_MOCK_PCSTATE_HH__
#define __CPU_PRED_BTB_TEST_MOCK_PCSTATE_HH__

#include <boost/dynamic_bitset.hpp>

#include "base/types.hh"

// #include "cpu/static_inst.hh"
namespace gem5
{
namespace branch_prediction
{
namespace btb_pred
{
using bitset = boost::dynamic_bitset<>;
/**
 * @brief Mock PCState for testing
 */
class PCStateBase
{
    Addr _pc = 0;
    MicroPC _upc = 0;
public:
    PCStateBase(Addr pc = 0) { _pc = pc; }
    Addr instAddr() const { return _pc; }
    Addr pc() const { return _pc; }
    void setPC(Addr pc) { _pc = pc; }

    // Implementation of abstract methods
    PCStateBase *clone() const { return new PCStateBase(_pc); }
    void output(std::ostream &os) const { os << "PC:" << _pc; }
    void advance() { _pc += 4; }
    bool branching() const { return false; }
};


/**
 * @brief Mock StaticInst for testing
 */
class MockStaticInst
{
public:
    enum Flags
    {
        IsNop = 0,
        IsControl,
        IsCondControl,
        IsIndirectControl,
        IsCall,
        IsReturn,
        IsUncondControl,
        NumFlags
    };

private:
    std::string _name;
    std::bitset<NumFlags> _flags;

public:
    MockStaticInst(const std::string &name = "mock") : _name(name) {}

    void setFlag(Flags flag) { _flags.set(flag); }
    bool isFlag(Flags flag) const { return _flags.test(flag); }

    bool isControl() const { return isFlag(IsControl); }
    bool isCondCtrl() const { return isFlag(IsCondControl); }
    bool isIndirectCtrl() const { return isFlag(IsIndirectControl); }
    bool isCall() const { return isFlag(IsCall); }
    bool isReturn() const { return isFlag(IsReturn); }
    bool isUncondCtrl() const { return isFlag(IsUncondControl); }

    void advancePC(PCStateBase &pc_state) const {
        pc_state.advance();
    }

    std::string generateDisassembly(Addr pc) const {
        return _name;
    }
};

using StaticInstPtr = std::shared_ptr<MockStaticInst>;

} // namespace btb_pred
} // namespace branch_prediction
} // namespace gem5

#endif // __CPU_PRED_BTB_TEST_MOCK_PCSTATE_HH__
