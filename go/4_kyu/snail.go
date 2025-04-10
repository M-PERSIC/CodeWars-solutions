// SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
//
// SPDX-License-Identifier: MIT-0

// https://www.codewars.com/kata/521c2db8ddc89b9b7a0000c1

package main

import "fmt"

func Snail(snaipMap [][]int) []int {
	output := []int{}
	if len(snaipMap) == 1 && len(snaipMap[0]) == 0 {
		return output
	}
	length, counter := len(snaipMap), 0
	for {
		if len(output) == length*length {
			return output
		}
		for top := counter; top < length-counter; top++ {
			output = append(output, snaipMap[counter][top])
		}
		for right := counter + 1; right < length-counter; right++ {
			output = append(output, snaipMap[right][length-counter-1])
		}
		for bottom := length - counter - 2; bottom >= counter; bottom-- {
			output = append(output, snaipMap[length-counter-1][bottom])
		}
		for left := length - counter - 2; left > counter; left-- {
			output = append(output, snaipMap[left][counter])
		}
		counter++
	}
}

func main() {
	fmt.Println(Snail([][]int{}))                                                       // []
	fmt.Println(Snail([][]int{{1, 2, 3}, {4, 5, 6}, {7, 8, 9}}))                        // 1 2 3 6 9 8 7 4 5
	fmt.Println(Snail([][]int{{1, 2, 3, 1}, {4, 5, 6, 4}, {7, 8, 9, 7}, {7, 8, 9, 7}})) // 1 2 3 1 4 7 7 9 8 7 7 4 5 6 9 8
}
