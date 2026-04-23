#!/usr/bin/env bash
# Print non-zero triple fusion counts from ssip result files.
# Usage: ./show_triple_fusions.sh <results_dir>

RESULTS_DIR="${1:-/home/dsun19/ece511_final_project/pit_suite/results/20260415_135358}"

found=0

while IFS= read -r -d '' file; do
    # Extract the three sub-counts (lines starting with two spaces)
    slli_add_ldst=$(grep -m1 '  SLLI+ADD+LD/ST:' "$file" | awk '{print $2}')
    srai_xor_sub=$(grep  -m1 '  SRAI+XOR+SUB:'   "$file" | awk '{print $2}')
    lui_addi_slli=$(grep -m1 '  LUI+ADDI+SLLI:'  "$file" | awk '{print $2}')

    # Skip files where all three are zero or missing
    [[ "$slli_add_ldst" == "0" || -z "$slli_add_ldst" ]] && \
    [[ "$srai_xor_sub"  == "0" || -z "$srai_xor_sub"  ]] && \
    [[ "$lui_addi_slli" == "0" || -z "$lui_addi_slli" ]] && continue

    # Print the relative path and only the non-zero sub-counts
    rel="${file#$RESULTS_DIR/}"
    echo "$rel"
    [[ "$slli_add_ldst" != "0" && -n "$slli_add_ldst" ]] && echo "  SLLI+ADD+LD/ST: $slli_add_ldst"
    [[ "$srai_xor_sub"  != "0" && -n "$srai_xor_sub"  ]] && echo "  SRAI+XOR+SUB:   $srai_xor_sub"
    [[ "$lui_addi_slli" != "0" && -n "$lui_addi_slli" ]] && echo "  LUI+ADDI+SLLI:  $lui_addi_slli"
    found=$((found + 1))
done < <(find "$RESULTS_DIR" -name "*.txt" -print0 | sort -z)

echo ""
echo "$found file(s) with non-zero triple fusions."
