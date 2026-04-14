

// std::vector<BTBTAGE::StatisticalCorrector::SCPrediction>
// BTBTAGE::StatisticalCorrector::getPredictions(Addr pc, std::vector<TagePrediction> &tagePreds)
// {
//     std::vector<int> scSums = {0,0};
//     std::vector<int> tageCtrCentereds;
//     std::vector<SCPrediction> scPreds;
//     std::vector<bool> sumAboveThresholds;
//     scPreds.resize(numBr);
//     sumAboveThresholds.resize(numBr);
//     for (int b = 0; b < numBr; b++) {
//         int phyBrIdx = tage->getShuffledBrIndex(pc, b);
//         std::vector<int> scOldCounters;
//         tageCtrCentereds.push_back((2 * tagePreds[b].mainCounter + 1) * 8);
//         for (int i = 0;i < scCntTable.size();i++) {
//             int index = getIndex(pc, i);
//             int tOrNt = tagePreds[b].taken ? 1 : 0;
//             int ctr = scCntTable[i][index][phyBrIdx][tOrNt];
//             scSums[b] += 2 * ctr + 1;
//         }
//         scSums[b] += tageCtrCentereds[b];
//         sumAboveThresholds[b] = abs(scSums[b]) > thresholds[b];

//         scPreds[b].tageTaken = tagePreds[b].taken;
//         scPreds[b].scUsed = tagePreds[b].mainFound;
//         scPreds[b].scPred = tagePreds[b].mainFound && sumAboveThresholds[b] ?
//             scSums[b] >= 0 : tagePreds[b].taken;
//         scPreds[b].scSum = scSums[b];

//         // stats
//         auto &stat = stats[b];
//         if (tagePreds[b].mainFound) {
//             stat->scUsedAtPred++;
//             if (sumAboveThresholds[b]) {
//                 stat->scConfAtPred++;
//                 if (scPreds[b].scPred == scPreds[b].tageTaken) {
//                     stat->scAgreeAtPred++;
//                 } else {
//                     stat->scDisagreeAtPred++;
//                 }
//             } else {
//                 stat->scUnconfAtPred++;
//             }
//         }
//     }
//     return scPreds;
// }

// std::vector<FoldedHist>
// BTBTAGE::StatisticalCorrector::getFoldedHist()
// {
//     return foldedHist;
// }

// bool
// BTBTAGE::StatisticalCorrector::satPos(int &counter, int counterBits)
// {
//     return counter == ((1 << (counterBits-1)) - 1);
// }

// bool
// BTBTAGE::StatisticalCorrector::satNeg(int &counter, int counterBits)
// {
//     return counter == -(1 << (counterBits-1));
// }

// Addr
// BTBTAGE::StatisticalCorrector::getIndex(Addr pc, int t)
// {
//     return getIndex(pc, t, foldedHist[t].get());
// }

// Addr
// BTBTAGE::StatisticalCorrector::getIndex(Addr pc, int t, bitset &foldedHist)
// {
//     bitset buf(tableIndexBits[t], pc >> tablePcShifts[t]);  // lower bits of PC
//     buf ^= foldedHist;
//     return buf.to_ulong();
// }

// void
// BTBTAGE::StatisticalCorrector::counterUpdate(int &ctr, int nbits, bool taken)
// {
//     if (taken) {
// 		if (ctr < ((1 << (nbits-1)) - 1))
// 			ctr++;
// 	} else {
// 		if (ctr > -(1 << (nbits-1)))
// 			ctr--;
//     }
// }

// void
// BTBTAGE::StatisticalCorrector::update(Addr pc, SCMeta meta, std::vector<bool> needToUpdates,
//     std::vector<bool> actualTakens)
// {
//     auto predHist = meta.indexFoldedHist;
//     auto preds = meta.scPreds;

//     for (int b = 0; b < numBr; b++) {
//         if (!needToUpdates[b]) {
//             continue;
//         }
//         int phyBrIdx = tage->getShuffledBrIndex(pc, b);
//         auto &p = preds[b];
//         bool scTaken = p.scPred;
//         bool actualTaken = actualTakens[b];
//         int tOrNt = p.tageTaken ? 1 : 0;
//         int sumAbs = std::abs(p.scSum);
//         // perceptron update
//         if (p.scUsed) {
//             if (sumAbs <= (thresholds[b] * 8 + 21) || scTaken != actualTaken) {
//                 for (int i = 0; i < numPredictors; i++) {
//                     auto idx = getIndex(pc, i, predHist[i].get());
//                     auto &ctr = scCntTable[i][idx][phyBrIdx][tOrNt];
//                     counterUpdate(ctr, scCounterWidth, actualTaken);
//                 }
//                 if (scTaken != actualTaken) {
//                     stats[b]->scUpdateOnMispred++;
//                 } else {
//                     stats[b]->scUpdateOnUnconf++;
//                 }
//             }

//             if (scTaken != p.tageTaken && sumAbs >= thresholds[b] - 4 && sumAbs <= thresholds[b] - 2) {

//                 bool cause = scTaken != actualTaken;
//                 counterUpdate(TCs[b], TCWidth, cause);
//                 if (satPos(TCs[b], TCWidth) && thresholds[b] <= maxThres) {
//                     thresholds[b] += 2;
//                 } else if (satNeg(TCs[b], TCWidth) && thresholds[b] >= minThres) {
//                     thresholds[b] -= 2;
//                 }

//                 if (satPos(TCs[b], TCWidth) || satNeg(TCs[b], TCWidth)) {
//                     TCs[b] = neutralVal;
//                 }
//             }

//             // stats
//             auto &stat = stats[b];
//             // bool sumAboveUpdateThreshold = sumAbs >= (thresholds[b] * 8 + 21);
//             bool sumAboveUseThreshold = sumAbs >= thresholds[b];
//             stat->scUsedAtCommit++;
//             if (sumAboveUseThreshold) {
//                 stat->scConfAtCommit++;
//                 if (scTaken == p.tageTaken) {
//                     stat->scAgreeAtCommit++;
//                 } else {
//                     stat->scDisagreeAtCommit++;
//                     if (scTaken == actualTaken) {
//                         stat->scCorrectTageWrong++;
//                     } else {
//                         stat->scWrongTageCorrect++;
//                     }
//                 }
//             } else {
//                 stat->scUnconfAtCommit++;
//             }
//         }

//     }
// }

// void
// BTBTAGE::StatisticalCorrector::recoverHist(std::vector<FoldedHist> &fh)
// {
//     for (int i = 0; i < numPredictors; i++) {
//         foldedHist[i].recover(fh[i]);
//     }
// }

// void
// BTBTAGE::StatisticalCorrector::doUpdateHist(const boost::dynamic_bitset<> &history,
//     int shamt, bool cond_taken)
// {
//     if (shamt == 0) {
//         return;
//     }
//     for (int t = 0; t < numPredictors; t++) {
//         foldedHist[t].update(history, shamt, cond_taken);
//     }
// }