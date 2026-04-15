# Copyright James Wainwright
#
# SPDX-License-Identifier: MPL-2.0

rori x0, x0, 0      # 60005013
rori x4, x13, 27    # 61b6d213
rori x31, x31, 63   # 63ffdf93
rorw x0, x0, x0     # 6000503b
rorw x4, x13, x27   # 61b6d23b
rorw x31, x31, x31  # 61ffdfbb
rolw x0, x0, x0     # 6000103b
rolw x4, x13, x27   # 61b6923b
rolw x31, x31, x31  # 61ff9fbb
roriw x0, x0, 0     # 6000501b
roriw x4, x13, 27   # 61b6d21b
roriw x31, x31, 31  # 61ffdf9b
packw x0, x0, x0    # 0800403b
packw x4, x13, x27  # 09b6c23b
packw x31, x31, x31 # 09ffcfbb
rev8 x0, x0         # 6b805013
rev8 x4, x27        # 6b8dd213
rev8 x31, x31       # 6b8fdf93
