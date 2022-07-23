"""
Michael Persico
Sept.29, 2021
Moving Zeros To The End
https://www.codewars.com/kata/52597aa56021e91c93000cb0
"""

def move_zeros(array):
    noZeroes, Zeroes = [], []
    for digit in array:
        noZeroes.append(digit) if digit != 0 else Zeroes.append(digit)
    return noZeroes + Zeroes

if __name__ == "__main__":
    print(move_zeros([1,0,1,2,-1,3])) # [1,1,2,-1,3,0]

