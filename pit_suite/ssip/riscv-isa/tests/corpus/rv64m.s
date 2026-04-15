# Copyright James Wainwright
#
# SPDX-License-Identifier: MPL-2.0

mulw x0, x0, x0     # 0200003b
mulw x4, x17, x29   # 03d8823b
mulw x31, x31, x31  # 03ff8fbb
divw x0, x0, x0     # 0200403b
divw x4, x17, x29   # 03d8c23b
divw x31, x31, x31  # 03ffcfbb
divuw x0, x0, x0    # 0200503b
divuw x4, x17, x29  # 03d8d23b
divuw x31, x31, x31 # 03ffdfbb
remw x0, x0, x0     # 0200603b
remw x4, x17, x29   # 03d8e23b
remw x31, x31, x31  # 03ffefbb
remuw x0, x0, x0    # 0200703b
remuw x4, x17, x29  # 03d8f23b
remuw x31, x31, x31 # 03ffffbb
