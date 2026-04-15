# Copyright James Wainwright
#
# SPDX-License-Identifier: MPL-2.0

lui x0, 0            # 00000037
lui x1, 123          # 0007b0b7
lui x31, 1048575     # ffffffb7
auipc x0, 0          # 00000017
auipc x1, 123        # 0007b097
auipc x31, 1048575   # ffffff97
jal x0, 0            # 0000006f
jal x1, -2           # fffff0ef
jal x31, 10          # 00a00fef
jal x31, 1048574     # 7fffffef
jalr x0, 0(x0)       # 00000067
jalr x1, -2(x31)     # ffef80e7
jalr x5, 10(x27)     # 00ad82e7
jalr x31, 2047(x31)  # 7fff8fe7
beq x0, x0, 0        # 00000063
beq x1, x31, -2      # fff08fe3
beq x5, x27, 10      # 01b28563
beq x31, x31, 2046   # 7fff8f63
bne x0, x0, 0        # 00001063
bne x1, x31, -2      # fff09fe3
bne x5, x27, 10      # 01b29563
bne x31, x31, 2046   # 7fff9f63
blt x0, x0, 0        # 00004063
blt x1, x31, -2      # fff0cfe3
blt x5, x27, 10      # 01b2c563
blt x31, x31, 2046   # 7fffcf63
bge x0, x0, 0        # 00005063
bge x1, x31, -2      # fff0dfe3
bge x5, x27, 10      # 01b2d563
bge x31, x31, 2046   # 7fffdf63
bltu x0, x0, 0       # 00006063
bltu x1, x31, -2     # fff0efe3
bltu x5, x27, 10     # 01b2e563
bltu x31, x31, 2046  # 7fffef63
bgeu x0, x0, 0       # 00007063
bgeu x1, x31, -2     # fff0ffe3
bgeu x5, x27, 10     # 01b2f563
bgeu x31, x31, 2046  # 7fffff63
lb x0, 0(x0)         # 00000003
lb x1, -2(x31)       # ffef8083
lb x5, 10(x27)       # 00ad8283
lb x31, 2047(x31)    # 7fff8f83
lh x0, 0(x0)         # 00001003
lh x1, -2(x31)       # ffef9083
lh x5, 10(x27)       # 00ad9283
lh x31, 2047(x31)    # 7fff9f83
lw x0, 0(x0)         # 00002003
lw x1, -2(x31)       # ffefa083
lw x5, 10(x27)       # 00ada283
lw x31, 2047(x31)    # 7fffaf83
lbu x0, 0(x0)        # 00004003
lbu x1, -2(x31)      # ffefc083
lbu x5, 10(x27)      # 00adc283
lbu x31, 2047(x31)   # 7fffcf83
lhu x0, 0(x0)        # 00005003
lhu x1, -2(x31)      # ffefd083
lhu x5, 10(x27)      # 00add283
lhu x31, 2047(x31)   # 7fffdf83
sb x0, 0(x0)         # 00000023
sb x1, -2(x31)       # fe1f8f23
sb x5, 10(x27)       # 005d8523
sb x31, 2047(x31)    # 7fff8fa3
sh x0, 0(x0)         # 00001023
sh x1, -2(x31)       # fe1f9f23
sh x5, 10(x27)       # 005d9523
sh x31, 2047(x31)    # 7fff9fa3
sw x0, 0(x0)         # 00002023
sw x1, -2(x31)       # fe1faf23
sw x5, 10(x27)       # 005da523
sw x31, 2047(x31)    # 7fffafa3
addi x0, x0, 0       # 00000013
addi x1, x31, -2     # ffef8093
addi x5, x27, 10     # 00ad8293
addi x31, x31, 2047  # 7fff8f93
slti x0, x0, 0       # 00002013
slti x1, x31, -2     # ffefa093
slti x5, x27, 10     # 00ada293
slti x31, x31, 2047  # 7fffaf93
sltiu x0, x0, 0      # 00003013
sltiu x1, x31, -2    # ffefb093
sltiu x5, x27, 10    # 00adb293
sltiu x31, x31, 2047 # 7fffbf93
xori x0, x0, 0       # 00004013
xori x1, x31, -2     # ffefc093
xori x5, x27, 10     # 00adc293
xori x31, x31, 2047  # 7fffcf93
ori x0, x0, 0        # 00006013
ori x1, x31, -2      # ffefe093
ori x5, x27, 10      # 00ade293
ori x31, x31, 2047   # 7fffef93
andi x0, x0, 0       # 00007013
andi x1, x31, -2     # ffeff093
andi x5, x27, 10     # 00adf293
andi x31, x31, 2047  # 7fffff93
slli x0, x0, 0       # 00001013
slli x5, x27, 10     # 00ad9293
slli x31, x31, 31    # 01ff9f93
srli x0, x0, 0       # 00005013
srli x5, x27, 10     # 00add293
srli x31, x31, 31    # 01ffdf93
srai x0, x0, 0       # 40005013
srai x5, x27, 10     # 40add293
srai x31, x31, 31    # 41ffdf93
add x0, x0, x0       # 00000033
add x1, x31, x13     # 00df80b3
add x5, x27, x31     # 01fd82b3
add x31, x31, x31    # 01ff8fb3
sub x0, x0, x0       # 40000033
sub x1, x31, x13     # 40df80b3
sub x5, x27, x31     # 41fd82b3
sub x31, x31, x31    # 41ff8fb3
sll x0, x0, x0       # 00001033
sll x1, x31, x13     # 00df90b3
sll x5, x27, x31     # 01fd92b3
sll x31, x31, x31    # 01ff9fb3
slt x0, x0, x0       # 00002033
slt x1, x31, x13     # 00dfa0b3
slt x5, x27, x31     # 01fda2b3
slt x31, x31, x31    # 01ffafb3
sltu x0, x0, x0      # 00003033
sltu x1, x31, x13    # 00dfb0b3
sltu x5, x27, x31    # 01fdb2b3
sltu x31, x31, x31   # 01ffbfb3
xor x0, x0, x0       # 00004033
xor x1, x31, x13     # 00dfc0b3
xor x5, x27, x31     # 01fdc2b3
xor x31, x31, x31    # 01ffcfb3
srl x0, x0, x0       # 00005033
srl x1, x31, x13     # 00dfd0b3
srl x5, x27, x31     # 01fdd2b3
srl x31, x31, x31    # 01ffdfb3
sra x0, x0, x0       # 40005033
sra x1, x31, x13     # 40dfd0b3
sra x5, x27, x31     # 41fdd2b3
sra x31, x31, x31    # 41ffdfb3
or x0, x0, x0        # 00006033
or x1, x31, x13      # 00dfe0b3
or x5, x27, x31      # 01fde2b3
or x31, x31, x31     # 01ffefb3
and x0, x0, x0       # 00007033
and x1, x31, x13     # 00dff0b3
and x5, x27, x31     # 01fdf2b3
and x31, x31, x31    # 01ffffb3
fence r, r           # 0220000f
fence w, w           # 0110000f
fence w, r           # 0120000f
fence r, w           # 0210000f
fence r, rw          # 0230000f
fence rw, w          # 0310000f
fence rw, rw         # 0330000f
fence o, r           # 0420000f
fence r, o           # 0240000f
fence i, r           # 0820000f
fence r, i           # 0280000f
fence io, r          # 0c20000f
fence r, io          # 02c0000f
fence iorw, iorw     # 0ff0000f
ecall                # 00000073
ebreak               # 00100073
