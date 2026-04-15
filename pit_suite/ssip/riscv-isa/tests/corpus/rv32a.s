# Copyright James Wainwright
#
# SPDX-License-Identifier: MPL-2.0

lr.w x0, (x0)             # 1000202f
lr.w x4, (x17)            # 1008a22f
lr.w x31, (x31)           # 100fafaf
sc.w x0, x0, (x0)         # 1800202f
sc.w x4, x13, (x27)       # 18dda22f
sc.w x31, x31, (x31)      # 19ffafaf
amoswap.w x0, x0, (x0)    # 0800202f
amoswap.w x4, x13, (x27)  # 08dda22f
amoswap.w x31, x31, (x31) # 09ffafaf
amoadd.w x0, x0, (x0)     # 0000202f
amoadd.w x4, x13, (x27)   # 00dda22f
amoadd.w x31, x31, (x31)  # 01ffafaf
amoxor.w x0, x0, (x0)     # 2000202f
amoxor.w x4, x13, (x27)   # 20dda22f
amoxor.w x31, x31, (x31)  # 21ffafaf
amoand.w x0, x0, (x0)     # 6000202f
amoand.w x4, x13, (x27)   # 60dda22f
amoand.w x31, x31, (x31)  # 61ffafaf
amoor.w x0, x0, (x0)      # 4000202f
amoor.w x4, x13, (x27)    # 40dda22f
amoor.w x31, x31, (x31)   # 41ffafaf
amomin.w x0, x0, (x0)     # 8000202f
amomin.w x4, x13, (x27)   # 80dda22f
amomin.w x31, x31, (x31)  # 81ffafaf
amomax.w x0, x0, (x0)     # a000202f
amomax.w x4, x13, (x27)   # a0dda22f
amomax.w x31, x31, (x31)  # a1ffafaf
amominu.w x0, x0, (x0)    # c000202f
amominu.w x4, x13, (x27)  # c0dda22f
amominu.w x31, x31, (x31) # c1ffafaf
amomaxu.w x0, x0, (x0)    # e000202f
amomaxu.w x4, x13, (x27)  # e0dda22f
amomaxu.w x31, x31, (x31) # e1ffafaf
