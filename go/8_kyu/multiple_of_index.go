// SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
//
// SPDX-License-Identifier: MIT-0

// https://www.codewars.com/kata/5a34b80155519e1a00000009

package main

import "fmt"

func multipleOfIndex(ints []int) []int {
	output := []int{}
	for i, j := range ints {
		if i != 0 && j%i == 0 {
			output = append(output, j)
		}
	}
	return output
}

func main() {
	fmt.Println(multipleOfIndex([]int{22, -6, 32, 82, 9, 25}))                                                         // (-6, 32, 25)
	fmt.Println(multipleOfIndex([]int{68, -1, 1, -7, 10, 10}))                                                         // (-1, 10)
	fmt.Println(multipleOfIndex([]int{11, -11}))                                                                       // (-11)
	fmt.Println(multipleOfIndex([]int{-56, -85, 72, -26, -14, 76, -27, 72, 35, -21, -67, 87, 0, 21, 59, 27, -92, 68})) // (-85, 72, 0, 68)
	fmt.Println(multipleOfIndex([]int{28, 38, -44, -99, -13, -54, 77, -51}))                                           // (38, -44, -99)
	fmt.Println(multipleOfIndex([]int{-1, -49, -1, 67, 8, -60, 39, 35}))                                               // (-49, 8, -60, 35)
}
