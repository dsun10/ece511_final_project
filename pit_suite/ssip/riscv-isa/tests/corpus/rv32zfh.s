# Copyright James Wainwright
#
# SPDX-License-Identifier: MPL-2.0

flh f0, 0(x0)               # 00001007
flh f13, -45(x7)            # fd339687
flh f4, 612(x23)            # 264b9207
flh f15, 2047(x12)          # 7ff61787
flh f25, -2048(x16)         # 80081c87
flh f31, -1(x31)            # ffff9f87
fsh f0, 0(x0)               # 00001027
fsh f13, -45(x7)            # fcd399a7
fsh f4, 612(x23)            # 264b9227
fsh f15, 2047(x12)          # 7ef61fa7
fsh f25, -2048(x16)         # 81981027
fsh f31, -1(x31)            # ffff9fa7
fmadd.h f0, f0, f0, f0      # 04007043
fmadd.h f13, f25, f3, f18   # 943cf6c3
fmadd.h f31, f31, f31, f31  # fdffffc3
fmsub.h f0, f0, f0, f0      # 04007047
fmsub.h f13, f25, f3, f18   # 943cf6c7
fmsub.h f31, f31, f31, f31  # fdffffc7
fnmsub.h f0, f0, f0, f0     # 0400704b
fnmsub.h f13, f25, f3, f18  # 943cf6cb
fnmsub.h f31, f31, f31, f31 # fdffffcb
fnmadd.h f0, f0, f0, f0     # 0400704f
fnmadd.h f13, f25, f3, f18  # 943cf6cf
fnmadd.h f31, f31, f31, f31 # fdffffcf
fadd.h f0, f0, f0           # 04007053
fadd.h f13, f26, f3         # 043d76d3
fadd.h f31, f31, f31        # 05ffffd3
fsub.h f0, f0, f0           # 0c007053
fsub.h f13, f26, f3         # 0c3d76d3
fsub.h f31, f31, f31        # 0dffffd3
fmul.h f0, f0, f0           # 14007053
fmul.h f13, f26, f3         # 143d76d3
fmul.h f31, f31, f31        # 15ffffd3
fdiv.h f0, f0, f0           # 1c007053
fdiv.h f13, f26, f3         # 1c3d76d3
fdiv.h f31, f31, f31        # 1dffffd3
fsqrt.h f0, f0              # 5c007053
fsqrt.h f13, f26            # 5c0d76d3
fsqrt.h f31, f31            # 5c0fffd3
fsgnj.h f0, f0, f0          # 24000053
fsgnj.h f13, f26, f3        # 243d06d3
fsgnj.h f31, f31, f31       # 25ff8fd3
fsgnjn.h f0, f0, f0         # 24001053
fsgnjn.h f13, f26, f3       # 243d16d3
fsgnjn.h f31, f31, f31      # 25ff9fd3
fsgnjx.h f0, f0, f0         # 24002053
fsgnjx.h f13, f26, f3       # 243d26d3
fsgnjx.h f31, f31, f31      # 25ffafd3
fmin.h f0, f0, f0           # 2c000053
fmin.h f13, f26, f3         # 2c3d06d3
fmin.h f31, f31, f31        # 2dff8fd3
fmax.h f0, f0, f0           # 2c001053
fmax.h f13, f26, f3         # 2c3d16d3
fmax.h f31, f31, f31        # 2dff9fd3
fcvt.s.h f0, f0             # 40200053
fcvt.s.h f12, f23           # 402b8653
fcvt.s.h f31, f31           # 402f8fd3
fcvt.h.s f0, f0             # 44007053
fcvt.h.s f12, f23           # 440bf653
fcvt.h.s f31, f31           # 440fffd3
fcvt.d.h f0, f0             # 42200053
fcvt.d.h f12, f23           # 422b8653
fcvt.d.h f31, f31           # 422f8fd3
fcvt.h.d f0, f0             # 44107053
fcvt.h.d f12, f23           # 441bf653
fcvt.h.d f31, f31           # 441fffd3
fcvt.q.h f0, f0             # 46200053
fcvt.q.h f12, f23           # 462b8653
fcvt.q.h f31, f31           # 462fffd3
fcvt.h.q f0, f0             # 44300053
fcvt.h.q f12, f23           # 443b8653
fcvt.h.q f31, f31           # 443f8fd3
feq.h x0, f0, f0            # a4002053
feq.h x13, f26, f3          # a43d26d3
feq.h x31, f31, f31         # a5ffafd3
flt.h x0, f0, f0            # a4001053
flt.h x13, f26, f3          # a43d16d3
flt.h x31, f31, f31         # a5ff9fd3
fle.h x0, f0, f0            # a4000053
fle.h x13, f26, f3          # a43d06d3
fle.h x31, f31, f31         # a5ff8fd3
fclass.h x0, f0             # e4001053
fclass.h x12, f23           # e40b9653
fclass.h x31, f31           # e40f9fd3
fcvt.w.h x0, f0             # c4007053
fcvt.w.h x12, f23           # c40bf653
fcvt.w.h x31, f31           # c40fffd3
fcvt.wu.h x0, f0            # c4107053
fcvt.wu.h x12, f23          # c41bf653
fcvt.wu.h x31, f31          # c41fffd3
fmv.x.h f0, x0              # e4000053
fmv.x.h f12, x23            # e40b8653
fmv.x.h f31, x31            # e40f8fd3
fcvt.h.w f0, x0             # d4007053
fcvt.h.w f12, x23           # d40bf653
fcvt.h.w f31, x31           # d40fffd3
fcvt.h.wu f0, x0            # d4107053
fcvt.h.wu f12, x23          # d41bf653
fcvt.h.wu f31, x31          # d41fffd3
fmv.h.x f0, x0              # f4000053
fmv.h.x f12, x23            # f40b8653
fmv.h.x f31, x31            # f40f8fd3
