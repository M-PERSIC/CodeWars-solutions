"""
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>

SPDX-License-Identifier: MIT-0
"""

# https://www.codewars.com/kata/526dbd6c8c0eb53254000110

def alphanumeric(password):
    return all(char.isalnum() for char in password) and len(password) > 0

if __name__ == "__main__":
    print(alphanumeric("iW4l0fXC95JHZDurmMBnCKai8pIQ")) # True
