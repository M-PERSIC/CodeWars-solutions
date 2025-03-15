// SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
//
// SPDX-License-Identifier: MIT-0

// https://www.codewars.com/kata/51b6249c4612257ac0000005

package main

import "fmt"

func Decode(roman string) (output int) {
	numerals := map[rune]int{'I': 1, 'V': 5, 'X': 10, 'L': 50, 'C': 100, 'D': 500, 'M': 1000}
	letters := []rune(roman)
	for index, letter := range letters {
		if index+1 != len(roman) && numerals[letter] < numerals[letters[index+1]] {
			output -= numerals[letter]
		} else {
			output += numerals[letter]
		}
	}
	return
}

func main() {
	fmt.Println(Decode("XXI"))     // 21
	fmt.Println(Decode("I"))       // 1
	fmt.Println(Decode("IV"))      // 4
	fmt.Println(Decode("MMVIII"))  // 2008
	fmt.Println(Decode("MDCLXVI")) // 1666
}
