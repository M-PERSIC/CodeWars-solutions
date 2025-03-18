// SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
//
// SPDX-License-Identifier: MIT-0

// https://www.codewars.com/kata/59321f29a010d5aa80000066

package main

import (
	"fmt"
	"sort"
)

func ShortestArrang(n int) []int {
	sequence := []int{}
	cases := [][]int{}
	for i := 1; i <= n; i++ {
		sequence = append(sequence, i)
	}
	for j := 0; j < n-1; j++ {
		sum := 0
		for k := j; k < n-1; k++ {
			sum += sequence[k]
			if sum == n {
				cases = append(cases, sequence[j:k+1])
				break
			} else if sum > n {
				break
			}
		}
	}
	if len(cases) == 0 {
		return []int{-1}
	}
	length, index := n, 0
	for l, m := range cases {
		if len(m) < length {
			length = len(m)
			index = l
		}
	}
	sort.Sort(sort.Reverse(sort.IntSlice(cases[index])))
	return cases[index]
}

func main() {
	fmt.Println(ShortestArrang(10)) // [4, 3, 2, 1]
	fmt.Println(ShortestArrang(14)) // [5, 4, 3, 2]
	fmt.Println(ShortestArrang(16)) // [-1]
	fmt.Println(ShortestArrang(22)) // [7, 6, 5, 4]
	fmt.Println(ShortestArrang(65)) // [33, 32]
	fmt.Println(ShortestArrang(18)) // [7,6,5]
}
