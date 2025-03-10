"""
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>

SPDX-License-Identifier: MIT-0
"""

# https://www.codewars.com/kata/52449b062fb80683ec000024

def generate_hashtag(s):
    if (len(s) != 0):
            word_list = s.strip().split(" ")
            hashtag = "#" + "".join([word.capitalize() for word in word_list])
            return hashtag if len(hashtag) <= 140 else False
    return False

if __name__ == "__main__":
    print(generate_hashtag("codewars is nice")) # #CodeWarsIsNice

