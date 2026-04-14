## 📄 文档：TAGE 块预测的“重读-更新”方案

### 1\. 目标

在 TAGE 块预测器中，实现一种\*\*不依赖流水线元数据（Metadata）\*\*的更新机制。

本方案（方案三）旨在通过“更新时重读”的方式，在硬件成本（避免 N 套并行更新逻辑）和学习效率（不丢失信息）之间达到最佳平衡。

**核心思想：** 预测器**只更新到第一个控制流变更点**（即第一个错误预测分支，或第一个正确预测的 Taken 分支）。
模型中对应exeBranchInfo, 代表执行时候为taken 的那条分支

### 2\. 方案逻辑

1.  **为什么只更新到“停止点”？**

      * 在一个 Fetch Block（例如 B0, B1, B2, B3）中，如果 B1 是第一个 Taken 分支，那么 B2 和 B3 根本不会被执行。更新它们是无效的，甚至是有害的（用错误的数据污染 TAGE 表）。
      * 同理，如果 B1 是第一个*错误预测*的分支（例如，预测 NT，实际 T），那么流水线在 B1 处就会被冲刷（flush），B2 和 B3 也不会被执行。
      * **结论：** 我们只需要（也必须）更新所有*实际被执行并提交*的分支。这个集合就是从 B0 到“停止点”的所有分支。

2.  **为什么这个方案是高效的？**

      * **信息最大化：** 它正确地强化了所有被正确预测的分支（包括 Not-Taken 的 B0, B1）并惩罚/纠正了“停止点”分支（B2）。
      * **成本可控：** 相比于“更新所有 N 个分支”的方案（需要 N 套并行的更新逻辑），本方案的硬件复杂度更低。虽然在最坏情况下（B7 是停止点）仍需要 N 套逻辑，但在平均情况下（第一个 Taken 在 B1 或 B2），硬件可以被流水化（pipelined）或部分激活（partially activated）。

### 3\. 实现：更新阶段算法

在你的模型中，当一个 Fetch Block 到达 Commit 阶段并触发更新时，你需要以下输入：

#### 必需的输入数据

1.  `startAddr`：该 Fetch Block 的起始 PC。
2.  `GHR`：**预测时**所使用的全局历史寄存器（GHR）, 当前模型使用PHR，等效的，
    真正使用的是meta->indexFoldedHist[ti].get()， 也就是折叠后的PHR生成的Index, 对应预测时候index.
(后两个模型中不存在，但可以暂时通过prepareUpdateEntries 来获取要更新的哪些分支和对应的跳转反向)
3.  `ActualResults[N]`：一个数组或位掩码，包含 B0 到 B(N-1) 的**真实**执行结果（T/NT）。
4.  `StopPoint`：一个整数，标记**停止点**的分支索引。
      * **情况 A（预测错误）：** `StopPoint` = 第一个错误预测的分支索引。
      * **情况 B（预测正确）：** `StopPoint` = 第一个 Taken 分支的索引。（如果全部 NT，`StopPoint` = 最后一个分支的索引）。

#### 步骤 1：重读（Re-Read）所有命中表

这是“重读”的核心。

1.  使用 `startAddr` 和 `GHR`，为**所有** TAGE 表（$T_1 \dots T_N$）计算它们各自的 `index` 和 `tag`。
2.  模拟SRAM并行读取：访问所有表，获取所有（`index` 命中）的表项。
3.  **构建总命中集（`HitSet_Total`）**：
      * 遍历所有读出的表项，只保留那些 `valid` 且 `tag` 匹配的表项。
      * `HitSet_Total` 是一个包含所有 {表号, 表项数据, ...} 的列表。

#### 步骤 2：循环更新（从 B0 到 StopPoint）

这是本方案的关键逻辑。你将模拟“分组-再排序”的预测过程，并立即将其与真实结果比较。

```plaintext
// 伪代码
function OnUpdate(startAddr, GHR, ActualResults, StopPoint):
    
    // 步骤 1: 拿到所有命中的表项
    HitSet_Total = Re_Read_All_Tables(startAddr, GHR)

    // 步骤 2: 循环更新到“停止点”
    for i from 0 to StopPoint:
        
        // A. 为 B[i] 找出 main 和 alt
        //    (这模拟了 "按 Position 分组")
        HitSet_B_i = Filter_By_Position(HitSet_Total, position=i)
        
        (main_entry, alt_entry) = Find_Main_And_Alt(HitSet_B_i) 
        // main_entry 是 HitSet_B_i 中表号最高的
        // alt_entry  是 HitSet_B_i 中表号次高的

        // B. 找出 B[i] 的原始预测
        predicted_dir = Get_Prediction_From(main_entry) 
        // 如果 main_entry 为空, 则 Get_Prediction_From(BasePredictor)

        // C. 拿到 B[i] 的真实结果
        actual_dir = ActualResults[i]

        // D. 调用标准的 TAGE 更新函数
        //    这个函数处理所有 ctr, u-bit, 和分配逻辑
        TAGE_Update_Single_Branch(
            main_entry,
            alt_entry,
            predicted_dir,
            actual_dir,
            branch_pc = startAddr + (i * branch_stride), // B[i]的PC
            ghr = GHR,
            new_entry_position = i  // 关键！
        )
```

#### 步骤 3：`TAGE_Update_Single_Branch` 的实现细节

这个函数是 TAGE 算法的心脏。当它被调用时，它只关心*一个*分支。

1.  **ctr 更新**：
      * 如果 `predicted_dir == actual_dir`：强化（增加）`main_entry` 的 `ctr`。
      * 如果 `predicted_dir != actual_dir`：削弱（减少）`main_entry` 的 `ctr`。
2.  **u-bit 更新**：
      * *仅当* `main_entry` 预测错误而 `alt_entry` 预测**正确**时，才增加 `alt_entry` 的 `u` bit。
      * （或根据你的 TAGE 变种）`main` 对而 `alt` 错时，增加 `main` 的 `u` bit。
3.  **分配（Allocation）逻辑**：
      * *仅当* `predicted_dir != actual_dir` **并且**（`main` 预测错了，`alt` 也预测错了，或者 `alt` 不存在）时触发。
      * 尝试在高于 `main_entry` 的表（例如 $T_{main+1} \dots T_N$）中寻找一个 `u=0` 的槽位。
      * **关键**：当你写入这个新分配的槽位时，你**必须**写入 `pos` 字段：
          * `New_Entry.pos = new_entry_position` （即 `i`）
          * `New_Entry.ctr = ...` (弱 Taken/NT)
          * `New_Entry.tag = ...` (根据 `startAddr` 和 `GHR` 计算)
          * `New_Entry.u = 0`

### 4\. 关键实现考量

1.  **GHR 的一致性**：`OnUpdate` 函数中用于“重读”和“分配”的 `GHR`，必须与 `OnPredict` 使用的 `GHR` 完全一致。
2.  **分配冲突（Allocation Conflicts）**：
      * **问题**：如果在同一个更新周期，B0 和 B1 都预测错误，并且都尝试分配到 $T_7$ 的同一个 `index`，会发生什么？
      * **模型中的处理**：你的 `for` 循环（`i from 0 to StopPoint`）天然地给了**位置靠前**的分支（如 B0）以**优先权**。B0 会先调用 `TAGE_Update_Single_Branch`，并可能占据 `T7` 的一个槽位。当 B1 再调用时，它可能会发现槽位已被 B0 占用。
      * **硬件现实**：这再次证明了**N路组相连**（Set-Associativity）的必要性。在 N-Way TAGE 中，B0 和 B1 可以和平共存，各自占据同一 `Set` (index) 中的不同 `Way`。在你的模型中，你可以暂时使用简单的“前者优先”规则。