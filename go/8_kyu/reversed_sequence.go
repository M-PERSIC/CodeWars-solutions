/*
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>

SPDX-License-Identifier: MIT-0
*/

// https://www.codewars.com/kata/5a00e05cc374cb34d100000d

package main

import "fmt"

func ReverseSeq(n int) (output []int) {
	output = make([]int, n)
	for i := n; i > 0; i-- {
		output[n-i] = i
	}
	return
}

func main() {
	fmt.Println(ReverseSeq(5)) // {5,4,3,2,1}
}