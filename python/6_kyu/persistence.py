"""
Michael Persico
Oct.02, 2021
Persistent Bugger.
https://www.codewars.com/kata/55bf01e5a717a0d57e0000ec
"""


import math

def persistence(n):
    n_str, count = str(n), 0
    while (int(n_str) >= 10):
        n_str = str(math.prod([int(digit) for digit in n_str]))
        count += 1
    return count
    
if __name__ == "__main__":
    print(persistence(39)) # 3
    print(persistence(4)) # 0
    print(persistence(25)) # 2
    print(persistence(999)) # 4
