"""
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>

SPDX-License-Identifier: MIT-0
"""

# https://www.codewars.com/kata/53368a47e38700bd8300030d

def namelist(names):
    return ", ".join([name.get('name') for name in names[:-1]]) +  (" & " if len(names) > 1 else "") + names[-1].get('name') if (len(names) > 0) else ""

if __name__ == "__main__":
    print(namelist([ {'name': 'Bart'}, {'name': 'Lisa'}, {'name': 'Maggie'} ])) # Bart, Lisa & Maggie
   print(namelist([ {'name': 'Bart'}, {'name': 'Lisa'} ])) # Bat & Lisa


    
