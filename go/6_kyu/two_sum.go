// SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
//
// SPDX-License-Identifier: MIT-0

// https://www.codewars.com/kata/52c31f8e6605bcc646000082

package main

import (
	"fmt"
	"sort"
)

func TwoSum(numbers []int, target int) [2]int {
	table := make(map[int]int)
	for index, num := range numbers {
		complement := target - num
		if item, ok := table[complement]; ok {
			output := []int{index, item}
			sort.Ints(output)
			return [2]int(output)
		}
		table[num] = index
	}
	return [2]int{}
}

func main() {
	fmt.Println(TwoSum([]int{1, 2, 3}, 4))              // [2]int{0, 2}
	fmt.Println(TwoSum([]int{1234, 5678, 9012}, 14690)) // [2]int{1, 2}
	fmt.Println(TwoSum([]int{2, 2, 3}, 4))              // [2]int{0, 1}
}
