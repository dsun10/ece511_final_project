#!/bin/bash
# Submit a single gem5 simulation to SLURM.
# Usage: ./slurm-run-one.sh <checkpoint_path> [fuse|nofuse]
#
# Example:
#   ./slurm-run-one.sh /fast-lab-share/tw12/spec2006_simpoints/.../429.mcf/6351/gcpt.bin fuse

set -euo pipefail

CPT_FILE="${1:?Usage: $0 <checkpoint_path> [fuse|nofuse]}"
CONFIG_MODE="${2:-nofuse}"

GEM5_ROOT="/research/dsun19/ece511_final_project/GEM5"
GEM5_BIN="$GEM5_ROOT/build/RISCV/gem5.fast"
DIFFTEST="/fast-lab-share/pradyun/xiangshan_difftest_so"
RESULT_DIR="/research/dsun19/ece511_final_project/sim_results/one-off/$(date +%Y%m%d_%H%M%S)_${CONFIG_MODE}"

case "$CONFIG_MODE" in
    fuse)   CONFIG_PY="$GEM5_ROOT/configs/example/fusion_config.py" ;;
    nofuse) CONFIG_PY="$GEM5_ROOT/configs/example/kmhv3.py" ;;
    *)      echo "Config mode must be 'fuse' or 'nofuse'" >&2; exit 1 ;;
esac

mkdir -p "$RESULT_DIR"

echo "Checkpoint : $CPT_FILE"
echo "Config     : $CONFIG_MODE ($CONFIG_PY)"
echo "Output dir : $RESULT_DIR"

job_id=$(sbatch --parsable \
    --job-name="gem5_one_${CONFIG_MODE}" \
    --partition=def \
    --nodes=1 \
    --ntasks=1 \
    --cpus-per-task=1 \
    --mem=4G \
    --output="$RESULT_DIR/slurm_%j.out" \
    --error="$RESULT_DIR/slurm_%j.err" \
    <<EOF
#!/bin/bash
set -euo pipefail
"$GEM5_BIN" -d "$RESULT_DIR" "$CONFIG_PY" \
    --generic-rv-cpt="$CPT_FILE" \
    --difftest="$DIFFTEST" \
    --gcpt-restorer None \
    -I 40000000 \
    &>> "$RESULT_DIR/output.txt"
EOF
)

echo "Submitted job $job_id"
echo "Watch: squeue -j $job_id"
echo "Tail:  tail -f $RESULT_DIR/output.txt"
