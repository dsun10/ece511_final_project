# Copyright James Wainwright
#
# SPDX-License-Identifier: MPL-2.0

andn x0, x0, x0    # 40007033
andn x3, x14, x27  # 41b771b3
andn x31, x31, x31 # 41ffffb3
orn x0, x0, x0     # 40006033
orn x3, x14, x27   # 41b761b3
orn x31, x31, x31  # 41ffefb3
xnor x0, x0, x0    # 40004033
xnor x3, x14, x27  # 41b741b3
xnor x31, x31, x31 # 41ffcfb3
clz x0, x0         # 60001013
clz x3, x26        # 600d1193
clz x31, x31       # 600f9f93
ctz x0, x0         # 60101013
ctz x3, x26        # 601d1193
ctz x31, x31       # 601f9f93
cpop x0, x0        # 60201013
cpop x3, x26       # 602d1193
cpop x31, x31      # 602f9f93
max x0, x0, x0     # 0a006033
max x3, x14, x27   # 0bb761b3
max x31, x31, x31  # 0bffefb3
maxu x0, x0, x0    # 0a007033
maxu x3, x14, x27  # 0bb771b3
maxu x31, x31, x31 # 0bffffb3
min x0, x0, x0     # 0a004033
min x3, x14, x27   # 0bb741b3
min x31, x31, x31  # 0bffcfb3
minu x0, x0, x0    # 0a005033
minu x3, x14, x27  # 0bb751b3
minu x31, x31, x31 # 0bffdfb3
sext.b x0, x0      # 60401013
sext.b x3, x26     # 604d1193
sext.b x31, x31    # 604f9f93
sext.h x0, x0      # 60501013
sext.h x3, x26     # 605d1193
sext.h x31, x31    # 605f9f93
zext.h x0, x0      # 08004033
zext.h x3, x26     # 080d41b3
zext.h x31, x31    # 080fcfb3
rol x0, x0, x0     # 60001033
rol x3, x14, x27   # 61b711b3
rol x31, x31, x31  # 61ff9fb3
ror x0, x0, x0     # 60005033
ror x3, x14, x27   # 61b751b3
ror x31, x31, x31  # 61ffdfb3
rori x0, x0, 0     # 60005013
rori x3, x14, 27   # 61b75193
rori x31, x31, 31  # 61ffdf93
orc.b x0, x0       # 28705013
orc.b x3, x26      # 287d5193
orc.b x31, x31     # 287fdf93
rev8 x0, x0        # 69805013
rev8 x3, x26       # 698d5193
rev8 x31, x31      # 698fdf93
