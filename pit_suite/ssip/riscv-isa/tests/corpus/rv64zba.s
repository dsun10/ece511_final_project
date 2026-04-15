# Copyright James Wainwright
#
# SPDX-License-Identifier: MPL-2.0

add.uw x0, x0, x0       # 0800003b
add.uw x3, x14, x27     # 09b701bb
add.uw x31, x31, x31    # 09ff8fbb
sh1add.uw x0, x0, x0    # 2000203b
sh1add.uw x3, x14, x27  # 21b721bb
sh1add.uw x31, x31, x31 # 21ffafbb
sh2add.uw x0, x0, x0    # 2000403b
sh2add.uw x3, x14, x27  # 21b741bb
sh2add.uw x31, x31, x31 # 21ffcfbb
sh3add.uw x0, x0, x0    # 2000603b
sh3add.uw x3, x14, x27  # 21b761bb
sh3add.uw x31, x31, x31 # 21ffefbb
slli.uw x0, x0, 0       # 0800101b
slli.uw x3, x27, 24     # 098d919b
slli.uw x31, x31, 63    # 0bff9f9b
