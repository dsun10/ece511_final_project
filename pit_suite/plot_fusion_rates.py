#!/usr/bin/env python3
"""
Plot SPEC2006 instruction fusion rates by benchmark.

Baseline Xiangshan fusions (already implemented):
  - Logic Fusions    (slli+add, lui+addi, shift/logic pairs, etc.)
  - Memory Fusions   (adjacent sequential loads, far/address-calc loads)

New fusions (our additions):
  - Branch Fusion    (ALU + branch)
  - JALR Fusion      (LUI/ADDI/AUIPC + JALR)
  - MAC Fusion       (MUL/MULW → ADD/ADDW accumulate)
  - Triple Fusion    (SLLI+ADD+LD/ST, SRAI+XOR+SUB, LUI+ADDI+SLLI)
"""

import re
import sys
import argparse
from pathlib import Path
from collections import defaultdict

import numpy as np
import matplotlib
import matplotlib.pyplot as plt
import matplotlib.patches as mpatches


# ── Parsing ──────────────────────────────────────────────────────────────────

PATTERNS = {
    'total':       r'Total Decoded Instructions: (\d+)',
    'fusions':     r'Fused Pairs: (\d+)',
    'adjloads':    r'Adjacent Load Fusions: (\d+)',
    'farloads':    r'Far Loads: (\d+)',
    'logicfusion': r'Logic Fusions: (\d+)',
    'alubranch':   r'Branch Fusion \(Theoretical\): (\d+)',
    'alujalr':     r'JALR Fusion \(Theoretical\): (\d+)',
    'macc':        r'Multiply-Accumulate Fusions: (\d+)',
    'triple':      r'^Triple Fusions: (\d+)',
}


def parse_file(path: Path) -> dict:
    text = path.read_text()
    out = {}
    for key, pat in PATTERNS.items():
        m = re.search(pat, text, re.MULTILINE)
        out[key] = int(m.group(1)) if m else 0
    return out


def aggregate_benchmark(bench_dir: Path) -> dict:
    totals: dict[str, int] = defaultdict(int)
    for txt in bench_dir.glob("*.txt"):
        for k, v in parse_file(txt).items():
            totals[k] += v
    return dict(totals)


# ── Category computation ──────────────────────────────────────────────────────

def compute_categories(d: dict) -> dict:
    """
    Return raw counts for each plot category.
    Logic baseline  = logicfusion + (fusions - alubranch - alujalr - macc
                                     - logicfusion - adjloads - farloads)
                    = fusions - alubranch - alujalr - macc - adjloads - farloads
    Memory baseline = adjloads + farloads
    """
    fused_baseline = d['fusions']
    logic   = fused_baseline - d['adjloads'] - d['farloads']
    memory  = d['adjloads'] + d['farloads']
    branch  = d['alubranch']
    jalr    = d['alujalr']
    mac     = d['macc']
    triple  = d['triple']
    return dict(logic=max(logic, 0), memory=memory,
                branch=branch, jalr=jalr, mac=mac, triple=triple)


# ── Plotting ──────────────────────────────────────────────────────────────────

COLORS = {
    'logic':  '#1a3a5c',   # dark navy      — Xiangshan baseline logic
    'memory': '#1f6fa8',   # medium blue    — Xiangshan baseline memory
    'branch': '#3aaa6e',   # teal-green     — New: branch fusion
    'jalr':   '#f0a500',   # amber          — New: JALR fusion
    'mac':    '#c0392b',   # red            — New: MAC fusion
    'triple': '#8e44ad',   # purple         — New: triple fusion
}

LABELS = {
    'logic':  'Xiangshan Baseline (Logic)',
    'memory': 'Xiangshan Baseline (Memory)',
    'branch': 'Branch Fusion (New)',
    'jalr':   'JALR Fusion (New)',
    'mac':    'MAC Fusion (New)',
    'triple': 'Triple Fusion (New)',
}

CATEGORIES = ['logic', 'memory', 'branch', 'jalr', 'mac', 'triple']


def plot(results_dir: Path, output: Path):
    # Collect per-benchmark aggregates
    rows = []
    for bench_dir in sorted(results_dir.iterdir()):
        if not bench_dir.is_dir():
            continue
        data = aggregate_benchmark(bench_dir)
        total = data.get('total', 0)
        if total == 0:
            continue
        cats = compute_categories(data)
        pct = {k: 100.0 * v / total for k, v in cats.items()}
        pct['_total'] = sum(pct.values())
        pct['_name']  = bench_dir.name
        rows.append(pct)

    if not rows:
        sys.exit(f"No .txt files found under {results_dir}")

    # Sort by total fusion rate descending
    rows.sort(key=lambda r: -r['_total'])

    names = [r['_name'] for r in rows]
    x = np.arange(len(names))

    fig, ax = plt.subplots(figsize=(22, 8))

    bottoms = np.zeros(len(names))
    for cat in CATEGORIES:
        vals = np.array([r[cat] for r in rows])
        ax.bar(x, vals, bottom=bottoms,
               color=COLORS[cat], label=LABELS[cat], width=0.75)
        bottoms += vals

    # Styling
    ax.set_xticks(x)
    ax.set_xticklabels(names, rotation=45, ha='right', fontsize=7.5)
    ax.set_ylabel('Percentage of Total Decoded Instructions (%)', fontsize=11)
    ax.set_title('SPEC2006 Instruction Fusion Rates by Benchmark', fontsize=13, pad=12)
    ax.yaxis.grid(True, linestyle='--', alpha=0.6, zorder=0)
    ax.set_axisbelow(True)
    ax.set_xlim(-0.6, len(names) - 0.4)

    # Separator line between baseline and new categories
    # Draw a thin dividing line at the top of memory bars for visual clarity
    for i, r in enumerate(rows):
        baseline_pct = r['logic'] + r['memory']
        if baseline_pct > 0:
            ax.plot([i - 0.375, i + 0.375], [baseline_pct, baseline_pct],
                    color='white', linewidth=0.8, zorder=5)

    # Legend — show baseline group and new group with a header via proxies
    legend_handles = []
    for cat in CATEGORIES:
        legend_handles.append(
            mpatches.Patch(color=COLORS[cat], label=LABELS[cat])
        )
    ax.legend(handles=legend_handles, loc='upper right', fontsize=9,
              framealpha=0.9)

    plt.tight_layout()
    fig.savefig(output, dpi=150, bbox_inches='tight')
    print(f"Saved → {output}")
    plt.show()


# ── CLI ───────────────────────────────────────────────────────────────────────

def main():
    parser = argparse.ArgumentParser(description=__doc__,
                                     formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument('results_dir', nargs='?',
                        default='/home/dsun19/ece511_final_project/pit_suite/results/20260415_135358',
                        help='Directory containing per-benchmark subdirectories of .txt result files')
    parser.add_argument('-o', '--output', default='fusion_rates.png',
                        help='Output image path (default: fusion_rates.png)')
    args = parser.parse_args()

    plot(Path(args.results_dir), Path(args.output))


if __name__ == '__main__':
    main()
