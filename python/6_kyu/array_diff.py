"""
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>

SPDX-License-Identifier: MIT-0
"""

# https://www.codewars.com/kata/523f5d21c841566fde000009

def array_diff(a,b):
    return [value for value in a if value not in b]

if __name__ == "__main__":
    print(array_diff([1,2],[1])) # [2]
    print(array_diff([1,2,2],[1])) # [2,2]
    print(array_diff([1,2,2],[])) # [1,2,2]
    print(array_diff([],[1,2])) # []
    print(array_diff([1,2,3],[1,2])) # [3]

    
