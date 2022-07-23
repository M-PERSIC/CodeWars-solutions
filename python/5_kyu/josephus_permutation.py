"""
Michael Persico
Oct.22, 2021
Josephus Permutation
https://www.codewars.com/kata/5550d638a99ddb113e0000a2
"""

def josephus(items,k):
    result, position, list_length = [], 0, len(items)
    if (len(items) > 0):
        for i in range(1,list_length):
            position = (position + k - 1) % len(items)
            result.append(items.pop(position))
    return result + items

if __name__ == "__main__":
    print(josephus([1,2,3,4,5,6,7,8,9,10],1)) # [1,2,3,4,5,6,7,8,9,10]
    print(josephus(["C","o","d","e","W","a","r","s"],4)) # ['e', 's', 'W', 'o', 'C', 'd', 'r', 'a']
