# Copyright James Wainwright
#
# SPDX-License-Identifier: MPL-2.0

flw f0, 0(x0)               # 00002007
flw f13, -45(x7)            # fd33a687
flw f4, 612(x23)            # 264ba207
flw f15, 2047(x12)          # 7ff62787
flw f25, -2048(x16)         # 80082c87
flw f31, -1(x31)            # ffffaf87
fsw f0, 0(x0)               # 00002027
fsw f13, -45(x7)            # fcd3a9a7
fsw f4, 612(x23)            # 264ba227
fsw f15, 2047(x12)          # 7ef62fa7
fsw f25, -2048(x16)         # 81982027
fsw f31, -1(x31)            # ffffafa7
fmadd.s f0, f0, f0, f0      # 00007043
fmadd.s f13, f25, f3, f18   # 903cf6c3
fmadd.s f31, f31, f31, f31  # f9ffffc3
fmsub.s f0, f0, f0, f0      # 00007047
fmsub.s f13, f25, f3, f18   # 903cf6c7
fmsub.s f31, f31, f31, f31  # f9ffffc7
fnmsub.s f0, f0, f0, f0     # 0000704b
fnmsub.s f13, f25, f3, f18  # 903cf6cb
fnmsub.s f31, f31, f31, f31 # f9ffffcb
fnmadd.s f0, f0, f0, f0     # 0000704f
fnmadd.s f13, f25, f3, f18  # 903cf6cf
fnmadd.s f31, f31, f31, f31 # f9ffffcf
fadd.s f0, f0, f0           # 00007053
fadd.s f13, f26, f3         # 003d76d3
fadd.s f31, f31, f31        # 01ffffd3
fsub.s f0, f0, f0           # 08007053
fsub.s f13, f26, f3         # 083d76d3
fsub.s f31, f31, f31        # 09ffffd3
fmul.s f0, f0, f0           # 10007053
fmul.s f13, f26, f3         # 103d76d3
fmul.s f31, f31, f31        # 11ffffd3
fdiv.s f0, f0, f0           # 18007053
fdiv.s f13, f26, f3         # 183d76d3
fdiv.s f31, f31, f31        # 19ffffd3
fsqrt.s f0, f0              # 58007053
fsqrt.s f13, f26            # 580d76d3
fsqrt.s f31, f31            # 580fffd3
fsgnj.s f0, f0, f0          # 20000053
fsgnj.s f13, f26, f3        # 203d06d3
fsgnj.s f31, f31, f31       # 21ff8fd3
fsgnjn.s f0, f0, f0         # 20001053
fsgnjn.s f13, f26, f3       # 203d16d3
fsgnjn.s f31, f31, f31      # 21ff9fd3
fsgnjx.s f0, f0, f0         # 20002053
fsgnjx.s f13, f26, f3       # 203d26d3
fsgnjx.s f31, f31, f31      # 21ffafd3
fmin.s f0, f0, f0           # 28000053
fmin.s f13, f26, f3         # 283d06d3
fmin.s f31, f31, f31        # 29ff8fd3
fmax.s f0, f0, f0           # 28001053
fmax.s f13, f26, f3         # 283d16d3
fmax.s f31, f31, f31        # 29ff9fd3
fcvt.w.s x0, f0             # c0007053
fcvt.w.s x12, f23           # c00bf653
fcvt.w.s x31, f31           # c00fffd3
fcvt.wu.s x0, f0            # c0107053
fcvt.wu.s x12, f23          # c01bf653
fcvt.wu.s x31, f31          # c01fffd3
fmv.x.w x0, f0              # e0000053
fmv.x.w x12, f23            # e00b8653
fmv.x.w x31, f31            # e00f8fd3
feq.s x0, f0, f0            # a0002053
feq.s x13, f26, f3          # a03d26d3
feq.s x31, f31, f31         # a1ffafd3
flt.s x0, f0, f0            # a0001053
flt.s x13, f26, f3          # a03d16d3
flt.s x31, f31, f31         # a1ff9fd3
fle.s x0, f0, f0            # a0000053
fle.s x13, f26, f3          # a03d06d3
fle.s x31, f31, f31         # a1ff8fd3
fclass.s x0, f0             # e0001053
fclass.s x12, f23           # e00b9653
fclass.s x31, f31           # e00f9fd3
fcvt.s.w f0, x0             # d0007053
fcvt.s.w f12, x23           # d00bf653
fcvt.s.w f31, x31           # d00fffd3
fcvt.s.wu f0, x0            # d0107053
fcvt.s.wu f12, x23          # d01bf653
fcvt.s.wu f31, x31          # d01fffd3
fmv.w.x f0, x0              # f0000053
fmv.w.x f12, x23            # f00b8653
fmv.w.x f31, x31            # f00f8fd3
