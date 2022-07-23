"""
Michael Persico
Sept.29, 2021
Friend or Foe?
https://www.codewars.com/kata/55b42574ff091733d900002f
"""

def friend(x):
    return [friend for friend in x if len(friend) == 4 and friend.isalpha()]
    
if __name__ == "__main__":
    print(friend(["Ryan", "Kieran", "Mark"])) # ["Ryan", "Mark"]
    print(friend(["Ryan", "Jimmy", "123", "4", "Cool Man"])) # ["Ryan"]
    print(friend(["Jimm", "Cari", "aret", "truehdnviegkwgvke", "sixtyiscooooool"])) # ["Jimm", "Cari", "aret"]
