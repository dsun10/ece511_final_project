# Copyright James Wainwright
#
# SPDX-License-Identifier: MPL-2.0

c.fldsp f1, 0(x2)    # 2082
c.fldsp f9, 48(x2)   # 34c2
c.fldsp f31, 128(x2) # 2f8a
c.fsdsp f1, 0(x2)    # a006
c.fsdsp f9, 48(x2)   # b826
c.fsdsp f31, 128(x2) # a17e
c.fld f8, 0(x8)      # 2000
c.fld f10, 24(x12)   # 2e08
c.fld f15, 64(x15)   # 23bc
c.fsd f8, 0(x8)      # a000
c.fsd f10, 24(x12)   # ae08
c.fsd f15, 64(x15)   # a3bc
