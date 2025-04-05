// SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
//
// SPDX-License-Identifier: MIT-0

// https://www.codewars.com/kata/58261acb22be6e2ed800003a

package main

import "fmt"

func GetVolumeOfCuboid(length, width, height float64) float64 {
	return length * width * height
}

func main() {
	fmt.Println(GetVolumeOfCuboid(1.0, 2.0, 2.0)) // 4
}
