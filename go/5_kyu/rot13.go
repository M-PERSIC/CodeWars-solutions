// SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
//
// SPDX-License-Identifier: MIT-0

// https://www.codewars.com/kata/52223df9e8f98c7aa7000062

package main

import (
	"fmt"
)

func Rot13(msg string) (output string) {
	for _, rune := range msg {
		switch {
		case rune >= 'A' && rune+13 <= 'Z':
			output += string(rune + 13)
		case rune >= 'A' && rune <= 'Z' && rune+13 > 'Z':
			output += string(65 + (rune+13)%91)
		case rune >= 'a' && rune+13 <= 'z':
			output += string(rune + 13)
		case rune >= 'a' && rune <= 'z' && rune+13 > 'z':
			output += string(97 + (rune+13)%123)
		default:
			output += string(rune)
		}
	}
	return output
}

func main() {
	fmt.Println(Rot13("EBG13 rknzcyr."))                                                           // ROT13 example
	fmt.Println(Rot13("123"))                                                                      // 123
	fmt.Println(Rot13("Guvf vf npghnyyl gur svefg xngn V rire znqr. Gunaxf sbe svavfuvat vg! :)")) // This is actually the first kata I ever made. Thanks for finishing it! :)
	fmt.Println(Rot13("@[`{"))                                                                     // @[`{
}
