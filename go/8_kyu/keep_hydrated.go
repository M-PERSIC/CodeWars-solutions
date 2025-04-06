// SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
//
// SPDX-License-Identifier: MIT-0

// https://www.codewars.com/kata/582cb0224e56e068d800003c

package main

import "fmt"

func Litres(time float64) int {
	return int(time * 0.5)
}

func main() {
	fmt.Println(Litres(2))    // 1
	fmt.Println(Litres(1.4))  // 0
	fmt.Println(Litres(12.3)) // 6
	fmt.Println(Litres(0.82)) // 0
	fmt.Println(Litres(11.8)) // 5
	fmt.Println(Litres(1787)) // 893
	fmt.Println(Litres(0))    // 0
}
