// SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
//
// SPDX-License-Identifier: MIT-0

// https://www.codewars.com/kata/5ce9c1000bab0b001134f5af

package main

import "fmt"

func QuarterOf(month int) int {
	switch month {
	case 1, 2, 3:
		return 1
	case 4, 5, 6:
		return 2
	case 7, 8, 9:
		return 3
	case 10, 11, 12:
		return 4
	default:
		return -1
	}
}

func main() {
	fmt.Println(QuarterOf(2))  // 1
	fmt.Println(QuarterOf(6))  // 2
	fmt.Println(QuarterOf(11)) // 4
}