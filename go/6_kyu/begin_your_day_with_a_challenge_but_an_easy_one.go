// SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
//
// SPDX-License-Identifier: MIT-0

// https://www.codewars.com/kata/5873b2010565844b9100026d/train/go

package main

import (
	"fmt"
	"math"
	"strconv"
	"strings"
)

func OneTwoThreeSol(n int) [2]string {
	output := [2]string{"0", "0"}
	if n > 0 {
		for ; n > 9 || n >= 1; n -= 9 {
			output[0] += strconv.Itoa(int(math.Min(float64(n), 9.0)))
			output[1] += strings.Repeat("1", int(math.Min(float64(n), 9.0)))
		}
		output[0], output[1] = output[0][1:], output[1][1:]
	}
	return output
}

func main() {
	fmt.Println(OneTwoThreeSol(0))  // [0,0]
	fmt.Println(OneTwoThreeSol(1))  // [1,1]
	fmt.Println(OneTwoThreeSol(2))  // [2,11]
	fmt.Println(OneTwoThreeSol(3))  // [3,111]
	fmt.Println(OneTwoThreeSol(19)) // [991,1111111111111111111]
}
