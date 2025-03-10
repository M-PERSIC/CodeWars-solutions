/*
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>

SPDX-License-Identifier: MIT-0
*/

// https://www.codewars.com/kata/5544c7a5cb454edb3c000047

package main

import "fmt"

func BouncingBall(h, bounce, window float64) int {
	result := -1
	switch {
	case h <= 0 || bounce <= 0 || bounce >= 1 || window >= h:
	default:
		result = 0
		h *= bounce
		for {
			if h > window {
				result += 2
				h *= bounce
			} else {
				result += 1
				break
			}
		}
	}
	return result
}

func main() {
	fmt.Println(BouncingBall(3, 0.66, 1.5)) // 3
	fmt.Println(BouncingBall(40, 0.4, 10))  // 3
	fmt.Println(BouncingBall(40, 1, 10))    // -1
	fmt.Println(BouncingBall(5, -1, 1.5))   // -1
	fmt.Println(BouncingBall(10, 0.6, 10))  // -1

}
