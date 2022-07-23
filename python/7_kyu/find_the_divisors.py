"""
Michael Persico
Sept.29, 2021
Find the divisors!
https://www.codewars.com/kata/544aed4c4a30184e960010f4
"""

def divisors(integer):
    divisors = [x for x in range(2,integer) if integer % x == 0]
    return divisors if len(divisors) > 0 else "{} is prime".format(integer)
    
if __name__ == "__main__":
    print(divisors(15)) # [3,5]
    print(divisors(253)) # [11,23]
    print(divisors(24)) # [2,3,4,6,8,12]
    print(divisors(25)) # [5]
    print(divisors(13)) # 13 is prime
    print(divisors(3)) # 3 is prime
    print(divisors(29)) # 29 is prime
