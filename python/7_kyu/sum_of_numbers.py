"""
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>

SPDX-License-Identifier: MIT-0
"""

# https://www.codewars.com/kata/55f2b110f61eb0177000053

def get_sum(a,b):
    return sum(range(min(a,b), max(a,b) + 1))
    
if __name__ == "__main__":
    print(get_sum(1,0)) # 1
    print(get_sum(1,2)) # 3
    print(get_sum(0,1)) # 1
    print(get_sum(1,1)) # 1
    print(get_sum(-1,0)) # -1
    print(get_sum(-1,2)) # 2
