/*
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>

SPDX-License-Identifier: MIT-0
*/

// https://www.codewars.com/kata/51f2d1cafc9c0f745c00037d

package main

import (
	"fmt"
	"strings"
)

func solution(str, ending string) bool {
	return strings.HasSuffix(str, ending)
}

func main() {
	fmt.Println(solution("abc", "bc")) // true
	fmt.Println(solution("abc", "d"))  // false
}
