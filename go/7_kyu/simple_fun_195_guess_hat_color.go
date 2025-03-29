// SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
//
// SPDX-License-Identifier: MIT-0

// https://www.codewars.com/kata/58c21c4ff130b7cab400009e

package main

import "fmt"

func GuessHatColor(a, b, c, d string) int {
	if (b == "black" && c == "black") || (b == "white" && c == "white") {
		return 1
	} else {
		return 2
	}
}

func main() {
	fmt.Println(GuessHatColor("white", "black", "white", "black")) // 2
	fmt.Println(GuessHatColor("white", "black", "black", "white")) // 1
}
