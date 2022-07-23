"""
Michael Persico
Oct.17, 2021
Not very secure 
https://www.codewars.com/kata/526dbd6c8c0eb53254000110
"""

def alphanumeric(password):
    return all(char.isalnum() for char in password) and len(password) > 0

if __name__ == "__main__":
    print(alphanumeric("iW4l0fXC95JHZDurmMBnCKai8pIQ")) # True
