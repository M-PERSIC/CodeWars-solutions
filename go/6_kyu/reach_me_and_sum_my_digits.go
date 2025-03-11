/*
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>

SPDX-License-Identifier: MIT-0
*/

// https://www.codewars.com/kata/55ffb44050558fdb200000a4

package main

import (
	"fmt"
	"strconv"
)

func SumDigNthTerm(initVal int, pattern []int, nth int) (output int) {
	seq := []int{initVal}
	for i := 0; i < nth; i++ {
		seq = append(seq, pattern...)
	}
	for j := 0; j < nth; j++ {
		switch j {
		case 0:
		default:
			seq[j] = seq[j-1] + seq[j]
		}
	}
	digits := len(strconv.Itoa(seq[nth-1]))
	for i := 0; i < digits; i++ {
		output += seq[nth-1] % 10
		seq[nth-1] /= 10
	}
	return
}

func main() {
	fmt.Println(SumDigNthTerm(10, []int{2, 1, 3}, 6))          // 10
	fmt.Println(SumDigNthTerm(10, []int{2, 1, 3}, 15))         // 10
	fmt.Println(SumDigNthTerm(10, []int{2, 1, 3}, 50))         // 9
	fmt.Println(SumDigNthTerm(10, []int{2, 1, 3}, 78))         // 10
	fmt.Println(SumDigNthTerm(10, []int{2, 1, 3}, 157))        // 7
	fmt.Println(SumDigNthTerm(10, []int{2, 2, 5, 8}, 6))       // 11
	fmt.Println(SumDigNthTerm(10, []int{2, 2, 5, 8}, 15))      // 11
	fmt.Println(SumDigNthTerm(10, []int{2, 2, 5, 8}, 50))      // 9
	fmt.Println(SumDigNthTerm(10, []int{2, 2, 5, 8}, 78))      // 11
	fmt.Println(SumDigNthTerm(10, []int{2, 2, 5, 8}, 157))     // 16
	fmt.Println(SumDigNthTerm(100, []int{2, 2, 5, 8}, 6))      // 11
	fmt.Println(SumDigNthTerm(100, []int{2, 2, 5, 8}, 15))     // 11
	fmt.Println(SumDigNthTerm(100, []int{2, 2, 5, 8}, 50))     // 9
	fmt.Println(SumDigNthTerm(100, []int{2, 2, 5, 8}, 78))     // 11
	fmt.Println(SumDigNthTerm(100, []int{2, 2, 5, 8}, 157))    // 16
	fmt.Println(SumDigNthTerm(1000, []int{2, 2, 5, 8}, 2550))  // 14
	fmt.Println(SumDigNthTerm(1000, []int{2, 2, 5, 8}, 25500)) // 26
}
