"""
Michael Persico
Sept.29, 2021
Ones and Zeros
https://www.codewars.com/kata/578553c3a1b8d5c40300037c
"""

def binary_array_to_number(arr):
    return int("0b" + "".join([str(digit) for digit in arr]), 2)

if __name__ == "__main__":
    print(binary_array_to_number([0,0,0,1])) # 1
    print(binary_array_to_number([0,0,1,0])) # 2
    print(binary_array_to_number([0,1,0,1])) # 5
    print(binary_array_to_number([1,0,0,1])) # 9
    print(binary_array_to_number([0,1,1,0])) # 6
    print(binary_array_to_number([1,1,1,1])) # 15
    print(binary_array_to_number([1,0,1,1])) # 11
