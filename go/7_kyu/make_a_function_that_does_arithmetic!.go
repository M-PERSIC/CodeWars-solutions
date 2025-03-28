// SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
//
// SPDX-License-Identifier: MIT-0

// https://www.codewars.com/kata/583f158ea20cfcbeb400000a

package main

import "fmt"

func Arithmetic(a int, b int, operator string) int {
	switch operator {
	case "add":
		return a + b
	case "subtract":
		return a - b
	case "multiply":
		return a * b
	default:
		return a / b
	}
}

func main() {
	fmt.Println(Arithmetic(8, 2, "add"))      // 10
	fmt.Println(Arithmetic(8, 2, "subtract")) // 6
	fmt.Println(Arithmetic(8, 2, "multiply")) // 16
	fmt.Println(Arithmetic(8, 2, "divide"))   // 4

}
