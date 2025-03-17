// SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
//
// SPDX-License-Identifier: MIT-0

// https://www.codewars.com/kata/65eb2c4c274bd91c27b38d32

package main

import (
	"fmt"
	"math"
)

func Solution(n int, m int) (output []int) {
	output = []int{}
	max := int(math.Sqrt(math.Sqrt(float64(m)))) - 1
	numbers := make([]int, max)
	primes := make([]bool, max)
	for i := 0; i < max; i++ {
		numbers[i] = i + 2
	}
	for j, num := range numbers {
		if !primes[j] {
			for k := j + num; k < len(numbers); k += num {
				primes[k] = true
			}
		}
	}
	for l := range primes {
		if !primes[l] {
			result := numbers[l] * numbers[l] * numbers[l] * numbers[l]
			if result >= n {
				output = append(output, result)
			}
		}
	}
	return

}

func main() {
	fmt.Println(Solution(2, 20))         // [16]
	fmt.Println(Solution(2, 100))        // [16, 81]
	fmt.Println(Solution(10000, 100000)) // [14641 28561 83521]
}
