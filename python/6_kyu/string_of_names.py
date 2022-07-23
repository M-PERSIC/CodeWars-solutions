"""
Michael Persico
Oct.09, 2021
Format a string of names like 'Bart, Lisa & Maggie'.
https://www.codewars.com/kata/53368a47e38700bd8300030d
"""


def namelist(names):
    return ", ".join([name.get('name') for name in names[:-1]]) +  (" & " if len(names) > 1 else "") + names[-1].get('name') if (len(names) > 0) else ""

if __name__ == "__main__":
    print(namelist([ {'name': 'Bart'}, {'name': 'Lisa'}, {'name': 'Maggie'} ])) # Bart, Lisa & Maggie
   print(namelist([ {'name': 'Bart'}, {'name': 'Lisa'} ])) # Bat & Lisa


    
