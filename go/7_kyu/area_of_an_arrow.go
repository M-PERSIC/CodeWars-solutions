/*
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>

SPDX-License-Identifier: MIT-0
*/

// https://www.codewars.com/kata/589478160c0f8a40870000bc

package main

import "fmt"


func ArrowArea(a, b int) float64 {
	return float64(a*b) / 4
}

func main() {
	fmt.Println(ArrowArea(4, 2)) // 2
}