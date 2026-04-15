# Copyright James Wainwright
#
# SPDX-License-Identifier: MPL-2.0

c.lwsp x1, 0(x2)         # 4082
c.lwsp x9, 44(x2)        # 54b2
c.lwsp x31, 128(x2)      # 4f8a
c.swsp x1, 0(x2)         # c006
c.swsp x9, 44(x2)        # d626
c.swsp x31, 128(x2)      # c17e
c.lw x8, 0(x8)           # 4000
c.lw x10, 36(x13)        # 52c8
c.lw x15, 64(x15)        # 43bc
c.sw x8, 0(x8)           # c000
c.sw x10, 36(x13)        # d2c8
c.sw x15, 64(x15)        # c3bc
c.addi x1, 1             # 0085
c.addi x13, 27           # 06ed
c.addi x31, 31           # 0ffd
c.addi x31, -32          # 1f81
c.j 0                    # a001
c.j 484                  # a2d5
c.j -486                 # bd29
c.j 2046                 # affd
c.j -2048                # b001
c.jal 0                  # 2001
c.jal 484                # 22d5
c.jal -486               # 3d29
c.jal 2046               # 2ffd
c.jal -2048              # 3001
c.jr x1                  # 8082
c.jr x27                 # 8d82
c.jr x31                 # 8f82
c.jalr x1                # 9082
c.jalr x27               # 9d82
c.jalr x31               # 9f82
c.beqz x8, 0             # c001
c.beqz x9, 42            # c48d
c.beqz x10, -44          # d971
c.beqz x15, 254          # cffd
c.beqz x15, -256         # d381
c.bnez x8, 0             # e001
c.bnez x9, 42            # e48d
c.bnez x10, -44          # f971
c.bnez x15, 254          # effd
c.bnez x15, -256         # f381
c.li x1, 0               # 4081
c.li x13, 27             # 46ed
c.li x17, -27            # 5895
c.li x31, 31             # 4ffd
c.li x31, -32            # 5f81
c.lui x1, 1              # 6085
c.lui x13, 27            # 66ed
c.lui x31, 31            # 6ffd
c.lui x31, 1048575       # 7ffd
c.addi x1, 0             # 0081
c.addi x13, 13           # 06b5
c.addi x17, -13          # 18cd
c.addi x31, 31           # 0ffd
c.addi x31, -32          # 1f81
c.addi16sp x2, 16        # 6141
c.addi16sp x2, 48        # 6145
c.addi16sp x2, -512      # 7101
c.addi16sp x2, 496       # 617d
c.addi4spn x8, x2, 4     # 0040
c.addi4spn x12, x2, 248  # 19b0
c.addi4spn x15, x2, 1020 # 1ffc
c.slli x1, 1             # 0086
c.slli x13, 27           # 06ee
c.slli x31, 31           # 0ffe
c.srli x8, 1             # 8005
c.srli x9, 16            # 80c1
c.srli x15, 31           # 83fd
c.srai x8, 1             # 8405
c.srai x9, 16            # 84c1
c.srai x15, 31           # 87fd
c.andi x8, 0             # 8801
c.andi x10, 5            # 8915
c.andi x12, -5           # 9a6d
c.andi x15, 31           # 8bfd
c.andi x15, -32          # 9b81
c.mv x1, x1              # 8086
c.mv x5, x17             # 82c6
c.mv x31, x31            # 8ffe
c.add x1, x1             # 9086
c.add x5, x17            # 92c6
c.add x31, x31           # 9ffe
c.or x8, x8              # 8c41
c.or x12, x13            # 8e55
c.or x15, x15            # 8fdd
c.xor x8, x8             # 8c21
c.xor x12, x13           # 8e35
c.xor x15, x15           # 8fbd
c.sub x8, x8             # 8c01
c.sub x12, x13           # 8e15
c.sub x15, x15           # 8f9d
unimp                    # 0000
c.nop                    # 0001
c.ebreak                 # 9002
