"""
Michael Persico
Oct.03, 2021
Break camelCase
https://www.codewars.com/kata/5208f99aee097e65520000148
"""

def solution(s):
    indexes = [index for index, char in enumerate(s) if char.isupper()]
    if (len(indexes) != 0):
       camel_case = ""
       ind = 0
       for index in indexes:   
           camel_case += s[ind:index] + " "
           ind = index
       return (camel_case + s[ind:]).strip()
    return s

print(solution("helloWorld")) # hello World
print(solution("camelCase")) # camel Case
print(solution("breakCamelCase")) # break Camel Case
