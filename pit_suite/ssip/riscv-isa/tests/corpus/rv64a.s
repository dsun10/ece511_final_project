# Copyright James Wainwright
#
# SPDX-License-Identifier: MPL-2.0

lr.d x0, (x0)             # 1000302f
lr.d x4, (x17)            # 1008b22f
lr.d x31, (x31)           # 100fbfaf
sc.d x0, x0, (x0)         # 1800302f
sc.d x4, x13, (x27)       # 18ddb22f
sc.d x31, x31, (x31)      # 19ffbfaf
amoswap.d x0, x0, (x0)    # 0800302f
amoswap.d x4, x13, (x27)  # 08ddb22f
amoswap.d x31, x31, (x31) # 09ffbfaf
amoadd.d x0, x0, (x0)     # 0000302f
amoadd.d x4, x13, (x27)   # 00ddb22f
amoadd.d x31, x31, (x31)  # 01ffbfaf
amoxor.d x0, x0, (x0)     # 2000302f
amoxor.d x4, x13, (x27)   # 20ddb22f
amoxor.d x31, x31, (x31)  # 21ffbfaf
amoand.d x0, x0, (x0)     # 6000302f
amoand.d x4, x13, (x27)   # 60ddb22f
amoand.d x31, x31, (x31)  # 61ffbfaf
amoor.d x0, x0, (x0)      # 4000302f
amoor.d x4, x13, (x27)    # 40ddb22f
amoor.d x31, x31, (x31)   # 41ffbfaf
amomin.d x0, x0, (x0)     # 8000302f
amomin.d x4, x13, (x27)   # 80ddb22f
amomin.d x31, x31, (x31)  # 81ffbfaf
amomax.d x0, x0, (x0)     # a000302f
amomax.d x4, x13, (x27)   # a0ddb22f
amomax.d x31, x31, (x31)  # a1ffbfaf
amominu.d x0, x0, (x0)    # c000302f
amominu.d x4, x13, (x27)  # c0ddb22f
amominu.d x31, x31, (x31) # c1ffbfaf
amomaxu.d x0, x0, (x0)    # e000302f
amomaxu.d x4, x13, (x27)  # e0ddb22f
amomaxu.d x31, x31, (x31) # e1ffbfaf
