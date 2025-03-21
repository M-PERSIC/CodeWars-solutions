// SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
//
// SPDX-License-Identifier: MIT-0

// https://www.codewars.com/kata/54b724efac3d5402db00065e
// NOTE: Requires the MORSE_CODE map

package main

import (
	"fmt"
	"strings"
)

func DecodeMorse(morseCode string) string {
	translation := ""
	for _, code := range strings.Split(morseCode, " ") {
		if code == "" {
			translation += " "
		} else {
			translation += MORSE_CODE[code]
		}
	}
	return strings.TrimSpace(strings.Replace(translation, "  ", " ", -1))
}

func main() {
	fmt.Println(DecodeMorse(".... . -.--   .--- ..- -.. .")) // HEY JUDE
}
