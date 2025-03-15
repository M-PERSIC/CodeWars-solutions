// SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
//
// SPDX-License-Identifier: MIT-0

// https://www.codewars.com/kata/5ce399e0047a45001c853c2b

package main

import "fmt"

func PartsSums(ls []uint64) []uint64 {
	output := make([]uint64, len(ls)+1)
	var sum uint64 = 0
	for _, i := range ls {
		sum += i
	}
	for j, k := range ls[:len(ls)] {
		output[j] = sum
		sum -= k
	}
	return output
}

func main() {
	fmt.Println(PartsSums([]uint64{0, 1, 3, 6, 10}))                                                  // {20, 20, 19, 16, 10, 0}
	fmt.Println(PartsSums([]uint64{1, 2, 3, 4, 5, 6}))                                                // {21, 20, 18, 15, 11, 6, 0}
	fmt.Println(PartsSums([]uint64{744125, 935, 407, 454, 430, 90, 144, 6710213, 889, 810, 2579358})) // {10037855, 9293730, 9292795, 9292388, 9291934, 9291504, 9291414, 9291270, 2581057, 2580168, 2579358, 00}
}
