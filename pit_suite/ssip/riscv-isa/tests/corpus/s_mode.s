# Copyright James Wainwright
#
# SPDX-License-Identifier: MPL-2.0

sret                # 10200073
sfence.vma x0, x0   # 12000073
sfence.vma x1, x31  # 13f08073
sfence.vma x31, x1  # 121f8073
sfence.vma x31, x31 # 13ff8073
