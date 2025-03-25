// SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
//
// SPDX-License-Identifier: MIT-0

// https://www.codewars.com/kata/5259b20d6021e9e14c0010d4

package main

import (
	"fmt"
	"strings"
)

func ReverseWords(str string) (output string) {
	for _, word := range strings.Split(str, " ") {
		runes := []rune(word)
		for index, _ := range runes[0 : len(runes)/2] {
			runes[index], runes[len(runes)-index-1] = runes[len(runes)-index-1], runes[index]
		}
		output += string(runes) + " "
	}
	return strings.TrimSpace(output)
}

func main() {
	fmt.Println(ReverseWords("This is an example!"))                          // sihT si na !elpmaxe
	fmt.Println(ReverseWords("The quick brown fox jumps over the lazy dog.")) // ehT kciuq nworb xof spmuj revo eht yzal .god
	fmt.Println(ReverseWords("apple"))                                        // elppa
	fmt.Println(ReverseWords("a b c d"))                                      // a b c d
	fmt.Println(ReverseWords("double  spaced  words"))                        // elbuod  decaps  sdrow
	fmt.Println(ReverseWords("stressed desserts"))                            // desserts stressed
	fmt.Println(ReverseWords("hello hello"))                                  // olleh olleh
}
