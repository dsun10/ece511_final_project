# Copyright James Wainwright
#
# SPDX-License-Identifier: MPL-2.0

flq f0, 0(x0)               # 00004007
flq f13, -45(x7)            # fd33c687
flq f4, 612(x23)            # 264bc207
flq f15, 2047(x12)          # 7ff64787
flq f25, -2048(x16)         # 80084c87
flq f31, -1(x31)            # ffffcf87
fsq f0, 0(x0)               # 00004027
fsq f13, -45(x7)            # fcd3c9a7
fsq f4, 612(x23)            # 264bc227
fsq f15, 2047(x12)          # 7ef64fa7
fsq f25, -2048(x16)         # 81984027
fsq f31, -1(x31)            # ffffcfa7
fmadd.q f0, f0, f0, f0      # 06007043
fmadd.q f13, f25, f3, f18   # 963cf6c3
fmadd.q f31, f31, f31, f31  # ffffffc3
fmsub.q f0, f0, f0, f0      # 06007047
fmsub.q f13, f25, f3, f18   # 963cf6c7
fmsub.q f31, f31, f31, f31  # ffffffc7
fnmsub.q f0, f0, f0, f0     # 0600704b
fnmsub.q f13, f25, f3, f18  # 963cf6cb
fnmsub.q f31, f31, f31, f31 # ffffffcb
fnmadd.q f0, f0, f0, f0     # 0600704f
fnmadd.q f13, f25, f3, f18  # 963cf6cf
fnmadd.q f31, f31, f31, f31 # ffffffcf
fadd.q f0, f0, f0           # 06007053
fadd.q f13, f26, f3         # 063d76d3
fadd.q f31, f31, f31        # 07ffffd3
fsub.q f0, f0, f0           # 0e007053
fsub.q f13, f26, f3         # 0e3d76d3
fsub.q f31, f31, f31        # 0fffffd3
fmul.q f0, f0, f0           # 16007053
fmul.q f13, f26, f3         # 163d76d3
fmul.q f31, f31, f31        # 17ffffd3
fdiv.q f0, f0, f0           # 1e007053
fdiv.q f13, f26, f3         # 1e3d76d3
fdiv.q f31, f31, f31        # 1fffffd3
fsqrt.q f0, f0              # 5e007053
fsqrt.q f13, f26            # 5e0d76d3
fsqrt.q f31, f31            # 5e0fffd3
fsgnj.q f0, f0, f0          # 26000053
fsgnj.q f13, f26, f3        # 263d06d3
fsgnj.q f31, f31, f31       # 27ff8fd3
fsgnjn.q f0, f0, f0         # 26001053
fsgnjn.q f13, f26, f3       # 263d16d3
fsgnjn.q f31, f31, f31      # 27ff9fd3
fsgnjx.q f0, f0, f0         # 26002053
fsgnjx.q f13, f26, f3       # 263d26d3
fsgnjx.q f31, f31, f31      # 27ffafd3
fmin.q f0, f0, f0           # 2e000053
fmin.q f13, f26, f3         # 2e3d06d3
fmin.q f31, f31, f31        # 2fff8fd3
fmax.q f0, f0, f0           # 2e001053
fmax.q f13, f26, f3         # 2e3d16d3
fmax.q f31, f31, f31        # 2fff9fd3
fcvt.s.q f0, f0             # 40307053
fcvt.s.q f12, f23           # 403bf653
fcvt.s.q f31, f31           # 403fffd3
fcvt.q.s f0, f0             # 46007053
fcvt.q.s f12, f23           # 460bf653
fcvt.q.s f31, f31           # 460fffd3
fcvt.d.q f0, f0             # 42307053
fcvt.d.q f12, f23           # 423bf653
fcvt.d.q f31, f31           # 423fffd3
fcvt.q.d f0, f0             # 46107053
fcvt.q.d f12, f23           # 461bf653
fcvt.q.d f31, f31           # 461fffd3
feq.q x0, f0, f0            # a6002053
feq.q x13, f26, f3          # a63d26d3
feq.q x31, f31, f31         # a7ffafd3
flt.q x0, f0, f0            # a6001053
flt.q x13, f26, f3          # a63d16d3
flt.q x31, f31, f31         # a7ff9fd3
fle.q x0, f0, f0            # a6000053
fle.q x13, f26, f3          # a63d06d3
fle.q x31, f31, f31         # a7ff8fd3
fclass.q x0, f0             # e6001053
fclass.q x12, f23           # e60b9653
fclass.q x31, f31           # e60f9fd3
fcvt.w.q x0, f0             # c6007053
fcvt.w.q x12, f23           # c60bf653
fcvt.w.q x31, f31           # c60fffd3
fcvt.wu.q x0, f0            # c6107053
fcvt.wu.q x12, f23          # c61bf653
fcvt.wu.q x31, f31          # c61fffd3
fcvt.q.w f0, x0             # d6007053
fcvt.q.w f12, x23           # d60bf653
fcvt.q.w f31, x31           # d60fffd3
fcvt.q.wu f0, x0            # d6107053
fcvt.q.wu f12, x23          # d61bf653
fcvt.q.wu f31, x31          # d61fffd3
