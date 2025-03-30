// SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
//
// SPDX-License-Identifier: MIT-0

// https://www.codewars.com/kata/576400f2f716ca816d001614

package main

import "fmt"

func ReduceFraction(fraction [2]int) [2]int {
	numerator, denominator := fraction[0], fraction[1]
	divisor := 0
	for {
		if numerator == denominator {
			divisor = numerator
			break
		} else if numerator < denominator {
			denominator = denominator - numerator
		} else {
			numerator = numerator - denominator
		}
	}
	fraction[0], fraction[1] = fraction[0]/divisor, fraction[1]/divisor
	return fraction
}

func main() {
	fmt.Println(ReduceFraction([2]int{45, 120})) // [3, 8]
	fmt.Println(ReduceFraction([2]int{60, 120})) // [3, 1]
	fmt.Println(ReduceFraction([2]int{80, 120})) // [2, 3]
	fmt.Println(ReduceFraction([2]int{4, 2}))    // [2, 1]
	fmt.Println(ReduceFraction([2]int{1000, 1})) // [1000, 1]

}
