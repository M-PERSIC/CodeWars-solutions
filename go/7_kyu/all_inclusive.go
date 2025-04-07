// SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
//
// SPDX-License-Identifier: MIT-0

// https://www.codewars.com/kata/5700c9acc1555755be00027e

package main

import (
	"fmt"
	"strings"
)

func ContainAllRots(strng string, arr []string) bool {
	queue := strings.Split(strng, "")
	for _ = range queue {
		present := false
		for i := range arr {
			if strings.Join(queue, "") == arr[i] {
				present = true
				break
			}
		}
		if !present {
			return false
		}
		queue = append(queue[1:], queue[0])
	}
	return true
}

func main() {
	fmt.Println(ContainAllRots("bsjq", []string{"bsjq", "qbsj", "sjqb", "twZNsslC", "jqbs"}))                                                       // true
	fmt.Println(ContainAllRots("XjYABhR", []string{"TzYxlgfnhf", "yqVAuoLjMLy", "BhRXjYA", "YABhRXj", "hRXjYAB", "jYABhRX", "XjYABhR", "ABhRXjY"})) // false
	fmt.Println(ContainAllRots("", []string{}))                                                                                                     // true

}
