    
// class StatisticalCorrector {
//       public:
//         // TODO: parameterize
//         StatisticalCorrector(BTBTAGE *tage) : tage(tage) {
//           scCntTable.resize(numPredictors);
//           tableIndexBits.resize(numPredictors);
//           for (int i = 0; i < numPredictors; i++) {
//             tableIndexBits[i] = ceilLog2(tableSizes[i]);
//             foldedHist.push_back(FoldedHist(histLens[i], tableIndexBits[i], numBr));
//             scCntTable[i].resize(tableSizes[i]);
//             for (auto &br_counters : scCntTable[i]) {
//               br_counters.resize(numBr);
//               for (auto &tOrNt : br_counters) {
//                 tOrNt.resize(2, 0);
//               }
//             }
//           }
//           // initial theshold
//           thresholds.resize(numBr, 6);
//           TCs.resize(numBr, neutralVal);
//         };

//         typedef struct SCPrediction {
//             bool tageTaken;
//             bool scUsed;
//             bool scPred;
//             int scSum;
//             SCPrediction() : tageTaken(false), scUsed(false), scPred(false), scSum(0) {}
//         } SCPrediction;

//         typedef struct SCMeta {
//             std::vector<FoldedHist> indexFoldedHist;
//             std::vector<SCPrediction> scPreds;
//         } SCMeta;

//       public:
//         Addr getIndex(Addr pc, int t);

//         Addr getIndex(Addr pc, int t, bitset &foldedHist);

//         std::vector<FoldedHist> getFoldedHist();

//         std::vector<SCPrediction> getPredictions(Addr pc, std::vector<TagePrediction> &tagePreds);

//         void update(Addr pc, SCMeta meta, std::vector<bool> needToUpdates, std::vector<bool> actualTakens);

//         void recoverHist(std::vector<FoldedHist> &fh);

//         void doUpdateHist(const boost::dynamic_bitset<> &history, int shamt, bool cond_taken);

//         void setStats(std::vector<TageBankStats *> stats) {
//           this->stats = stats;
//         }

//       private:
//         int numBr;

//         BTBTAGE *tage;

//         int numPredictors = 4;

//         int scCounterWidth = 6;

//         std::vector<int> thresholds;

//         int minThres = 5;

//         int maxThres = 31;

//         std::vector<int> TCs;

//         int TCWidth = 6;

//         int neutralVal = 0;

//         std::vector<FoldedHist> foldedHist;

//         std::vector<int> tableIndexBits;

//         // std::vector<bool> tagVec;

//         // table - table index - numBr - taken/not taken
//         std::vector<std::vector<std::vector<std::vector<int>>>> scCntTable;

//         std::vector<unsigned> histLens {0, 4, 10, 16};

//         std::vector<int> tableSizes {256, 256, 256, 256};

//         std::vector<int> tablePcShifts {1, 1, 1, 1};

//         bool satPos(int &counter, int counterBits);

//         bool satNeg(int &counter, int counterBits);

//         void counterUpdate(int &ctr, int nbits, bool taken);

//         std::vector<TageBankStats*> stats;

//     };

    // StatisticalCorrector sc;