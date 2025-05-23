/*
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
 
SPDX-License-Identifier: MIT-0
*/

// https://www.codewars.com/kata/57a5c31ce298a7e6b7000334

fn bin_to_decimal(inp: &str) -> i32 {
    i32::from_str_radix(inp, 2).unwrap()
}

fn main() {
	println!("{}", bin_to_decimal("0")); // 0
	println!("{}", bin_to_decimal("1")); // 1
	println!("{}", bin_to_decimal("1001001")); // 73
}
