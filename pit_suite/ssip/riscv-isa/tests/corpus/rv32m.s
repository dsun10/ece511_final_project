# Copyright James Wainwright
#
# SPDX-License-Identifier: MPL-2.0

mul x0, x0, x0       # 02000033
mul x4, x17, x23     # 03788233
mul x31, x31, x31    # 03ff8fb3
mulh x0, x0, x0      # 02001033
mulh x4, x17, x23    # 03789233
mulh x31, x31, x31   # 03ff9fb3
mulhsu x0, x0, x0    # 02002033
mulhsu x4, x17, x23  # 0378a233
mulhsu x31, x31, x31 # 03ffafb3
mulhu x0, x0, x0     # 02003033
mulhu x4, x17, x23   # 0378b233
mulhu x31, x31, x31  # 03ffbfb3
div x0, x0, x0       # 02004033
div x4, x17, x23     # 0378c233
div x31, x31, x31    # 03ffcfb3
divu x0, x0, x0      # 02005033
divu x4, x17, x23    # 0378d233
divu x31, x31, x31   # 03ffdfb3
rem x0, x0, x0       # 02006033
rem x4, x17, x23     # 0378e233
rem x31, x31, x31    # 03ffefb3
remu x0, x0, x0      # 02007033
remu x4, x17, x23    # 0378f233
remu x31, x31, x31   # 03ffffb3
