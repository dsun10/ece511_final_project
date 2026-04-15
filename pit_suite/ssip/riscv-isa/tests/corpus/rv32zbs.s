# Copyright James Wainwright
#
# SPDX-License-Identifier: MPL-2.0

bclr x0, x0, x0    # 48001033
bclr x3, x14, x27  # 49b711b3
bclr x31, x31, x31 # 49ff9fb3
bclri x0, x0, 0    # 48001013
bclri x3, x14, 27  # 49b71193
bclri x31, x31, 31 # 49ff9f93
bext x0, x0, x0    # 48005033
bext x3, x14, x27  # 49b751b3
bext x31, x31, x31 # 49ffdfb3
bexti x0, x0, 0    # 48005013
bexti x3, x14, 27  # 49b75193
bexti x31, x31, 31 # 49ffdf93
binv x0, x0, x0    # 68001033
binv x3, x14, x27  # 69b711b3
binv x31, x31, x31 # 69ff9fb3
binvi x0, x0, 0    # 68001013
binvi x3, x14, 27  # 69b71193
binvi x31, x31, 31 # 69ff9f93
bset x0, x0, x0    # 28001033
bset x3, x14, x27  # 29b711b3
bset x31, x31, x31 # 29ff9fb3
bseti x0, x0, 0    # 28001013
bseti x3, x14, 27  # 29b71193
bseti x31, x31, 31 # 29ff9f93
