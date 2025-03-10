/*
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>

SPDX-License-Identifier: MIT-0
*/

// https://www.codewars.com/kata/57f75cc397d62fc93d000059

package main

import (
	"fmt"
	"strconv"
	"strings"
)

func Calc(s string) int {
	runes := []rune(s)
	chars := []string{}
	for _, rune := range runes {
		chars = append(chars, fmt.Sprintf("%d", rune))
	}
	num := strings.Join(chars, "")
	sum := func(input string) int {
		total := 0
		for _, j := range strings.Split(input, "") {
			digit, _ := strconv.Atoi(j)
			total += digit
		}
		return total
	}
	return sum(num) - sum(strings.ReplaceAll(num, "7", "1"))
}

func main() {
	fmt.Println(Calc("ABC")) // 7
}
