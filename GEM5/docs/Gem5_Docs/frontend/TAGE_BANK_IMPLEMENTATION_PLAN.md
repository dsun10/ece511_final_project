# TAGE Bank机制实现计划

## 状态：✅ 已完成（2025-11-19）

**实施总结**：
- ✅ 所有5个阶段已完成
- ✅ 编译成功，测试通过
- ✅ Bank冲突检测正常工作
- ✅ 统计数据正确记录

**关键决策**：
1. **调用顺序**：确认为先`update()`后`putPCHistory()`（同一tick内）
2. **清空策略**：采用方案2 - 在`update()`结束时清空`predBankValid`
3. **参数化实现**：完全参数化，避免硬编码
4. **表容量**：保持不变，4个bank共享tableSizes

---

## 目标
在gem5 TAGE预测器中实现与XiangShan RTL一致的Bank机制，模拟硬件中的bank冲突行为。

## 设计决策

### 1. Bank分区方案
以 XiangShan 语义对齐：先去掉 `instShiftAmt`（目前为 1）个指令粒度位，再取随后的 `ceilLog2(numBanks)` 位作为 bank id，剩余更高位与 folded history 组合得到 index / tag。这样可以直接利用 `startPC` 的最低有效位，默认只屏蔽半字节对齐，并允许在未来按需扩展 bank 颗粒度。

### 2. 周期模型（已确认实际调用顺序）✅
- 一个"周期"内：**先调用 `update(块B)` 进行更新，后调用 `putPCHistory(块A)` 进行预测**
  ```
  Fetch::tick() {
    1. initializeTickState() -> update(块B)      // 更新上一次的预测
    2. fetchAndProcessInstructions()
    3. updateBranchPredictors() -> putPCHistory(块A)  // 进行新预测
  }
  ```
- 如果更新块B和预测块A的bank_id相同，则认为发生bank冲突
- 冲突检测时机：在`update()`开始时检测，如果与上一次的预测bank冲突，则丢弃本次更新
- 简化版本：冲突时**直接丢弃更新**（模拟性能下限）

### 2.1 predBankValid清空策略（方案2）✅
采用"在update结束时清空"的策略：
```cpp
void update() {
    // 检测冲突
    if (predBankValid && updateBank == lastPredBankId) {
        // 发生冲突，丢弃更新
        predBankValid = false;
        return;
    }

    // 正常更新逻辑...

    // 结束时清空，表示预测已被消费
    predBankValid = false;
}

void putPCHistory() {
    // 记录新预测
    lastPredBankId = getBankId(alignedPC);
    predBankValid = true;
}
```
**设计理由**：
- 保证预测状态只被消费一次
- 简单且符合"预测-更新"语义
- 用户保证一个tick内只调用一次update()

### 3. 数据结构
- **保持三维数组**：`tageTable[table][index][way]`
- **不改为四维**：在 index/tag 计算时剥离 bank 比特即可
- bank 信息通过 `bankBaseShift`/`bankIdWidth` 描述，内联函数统一取用

---

## 修改清单

### 调整记录（概要）
- `btb_tage.hh` 中新增 `blockWidth` 与 `bankBaseShift`，前者仍等于 `floorLog2(blockSize)`，后者默认为 `instShiftAmt`，用于剥离指令对齐位；`indexShift` 计算依赖上述两个常量。
- `getBankId()`/`getTageIndex()`/`getTageTag()` 等 helper 统一使用这些新偏移，确保预测与更新都以同样的 `startPC` 语义工作；bank 位只在启用冲突模拟时跳过，其余情况下维持 legacy 行为。
- `putPCHistory()`/`update()` 记录 bank id 时直接基于 `startPC`，减少额外对齐开销，统计项维持不变。

---

## 潜在问题与解决方案

### 问题1: Index空间变化
**问题**：从 `pc >> 5` 改为 `pc >> 7` 后，index的熵减少了
- 原来：pc的每2bit（因为右移5后，相邻指令差2）影响index
- 现在：pc的每32B（因为跳过bank位）影响index

**影响**：
- Index分布可能变得不均匀
- 冲突率可能增加

**解决**：这是预期行为，RTL也是这样做的。通过bank分区，实际上总容量不变。

### 问题2: FoldedHist需要调整吗？
**答**：不需要。FoldedHist是history的折叠，与PC的bit选择无关。

### 问题3: 是否需要清空predBankValid？
**已采用方案2（在update结束时清空）**：
- 每次预测时设置 `predBankValid = true`
- 每次update结束时设置 `predBankValid = false`（无论是否冲突）
- 保证预测状态只被消费一次，符合"预测-更新"语义

**设计理由**：
- ✅ 简单且正确：一个预测对应一次更新
- ✅ 避免误报：不会和很久之前的预测冲突
- ✅ 依赖保证：用户保证一个tick内只调用一次update()

---

## 未来优化方向（阶段6+）

### 开窗机制
```cpp
class BTBTAGE {
private:
    unsigned updateBlockedCount[4];  // 每个bank的阻塞计数
    static constexpr unsigned WINDOW_THRESHOLD = 8;
    int forceUpdateBank;  // -1表示无强制，0-3表示强制该bank的更新

    void update() {
        if (bankConflict) {
            updateBlockedCount[updateBank]++;
            if (updateBlockedCount[updateBank] >= WINDOW_THRESHOLD) {
                forceUpdateBank = updateBank;
                updateBlockedCount[updateBank] = 0;
            }
            return;
        }
        // ... normal update, clear counter ...
        updateBlockedCount[updateBank] = 0;
    }

    void putPCHistory() {
        if (forceUpdateBank >= 0 && getBankId(alignedPC) == forceUpdateBank) {
            // Block this prediction, force update first
            // HOW TO IMPLEMENT? Need to coordinate with BPU top level
        }
    }
}
```
