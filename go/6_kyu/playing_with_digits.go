// SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
//
// SPDX-License-Identifier: MIT-0

// https://www.codewars.com/kata/5552101f47fc5178b1000050

package main

import (
	"fmt"
	"math"
	"strconv"
)

func DigPow(n, p int) int {
	digits, result := []rune(strconv.Itoa(n)), 0.0
	for index, digit := range digits {
		result += math.Pow(float64(digit-'0'), float64(p+index))
	}
	if int(result)%n == 0.0 {
		return int(result) / n
	} else {
		return -1
	}

}

func main() {
	fmt.Println(DigPow(89, 1))    // 1
	fmt.Println(DigPow(92, 1))    // -1
	fmt.Println(DigPow(695, 2))   // 2
	fmt.Println(DigPow(46288, 3)) // 51
}
