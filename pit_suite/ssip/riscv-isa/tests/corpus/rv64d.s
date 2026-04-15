# Copyright James Wainwright
#
# SPDX-License-Identifier: MPL-2.0

fcvt.l.d x0, f0    # c2207053
fcvt.l.d x12, f23  # c22bf653
fcvt.l.d x31, f31  # c22fffd3
fcvt.lu.d x0, f0   # c2307053
fcvt.lu.d x12, f23 # c23bf653
fcvt.lu.d x31, f31 # c23fffd3
fmv.x.d x0, f0     # e2000053
fmv.x.d x12, f23   # e20b8653
fmv.x.d x31, f31   # e20f8fd3
fcvt.d.l f0, x0    # d2207053
fcvt.d.l f12, x23  # d22bf653
fcvt.d.l f31, x31  # d22fffd3
fcvt.d.lu f0, x0   # d2307053
fcvt.d.lu f12, x23 # d23bf653
fcvt.d.lu f31, x31 # d23fffd3
fmv.d.x f0, x0     # f2000053
fmv.d.x f12, x23   # f20b8653
fmv.d.x f31, x31   # f20f8fd3
