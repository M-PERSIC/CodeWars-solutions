package main

import "fmt"

func BreakChocolate(n, m int) int {
	if n != 0 && m != 0 {
		return n*m - 1
	} else {
		return 0
	}
}

func main() {
	fmt.Println(BreakChocolate(1, 1)) // 0
	fmt.Println(BreakChocolate(2, 1)) // 1
	fmt.Println(BreakChocolate(3, 1)) // 2
	fmt.Println(BreakChocolate(5, 5)) // 24
}
