# Copyright James Wainwright
#
# SPDX-License-Identifier: MPL-2.0

c.flwsp f1, 0(x2)    # 6082
c.flwsp f9, 44(x2)   # 74b2
c.flwsp f31, 128(x2) # 6f8a
c.fswsp f1, 0(x2)    # e006
c.fswsp f9, 44(x2)   # f626
c.fswsp f31, 128(x2) # e17e
c.flw f8, 0(x8)      # 6000
c.flw f10, 24(x12)   # 6e08
c.flw f15, 64(x15)   # 63bc
c.fsw f8, 0(x8)      # e000
c.fsw f10, 24(x12)   # ee08
c.fsw f15, 64(x15)   # e3bc
