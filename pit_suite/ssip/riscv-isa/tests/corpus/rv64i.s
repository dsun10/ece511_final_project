# Copyright James Wainwright
#
# SPDX-License-Identifier: MPL-2.0

lwu x0, 0(x0)         # 00006003
lwu x1, -2(x31)       # ffefe083
lwu x31, -203(x5)     # f352ef83
lwu x31, 2047(x31)    # 7fffef83
lwu x31, -2048(x31)   # 800fef83
ld x0, 0(x0)          # 00003003
ld x1, -2(x31)        # ffefb083
ld x31, -203(x5)      # f352bf83
ld x31, 2047(x31)     # 7fffbf83
ld x31, -2048(x31)    # 800fbf83
sd x0, 0(x0)          # 00003023
sd x1, -2(x31)        # fe1fbf23
sd x31, -203(x5)      # f3f2baa3
sd x31, 2047(x31)     # 7fffbfa3
sd x31, -2048(x31)    # 81ffb023
slli x0, x0, 0        # 00001013
slli x1, x13, 27      # 01b69093
slli x16, x23, 31     # 01fb9813
slli x31, x31, 63     # 03ff9f93
srli x0, x0, 0        # 00005013
srli x1, x13, 27      # 01b6d093
srli x16, x23, 31     # 01fbd813
srli x31, x31, 63     # 03ffdf93
srai x0, x0, 0        # 40005013
srai x1, x13, 27      # 41b6d093
srai x16, x23, 31     # 41fbd813
srai x31, x31, 63     # 43ffdf93
addiw x0, x0, 0       # 0000001b
addiw x5, x27, 24     # 018d829b
addiw x31, x31, 2047  # 7fff8f9b
addiw x31, x31, -2048 # 800f8f9b
slliw x0, x0, 0       # 0000101b
slliw x5, x13, 27     # 01b6929b
slliw x31, x31, 31    # 01ff9f9b
srliw x0, x0, 0       # 0000501b
srliw x5, x13, 27     # 01b6d29b
srliw x31, x31, 31    # 01ffdf9b
sraiw x0, x0, 0       # 4000501b
sraiw x5, x13, 27     # 41b6d29b
sraiw x31, x31, 31    # 41ffdf9b
addw x0, x0, x0       # 0000003b
addw x4, x13, x27     # 01b6823b
addw x31, x31, x31    # 01ff8fbb
subw x0, x0, x0       # 4000003b
subw x4, x13, x27     # 41b6823b
subw x31, x31, x31    # 41ff8fbb
sllw x0, x0, x0       # 0000103b
sllw x4, x13, x27     # 01b6923b
sllw x31, x31, x31    # 01ff9fbb
srlw x0, x0, x0       # 0000503b
srlw x4, x13, x27     # 01b6d23b
srlw x31, x31, x31    # 01ffdfbb
sraw x0, x0, x0       # 4000503b
sraw x4, x13, x27     # 41b6d23b
sraw x31, x31, x31    # 41ffdfbb
