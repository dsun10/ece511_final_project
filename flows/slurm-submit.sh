#!/usr/bin/env bash
# Submit gem5 SPEC2006 simpoint simulations to SLURM.
# Usage: ./slurm-submit.sh [fuse|nofuse]  (default: nofuse)
#
# Each checkpoint becomes an independent SLURM job.  Results land under
#   $RESULT_DIR/<bench>/<cp_num>/<fuse|nofuse>/

set -euo pipefail

CONFIG_MODE="${1:-nofuse}"

# --- Paths ---
GEM5_ROOT="/research/dsun19/ece511_final_project/GEM5"
GEM5_BIN="$GEM5_ROOT/build/RISCV/gem5.fast"
DIFFTEST="/storage/pradyun/xiangshan/difftest_NEMU/build/riscv64-nemu-interpreter-so"
CPT_ROOT="/fast-lab-share/tw12/spec2006_simpoints/sim_checkpoint_results/spec-cpt"
RESULT_DIR="/research/dsun19/ece511_final_project/sim_results/spec06"
LOG_DIR="$RESULT_DIR/slurm_logs"

case "$CONFIG_MODE" in
    fuse)   CONFIG_PY="$GEM5_ROOT/configs/example/fusion_config.py" ;;
    nofuse) CONFIG_PY="$GEM5_ROOT/configs/example/kmhv3.py" ;;
    *)      echo "Usage: $0 [fuse|nofuse]" >&2; exit 1 ;;
esac

mkdir -p "$LOG_DIR"

# --- Checkpoint list (bench | cp_num) ---
CHECKPOINTS=(
    "400.perlbench_diffmail    | 7161"
    "403.gcc_200               | 536"
    "429.mcf                   | 6351"
    "447.dealII                | 18272"
    "473.astar_biglake         | 1504"
    #"400.perlbench_checkspam   | 19683"
    #"400.perlbench_splitmail   | 21165"
    #"401.bzip2_chicken         | 2919"
    #"401.bzip2_inputcombined   | 12217"
    #"401.bzip2_inputprogram    | 16624"
    #"401.bzip2_inputsource     | 16155"
    #"401.bzip2_liberty         | 4520"
    #"401.bzip2_texthtml        | 3848"
    #"403.gcc_166               | 1765"
    #"403.gcc_cpdecl            | 1953"
    #"403.gcc_ctypeck           | 774"
    #"403.gcc_expr              | 1450"
    #"403.gcc_expr2             | 2746"
    #"403.gcc_g23               | 3378"
    #"403.gcc_s04               | 1251"
    #"403.gcc_scilab            | 852"
    #"410.bwaves                | 15669"
    #"416.gamess_cytosine       | 33659"
    #"416.gamess_h2ocu2         | 36715"
    #"416.gamess_triazolium     | 164120"
    #"433.milc                  | 25298"
    #"434.zeusmp                | 7684"
    #"435.gromacs               | 3658"
    #"436.cactusADM             | 47083"
    #"437.leslie3d              | 17599"
    #"444.namd                  | 56650"
    #"445.gobmk_1313            | 372"
    #"445.gobmk_nngs            | 2579"
    #"445.gobmk_score2          | 12661"
    #"445.gobmk_trevorc         | 5547"
    #"445.gobmk_trevord         | 7288"
    #"450.soplex_pds            | 7234"
    #"450.soplex_ref            | 6083"
    #"453.povray                | 22449"
    #"454.calculix              | 92948"
    #"456.hmmer_nph3            | 5026"
    #"456.hmmer_retro           | 100657"
    #"458.sjeng                 | 98914"
    #"459.GemsFDTD              | 42779"
    #"462.libquantum            | 85224"
    #"464.h264ref_foremanbase   | 3244"
    #"464.h264ref_foremanmain   | 18426"
    #"464.h264ref_sssmain       | 71347"
    #"465.tonto                 | 3279"
    #"470.lbm                   | 6767"
    #"471.omnetpp               | 3353"
    #"473.astar_rivers          | 8943"
    #"481.wrf                   | 31374"
    #"482.sphinx3               | 113583"
    #"483.xalancbmk             | 10869"
)

# --- Submit one job per checkpoint ---
submitted=0
skipped=0

for entry in "${CHECKPOINTS[@]}"; do
    bench="${entry%|*}"; bench="${bench// /}"   # trim spaces
    cp_num="${entry##*|}";  cp_num="${cp_num// /}"

    cp_path="$CPT_ROOT/$bench/$cp_num"
    if [ ! -d "$cp_path" ]; then
        echo "[skip] checkpoint dir not found: $cp_path"
        (( skipped++ )) || true
        continue
    fi

    cpt_file=$(ls "$cp_path"/*.zstd 2>/dev/null | head -n 1)
    if [ -z "$cpt_file" ]; then
        echo "[skip] no .zstd file in $cp_path"
        (( skipped++ )) || true
        continue
    fi

    out_dir="$RESULT_DIR/$bench/$cp_num/$CONFIG_MODE"
    mkdir -p "$out_dir"

    job_name="${bench}_${cp_num}_${CONFIG_MODE}"

    job_id=$(sbatch --parsable \
        --job-name="$job_name" \
        --partition=def \
        --nodes=1 \
        --ntasks=1 \
        --cpus-per-task=1 \
        --mem=16G \
        --output="$LOG_DIR/${job_name}_%j.out" \
        --error="$LOG_DIR/${job_name}_%j.err" \
        -- /usr/bin/env bash -c "
            set -euo pipefail
            echo \"[gem5] starting: $bench cp $cp_num ($CONFIG_MODE)\"
            '$GEM5_BIN' -d '$out_dir' '$CONFIG_PY' \
                --generic-rv-cpt='$cpt_file' \
                --difftest='$DIFFTEST' \
                --gcpt-restorer None \
                -I 40000000 \
                &>> '$out_dir/output.txt'
            echo \"[gem5] done: $bench cp $cp_num\"
        ")

    echo "[submitted] job $job_id — $bench cp $cp_num ($CONFIG_MODE)"
    (( submitted++ )) || true
done

echo ""
echo "Submitted: $submitted jobs  |  Skipped: $skipped"
echo "Logs:    $LOG_DIR"
echo "Results: $RESULT_DIR"
