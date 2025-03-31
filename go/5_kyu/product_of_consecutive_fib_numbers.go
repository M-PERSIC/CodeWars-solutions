// SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
//
// SPDX-License-Identifier: MIT-0

// https://www.codewars.com/kata/5541f58a944b85ce6d00006a

package main

import "fmt"

func ProductFib(prod uint64) [3]uint64 {
	var a, b uint64 = 0, 1
	for {
		if a*b < prod {
			a, b = b, a+b
		} else if a*b == prod {
			return [3]uint64{a, b, 1}
		} else {
			return [3]uint64{a, b, 0}
		}
	}
}

func main() {
	fmt.Println(ProductFib(714))      // [21, 34, 1]
	fmt.Println(ProductFib(800))      // [34, 55, 0]
	fmt.Println(ProductFib(4895))     // [55, 89, 1]
	fmt.Println(ProductFib(5895))     // [89, 144, 1]
	fmt.Println(ProductFib(74049690)) // [6765, 10946, 1]
	fmt.Println(ProductFib(84049690)) // [10946, 17711, 0]
}
