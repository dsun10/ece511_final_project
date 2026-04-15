# Copyright James Wainwright
#
# SPDX-License-Identifier: MPL-2.0

clzw x0, x0        # 6000101b
clzw x3, x27       # 600d919b
clzw x31, x31      # 600f9f9b
ctzw x0, x0        # 6010101b
ctzw x3, x27       # 601d919b
ctzw x31, x31      # 601f9f9b
cpopw x0, x0       # 6020101b
cpopw x3, x27      # 602d919b
cpopw x31, x31     # 602f9f9b
rolw x0, x0, x0    # 6000103b
rolw x3, x14, x27  # 61b711bb
rolw x31, x31, x31 # 61ff9fbb
rori x0, x0, 0     # 60005013
rori x3, x14, 27   # 61b75193
rori x31, x31, 63  # 63ffdf93
roriw x0, x0, 0    # 6000501b
roriw x3, x14, 27  # 61b7519b
roriw x31, x31, 31 # 61ffdf9b
rorw x0, x0, x0    # 6000503b
rorw x3, x14, x27  # 61b751bb
rorw x31, x31, x31 # 61ffdfbb
