"""
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>

SPDX-License-Identifier: MIT-0
"""

# https://www.codewars.com/kata/551dc350bf4e526099000ae5

def song_decoder(song):
    return " ".join([word for word in song.split("WUB") if len(word) != 0])
    
if __name__ == "__main__":
    print(song_decoder("AWUBBWUBC")) # A B C
    print(song_decoder("AWUBWUBWUBBWUBWUBWUBC")) # A B C
    print(song_decoder("WUBAWUBBWUBCWUB")) # A B C

