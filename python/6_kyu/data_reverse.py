"""
Michael Persico
Oct.09, 2021
Data Reverse
https://www.codewars.com/kata/569d488d61b812a0f7000015
"""

def data_reverse(data):
    return sum([data[i:i + 8] for i in range(0,len(data), 8)][::-1], [])

if __name__ == "__main__":
    print(data_reverse([1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,1,1,0,1,0,1,0,1,0])) # [1,0,1,0,1,0,1,0,0,0,0,0,1,1,1,1,0,0,0,0,0,0,0,0,1,1,1,1,1,1,1,1]
