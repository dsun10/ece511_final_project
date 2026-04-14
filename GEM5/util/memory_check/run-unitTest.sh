#!/bin/bash

gem5_home=$(pwd)
log_file=$gem5_home/valgrind.out

# cmd to test
# e.g. ./run-unitTest.sh ./build/RISCV/cpu/pred/btb/test/btb.test.debug --gtest_filter=BTBTest.ConditionalCounter
# use all args
run_cmd="$@"

valgrind --tool=memcheck \
    --track-origins=yes \
    --show-leak-kinds=all \
    --leak-check=full \
    --log-file=$log_file \
    -s \
    --suppressions=$gem5_home/util/valgrind-suppressions \
    $run_cmd

python3 $gem5_home/util/memory_check/check-memory-error.py $log_file