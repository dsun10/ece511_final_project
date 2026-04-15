# Copyright James Wainwright
#
# SPDX-License-Identifier: MPL-2.0

c.ldsp x1, 0(x2)    # 6082
c.ldsp x9, 40(x2)   # 74a2
c.ldsp x21, 128(x2) # 6a8a
c.ldsp x21, 256(x2) # 6a92
c.sdsp x1, 0(x2)    # e006
c.sdsp x9, 40(x2)   # f426
c.sdsp x21, 128(x2) # e156
c.sdsp x21, 256(x2) # e256
c.ld x8, 0(x8)      # 6000
c.ld x10, 48(x12)   # 7a08
c.ld x15, 128(x15)  # 63dc
c.sd x8, 0(x8)      # e000
c.sd x10, 48(x12)   # fa08
c.sd x15, 128(x15)  # e3dc
c.addiw x1, 0       # 2081
c.addiw x5, 23      # 22dd
c.addiw x31, 31     # 2ffd
c.addw x8, x8       # 9c21
c.addw x12, x13     # 9e35
c.addw x15, x15     # 9fbd
c.subw x8, x8       # 9c01
c.subw x12, x13     # 9e15
c.subw x15, x15     # 9f9d
