// SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
//
// SPDX-License-Identifier: MIT-0

// https://www.codewars.com/kata/514b92a657cdc65150000006

package main

import "fmt"

func Multiple3And5(number int) (output int) {
	for i := 1; i < number; i++ {
		if i%5 == 0 || i%3 == 0 {
			output += i
		}
	}
	return
}

func main() {
	fmt.Println(Multiple3And5(10)) // 23
}
