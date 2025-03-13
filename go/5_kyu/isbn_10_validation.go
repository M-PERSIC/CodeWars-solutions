/*
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>

SPDX-License-Identifier: MIT-0
*/

// https://www.codewars.com/kata/51fc12de24a9d8cb0e000001

package main

import (
	"fmt"
	"strconv"
	"unicode"
)

func ValidISBN10(isbn string) bool {
	if len(isbn) < 10 {
		return false
	}
	characters := []rune(isbn)
	result := 0
	for index, character := range characters {
		switch {
		case unicode.IsDigit(character):
			num, _ := strconv.Atoi(string(character))
			result += num * (index + 1)
		case unicode.ToUpper(character) == 'X' && index+1 == len(characters):
			result += 10 * (index + 1)
		default:
			return false
		}
	}
	return result%11 == 0
}

func main() {
	fmt.Println(ValidISBN10("1112223339")) // true
	fmt.Println(ValidISBN10("048665088X")) // true
	fmt.Println(ValidISBN10("1293000000")) // true
	fmt.Println(ValidISBN10("1234554321")) // true
	fmt.Println(ValidISBN10("1234512345")) // false
	fmt.Println(ValidISBN10("1293"))       // false
	fmt.Println(ValidISBN10("X123456788")) // false
	fmt.Println(ValidISBN10("ABCDEFGHIJ")) // false
	fmt.Println(ValidISBN10("XXXXXXXXXX")) // false
}
