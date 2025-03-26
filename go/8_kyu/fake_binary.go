// SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
//
// SPDX-License-Identifier: MIT-0

// https://www.codewars.com/kata/57eae65a4321032ce000002d

package main

import (
	"fmt"
	"strings"
)

func FakeBin(x string) string {
	Switch := func(r rune) rune {
		switch {
		case r < 53:
			return '0'
		default:
			return '1'
		}
	}
	return strings.Map(Switch, x)
}

func main() {
	fmt.Println(FakeBin("45385593107843568"))           // 01011110001100111
	fmt.Println(FakeBin("509321967506747"))             // 101000111101101
	fmt.Println(FakeBin("366058562030849490134388085")) // 011011110000101010000011011
	fmt.Println(FakeBin("15889923"))                    // 01111100
	fmt.Println(FakeBin("800857237867"))                // 100111001111
}
