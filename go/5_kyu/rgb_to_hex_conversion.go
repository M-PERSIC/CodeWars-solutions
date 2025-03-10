/*
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>

SPDX-License-Identifier: MIT-0
*/

// https://www.codewars.com/kata/513e08acc600c94f01000001

packa
package main

import (
	"fmt"
)

func RGB(r, g, b int) string {
	convert := func(num int) string {
		switch {
		case num < 0:
			return "00"
		case num > 255:
			return "FF"
		default:
			return fmt.Sprintf("%02X", num)
		}
	}
	return convert(r) + convert(g) + convert(b)
}

func main() {
	fmt.Println(RGB(0, 0, 0))       // 000000
	fmt.Println(RGB(1, 2, 3))       // 010203
	fmt.Println(RGB(255, 255, 255)) // FFFFFF
	fmt.Println(RGB(254, 253, 252)) // FEFDFC
	fmt.Println(RGB(-20, 275, 125)) //00FF7D
}
