# Copyright James Wainwright
#
# SPDX-License-Identifier: MPL-2.0

ror x0, x0, x0      # 60005033
ror x4, x13, x27    # 61b6d233
ror x31, x31, x31   # 61ffdfb3
rol x0, x0, x0      # 60001033
rol x4, x13, x27    # 61b69233
rol x31, x31, x31   # 61ff9fb3
rori x0, x0, 0      # 60005013
rori x4, x13, 27    # 61b6d213
rori x31, x31, 31   # 61ffdf93
andn x0, x0, x0     # 40007033
andn x4, x13, x27   # 41b6f233
andn x31, x31, x31  # 41ffffb3
orn x0, x0, x0      # 40006033
orn x4, x13, x27    # 41b6e233
orn x31, x31, x31   # 41ffefb3
xnor x0, x0, x0     # 40004033
xnor x4, x13, x27   # 41b6c233
xnor x31, x31, x31  # 41ffcfb3
pack x0, x0, x0     # 08004033
pack x4, x13, x27   # 09b6c233
pack x31, x31, x31  # 09ffcfb3
packh x0, x0, x0    # 08007033
packh x4, x13, x27  # 09b6f233
packh x31, x31, x31 # 09ffffb3
brev8 x0, x0        # 68705013
brev8 x4, x27       # 687DD213
brev8 x31, x31      # 687FDF93
rev8 x0, x0         # 69805013
rev8 x4, x27        # 698dd213
rev8 x31, x31       # 698fdf93
zip x0, x0          # 08f01013
zip x4, x27         # 08fd9213
zip x31, x31        # 08ff9f93
unzip x0, x0        # 08f05013
unzip x4, x27       # 08fdd213
unzip x31, x31      # 08ffdf93
