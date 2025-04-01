// SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
//
// SPDX-License-Identifier: MIT-0

// https://www.codewars.com/kata/52fe629e48970ad2bd0007e6

package main

import (
	"fmt"
	"math/big"
)

func ModPow(base, exponent, modulo uint64) uint64 {
	return new(big.Int).Exp(big.NewInt(int64(base)), big.NewInt(int64(exponent)), big.NewInt(int64(modulo))).Uint64()
}

func main() {
	fmt.Println(ModPow(2, 3, 5))          // 3
	fmt.Println(ModPow(4, 12, 3))         // 1
	fmt.Println(ModPow(11, 10, 300))      // 1
	fmt.Println(ModPow(11, 100000, 49))   // 32
	fmt.Println(ModPow(5, 100000000, 19)) // 5
}
