"""
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>

SPDX-License-Identifier: MIT-0
"""

# https://www.codewars.com/546f922b54af40e1e90001da

import re

def alphabet_position(text):
    text = " ".join(re.findall("[a-zA-Z]+", text.lower()))
    positions = [ord(char) - 96 for char in text.replace(" ","")]
    return ' '.join(str(position) for position in positions).strip()
    

if __name__ == "__main__":
        print(alphabet_position("The sunset sets at twelve o' clock.")) # 20 8 5 19 21 14 19 5 20 19 5 20 19 1 20 20 23 5 12 22 5 15 3 12 15 3 11
        
