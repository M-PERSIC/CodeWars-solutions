"""
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>

SPDX-License-Identifier: MIT-0
"""

# https://www.codewars.com/kata/53dbd5315a3c69eed20002dd

def filter_list(l):
    return list(filter(lambda char: (type(char) != str), l))

if __name__ == "__main__":
    print(filter_list([1,2,'a','b'])) # [1,2]
    print(filter_list([1,'a','b',0.15])) # [1,0,15]
    print(filter_list([1,2,'aasf','1','123',123])) # [1,2,123]
