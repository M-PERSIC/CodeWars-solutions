"""
Michael Persico
Aug.02, 2022
Detect Pangram
https://www.codewars.com/kata/545cedaa9943f7fe7b000048
"""

import string 
def is_pangram(s):
    return all(char in s.lower() for  char in list(string.ascii_lowercase))
    
if __name__ == "__main__":
	print(is_pangram("The quick, brown fox jumps over the lazy dog!")) # True
	print(is_pangram("1bcdefghijklmnopqrstuvwxyz")) # False
