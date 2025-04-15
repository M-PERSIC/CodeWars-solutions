// SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
//
// SPDX-License-Identifier: MIT-0

// https://www.codewars.com/kata/585d7d5adb20cf33cb000235

package main

import (
	"fmt"
	"sort"
)

func FindUniq(arr []float32) float32 {
	sort.Slice(arr, func(i, j int) bool { return arr[i] < arr[j] })
	for i, elem := range arr {
		if i-1 >= 0 && i+1 < len(arr) && arr[i-1] != elem && arr[i+1] != elem {
			return elem
		} else if i-1 < 0 && i+1 < len(arr) && arr[i+1] != elem {
			return elem
		} else if i-1 >= 0 && i+1 == len(arr) && arr[i-1] != elem {
			return elem
		}
	}
	return -1.0
}

func main() {
	fmt.Println(FindUniq([]float32{1.0, 1.0, 1.0, 2.0, 1.0, 1.0})) // 2
	fmt.Println(FindUniq([]float32{0, 0, 0.55, 0, 0}))             // 0.55
}
