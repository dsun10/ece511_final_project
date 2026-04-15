# PIT Suite

Tools to generate and analyze Profiling Instruction Traces (PIT).

PIT is a largely made-up format that contains exactly enough information to do generic
workload analysis for RISC-V. A PIT trace consists of a stream of RISC-V instructions,
padded to be 32-bit (even if it is compressed). If the instruction is known to be a
memory instruction by NEMU, then it is followed by a 64-bit memory address. The
lean-ness of the format allows for fast analysis on instruction patterns, extension, and
makes all ostensible dependencies known.

The NEMU subfolder was branched off of mainline NEMU at v2026.01.r5. You can use this as
you would standard NEMU, but add the flag `-P <tracename>.pit` to generate PIT
files. These can then be consumed for analysis.

The ssip folder (SuperSonic Instruction Parser) serves to fill in that second
purpose. The `riscv_isa` subfolder is a forked version of the Rust crate with some
bugfixes for handling compressed instructions, as well as additional helpers to extract
register sources/destinations as an `Option` for instruction-agnostic dependency
tracking. SSIP is currently configured to perform an instruction mix analysis and
look for Xiangshan's fusion idioms, in addition to some theoretical ALU-Branch fusion
patterns we are currently investigating.
