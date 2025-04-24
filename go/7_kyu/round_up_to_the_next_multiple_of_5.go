// SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
//
// SPDX-License-Identifier: MIT-0

// https://www.codewars.com/kata/55d1d6d5955ec6365400006d

package main

import (
	"fmt"
	"math"
)

func RoundToNext5(n int) int {
	return int(math.Ceil(float64(n)/5) * 5)
}

func main() {
	fmt.Println(RoundToNext5(0))  // 0
	fmt.Println(RoundToNext5(2))  // 5
	fmt.Println(RoundToNext5(3))  // 5
	fmt.Println(RoundToNext5(12)) // 15
	fmt.Println(RoundToNext5(21)) // 25
	fmt.Println(RoundToNext5(30)) // 30
	fmt.Println(RoundToNext5(30)) // 30
	fmt.Println(RoundToNext5(-2)) // 0
	fmt.Println(RoundToNext5(-5)) // -5
}
