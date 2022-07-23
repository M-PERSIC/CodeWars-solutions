"""
Michael Persico
Oct.12, 2021
The Hashtag Generator
https://www.codewars.com/kata/52449b062fb80683ec000024
"""

def generate_hashtag(s):
    if (len(s) != 0):
            word_list = s.strip().split(" ")
            hashtag = "#" + "".join([word.capitalize() for word in word_list])
            return hashtag if len(hashtag) <= 140 else False
    return False

if __name__ == "__main__":
    print(generate_hashtag("codewars is nice")) # #CodeWarsIsNice

