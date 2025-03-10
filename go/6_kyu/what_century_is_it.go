/*
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>

SPDX-License-Identifier: MIT-0
*/

// https://www.codewars.com/kata/52fb87703c1351ebd200081f

package main

import (
	"fmt"
	"strconv"
	"strings"
)

func WhatCentury(year string) string {
	var output string
	if year[len(year)-2:len(year)] == "00" {
		output = year[0:1] + year[1:2]
	} else {
		cent, _ := strconv.Atoi(string(year[0:2]))
		output = strconv.Itoa(cent + 1)
	}
	switch {
	case strings.HasSuffix(output, "1") && !strings.HasSuffix(output, "11"):
		return output + "st"
	case strings.HasSuffix(output, "2") && !strings.HasSuffix(output, "12"):
		return output + "nd"
	case strings.HasSuffix(output, "3") && !strings.HasSuffix(output, "13"):
		return output + "rd"
	default:
		return output + "th"
	}
}

func main() {
	fmt.Println(WhatCentury("2011")) // 21st
	fmt.Println(WhatCentury("2154")) // 22nd
	fmt.Println(WhatCentury("2259")) // 23rd
	fmt.Println(WhatCentury("1234")) // 13th
	fmt.Println(WhatCentury("1023")) // 11th
}
