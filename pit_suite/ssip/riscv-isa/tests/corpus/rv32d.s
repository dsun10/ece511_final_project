# Copyright James Wainwright
#
# SPDX-License-Identifier: MPL-2.0

fld f0, 0(x0)               # 00003007
fld f13, -45(x7)            # fd33b687
fld f4, 612(x23)            # 264bb207
fld f15, 2047(x12)          # 7ff63787
fld f25, -2048(x16)         # 80083c87
fld f31, -1(x31)            # ffffbf87
fsd f0, 0(x0)               # 00003027
fsd f13, -45(x7)            # fcd3b9a7
fsd f4, 612(x23)            # 264bb227
fsd f15, 2047(x12)          # 7ef63fa7
fsd f25, -2048(x16)         # 81983027
fsd f31, -1(x31)            # ffffbfa7
fmadd.d f0, f0, f0, f0      # 02007043
fmadd.d f13, f25, f3, f18   # 923cf6c3
fmadd.d f31, f31, f31, f31  # fbffffc3
fmsub.d f0, f0, f0, f0      # 02007047
fmsub.d f13, f25, f3, f18   # 923cf6c7
fmsub.d f31, f31, f31, f31  # fbffffc7
fnmsub.d f0, f0, f0, f0     # 0200704b
fnmsub.d f13, f25, f3, f18  # 923cf6cb
fnmsub.d f31, f31, f31, f31 # fbffffcb
fnmadd.d f0, f0, f0, f0     # 0200704f
fnmadd.d f13, f25, f3, f18  # 923cf6cf
fnmadd.d f31, f31, f31, f31 # fbffffcf
fadd.d f0, f0, f0           # 02007053
fadd.d f13, f26, f3         # 023d76d3
fadd.d f31, f31, f31        # 03ffffd3
fsub.d f0, f0, f0           # 0a007053
fsub.d f13, f26, f3         # 0a3d76d3
fsub.d f31, f31, f31        # 0bffffd3
fmul.d f0, f0, f0           # 12007053
fmul.d f13, f26, f3         # 123d76d3
fmul.d f31, f31, f31        # 13ffffd3
fdiv.d f0, f0, f0           # 1a007053
fdiv.d f13, f26, f3         # 1a3d76d3
fdiv.d f31, f31, f31        # 1bffffd3
fsqrt.d f0, f0              # 5a007053
fsqrt.d f13, f26            # 5a0d76d3
fsqrt.d f31, f31            # 5a0fffd3
fsgnj.d f0, f0, f0          # 22000053
fsgnj.d f13, f26, f3        # 223d06d3
fsgnj.d f31, f31, f31       # 23ff8fd3
fsgnjn.d f0, f0, f0         # 22001053
fsgnjn.d f13, f26, f3       # 223d16d3
fsgnjn.d f31, f31, f31      # 23ff9fd3
fsgnjx.d f0, f0, f0         # 22002053
fsgnjx.d f13, f26, f3       # 223d26d3
fsgnjx.d f31, f31, f31      # 23ffafd3
fmin.d f0, f0, f0           # 2a000053
fmin.d f13, f26, f3         # 2a3d06d3
fmin.d f31, f31, f31        # 2bff8fd3
fmax.d f0, f0, f0           # 2a001053
fmax.d f13, f26, f3         # 2a3d16d3
fmax.d f31, f31, f31        # 2bff9fd3
fcvt.s.d f0, f0             # 40107053
fcvt.s.d f12, f23           # 401bf653
fcvt.s.d f31, f31           # 401fffd3
fcvt.d.s f0, f0             # 42007053
fcvt.d.s f12, f23           # 420bf653
fcvt.d.s f31, f31           # 420fffd3
feq.d x0, f0, f0            # a2002053
feq.d x13, f26, f3          # a23d26d3
feq.d x31, f31, f31         # a3ffafd3
flt.d x0, f0, f0            # a2001053
flt.d x13, f26, f3          # a23d16d3
flt.d x31, f31, f31         # a3ff9fd3
fle.d x0, f0, f0            # a2000053
fle.d x13, f26, f3          # a23d06d3
fle.d x31, f31, f31         # a3ff8fd3
fclass.d x0, f0             # e2001053
fclass.d x12, f23           # e20b9653
fclass.d x31, f31           # e20f9fd3
fcvt.w.d x0, f0             # c2007053
fcvt.w.d x12, f23           # c20bf653
fcvt.w.d x31, f31           # c20fffd3
fcvt.wu.d x0, f0            # c2107053
fcvt.wu.d x12, f23          # c21bf653
fcvt.wu.d x31, f31          # c21fffd3
fcvt.d.w f0, x0             # d2007053
fcvt.d.w f12, x23           # d20bf653
fcvt.d.w f31, x31           # d20fffd3
fcvt.d.wu f0, x0            # d2107053
fcvt.d.wu f12, x23          # d21bf653
fcvt.d.wu f31, x31          # d21fffd3
