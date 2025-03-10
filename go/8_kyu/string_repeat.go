/*
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>

SPDX-License-Identifier: MIT-0
*/

// https://www.codewars.com/kata/57a0e5c372292dd76d000d7e

package main

import ( 
	"strings"
	"fmt"
)
func RepeatStr(repetitions int, value string) string {
  return strings.Repeat(value, repetitions)
}

func main() {
	fmt.Println(RepeatStr(5, "Hello")) // HelloHelloHelloHelloHello
	fmt.Println(RepeatStr(6, "I")) // IIIIII
}