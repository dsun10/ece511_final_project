# Copyright James Wainwright
#
# SPDX-License-Identifier: MPL-2.0

fcvt.l.s x0, f0    # c0207053
fcvt.l.s x12, f23  # c02bf653
fcvt.l.s x31, f31  # c02fffd3
fcvt.lu.s x0, f0   # c0307053
fcvt.lu.s x12, f23 # c03bf653
fcvt.lu.s x31, f31 # c03fffd3
fcvt.s.l f0, x0    # d0207053
fcvt.s.l f12, x23  # d02bf653
fcvt.s.l f31, x31  # d02fffd3
fcvt.s.lu f0, x0   # d0307053
fcvt.s.lu f12, x23 # d03bf653
fcvt.s.lu f31, x31 # d03fffd3
