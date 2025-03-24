// SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
//
// SPDX-License-Identifier: MIT-0

// https://www.codewars.com/kata/526989a41034285187000de4

package main

import (
	"fmt"
	"math"
	"strconv"
	"strings"
)

func IpsBetween(start, end string) (output int) {
	start_octets, end_octets := strings.Split(start, "."), strings.Split(end, ".")
	for index, octet := range end_octets {
		start_octet, _ := strconv.Atoi(start_octets[index])
		end_octet, _ := strconv.Atoi(octet)
		address := int(math.Pow(256.0, float64(len(end_octets)-index-1)))
		output += int(float64(end_octet*address - start_octet*address))
	}
	return int(math.Abs(float64(output)))
}

func main() {
	fmt.Println(IpsBetween("10.0.0.0", "10.0.0.50")) // 50
	fmt.Println(IpsBetween("10.0.1.0", "10.0.0.0"))  // 256
	fmt.Println(IpsBetween("20.0.0.10", "20.0.1.0")) // 246
}
