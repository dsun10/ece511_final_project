# Copyright James Wainwright
#
# SPDX-License-Identifier: MPL-2.0

csrrw x0, 0, x0      # 00001073
csrrw x3, 3023, x27  # bcfd91f3
csrrw x31, 4095, x31 # ffff9ff3
csrrs x0, 0, x0      # 00002073
csrrs x3, 3023, x27  # bcfda1f3
csrrs x31, 4095, x31 # ffffaff3
csrrc x0, 0, x0      # 00003073
csrrc x3, 3023, x27  # bcfdb1f3
csrrc x31, 4095, x31 # ffffbff3
csrrwi x0, 0, 0      # 00005073
csrrwi x3, 43, 13    # 02b6d1f3
csrrwi x31, 4095, 31 # ffffdff3
csrrsi x0, 0, 0      # 00006073
csrrsi x3, 43, 13    # 02b6e1f3
csrrsi x31, 4095, 31 # ffffeff3
csrrci x0, 0, 0      # 00007073
csrrci x3, 43, 13    # 02b6f1f3
csrrci x31, 4095, 31 # fffffff3
