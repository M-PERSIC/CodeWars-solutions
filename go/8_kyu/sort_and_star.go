// SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
//
// SPDX-License-Identifier: MIT-0

// https://www.codewars.com/kata/57cfdf34902f6ba3d300001e

package main

import (
	"fmt"
	"sort"
)

func TwoSort(arr []string) (output string) {
	sort.Strings(arr)
	for _, char := range arr[0] {
		output += string(char) + "***"
	}
	return output[:len(output)-3]
}

func main() {
	fmt.Println(TwoSort([]string{"bitcoin", "take", "over", "the", "world", "maybe", "who", "knows", "perhaps"}))                         // b***i***t***c***o***i***n
	fmt.Println(TwoSort([]string{"turns", "out", "random", "test", "cases", "are", "easier", "than", "writing", "out", "basic", "ones"})) // a***r***e
	fmt.Println(TwoSort([]string{"lets", "talk", "about", "javascript", "the", "best", "language"}))                                      // a***b***o***u***t
	fmt.Println(TwoSort([]string{"i", "want", "to", "travel", "the", "world", "writing", "code", "one", "day"}))                          // c***o***d***e
	fmt.Println(TwoSort([]string{"Lets", "all", "go", "on", "holiday", "somewhere", "very", "cold"}))                                     // L***e***t***s

}
