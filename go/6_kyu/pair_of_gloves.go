// SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
//
// SPDX-License-Identifier: MIT-0

// https://www.codewars.com/kata/58235a167a8cb37e1a0000db

package main

import (
	"fmt"
)

func NumberOfPairs(gloves []string) (output int) {
	colors := make(map[string]int)
	for _, color := range gloves {
		colors[color] += 1
	}
	for _, num := range colors {
		output += num / 2
	}
	return
}

func main() {
	fmt.Println(NumberOfPairs([]string{"red", "red"}))                                                        // 1
	fmt.Println(NumberOfPairs([]string{"red", "green", "blue"}))                                              // 0
	fmt.Println(NumberOfPairs([]string{"gray", "black", "purple", "purple", "gray", "black"}))                // 3
	fmt.Println(NumberOfPairs([]string{}))                                                                    // 0
	fmt.Println(NumberOfPairs([]string{"red", "green", "blue", "blue", "red", "green", "red", "red", "red"})) // 4
}
