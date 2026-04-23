#!/bin/bash
# Usage: ./run-sims.sh [fuse|nofuse]  (default: nofuse)

CONFIG_MODE="${1:-nofuse}"

# --- Path Configurations ---
export GEM5_BIN="/home/dsun19/ece511_final_project/GEM5/build/RISCV/gem5.fast"
NO_FUSE_CONFIG_PY="/home/dsun19/ece511_final_project/GEM5/configs/example/kmhv3.py"
FUSE_CONFIG_PY="/home/dsun19/ece511_final_project/GEM5/configs/example/fusion_config.py"

if [ "$CONFIG_MODE" = "fuse" ]; then
    export CONFIG_PY="$FUSE_CONFIG_PY"
elif [ "$CONFIG_MODE" = "nofuse" ]; then
    export CONFIG_PY="$NO_FUSE_CONFIG_PY"
else
    echo "Usage: $0 [fuse|nofuse]" >&2
    exit 1
fi

export CONFIG_MODE
export ROOT="/fast-lab-share/tw12/spec2006_simpoints/sim_checkpoint_results/spec-cpt"
export RESULT_DIR="/home/dsun19/ece511_final_project/sim_results/spec06"

mkdir -p "$RESULT_DIR"


# --- Checkpoints with most weight ---
CHECKPOINTS=(
    #"400.perlbench_checkspam   | 19683"
    "400.perlbench_diffmail    | 7161"
    #"400.perlbench_splitmail   | 21165"          
    #"401.bzip2_chicken         | 2919"           
    #"401.bzip2_inputcombined   | 12217"          
    #"401.bzip2_inputprogram    | 16624"          
    #"401.bzip2_inputsource     | 16155"          
    #"401.bzip2_liberty         | 4520"           
    #"401.bzip2_texthtml        | 3848"           
    #"403.gcc_166               | 1765"           
    "403.gcc_200               | 536"            
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
    "429.mcf                   | 6351"           
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
    "447.dealII                | 18272"          
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
    "473.astar_biglake         | 1504"           
    #"473.astar_rivers          | 8943"           
    #"481.wrf                   | 31374"          
    #"482.sphinx3               | 113583"         
    #"483.xalancbmk             | 10869"
)

# --- Simulation Function ---
run_sim() {
    local bench="$1"
    local cp_num="$2"

    local cp_path="$ROOT/$bench/$cp_num"

    [ -d "$cp_path" ] || return 0

    local cpt_file
    cpt_file=$(ls "$cp_path"/*.zstd 2>/dev/null | head -n 1)
    [ -n "$cpt_file" ] || return 0

    local out_dir="$RESULT_DIR/$bench/$cp_num/$CONFIG_MODE"
    mkdir -p "$out_dir"

    echo "running $cp_path"

    # Execution (redirecting all gem5 output to a per-run log)
    $GEM5_BIN -d "$out_dir" "$CONFIG_PY" \
        --generic-rv-cpt="$cpt_file" \
        --difftest=/storage/pradyun/xiangshan/difftest_NEMU/build/riscv64-nemu-interpreter-so \
        --gcpt-restorer None -I 40000000 &> "$out_dir/output.txt"
}

export -f run_sim

echo "Launching simulations (config: $CONFIG_MODE)..."

JOBS="${JOBS:-48}"
echo "Parallelism: $JOBS jobs"

printf '%s\n' "${CHECKPOINTS[@]}" | \
    parallel -j "$JOBS" --joblog "$RESULT_DIR/sweep_results.log" --colsep '\s*\|\s*' \
    run_sim {1} {2}

echo "All simulations complete."
