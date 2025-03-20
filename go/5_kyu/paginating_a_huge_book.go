// SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
//
// SPDX-License-Identifier: MIT-0

// https://www.codewars.com/kata/55905b7597175ffc1a00005a

package main

import (
	"fmt"
	"strconv"
)

func PageDigits(pages uint64) uint64 {
	var total, left uint64 = 0, pages
	var group, count uint64 = 9, 1
	for i := 1; i < len([]rune(strconv.FormatUint(pages, 10))); i++ {
		left -= group
		total += group * count
		group *= 10
		count++
	}
	return total + left*count
}

func main() {
	fmt.Println(PageDigits(0))   // 0
	fmt.Println(PageDigits(12))  // 15
	fmt.Println(PageDigits(100)) // 192
}
