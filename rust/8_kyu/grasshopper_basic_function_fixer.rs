/*
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
 
SPDX-License-Identifier: MIT-0
*/

// https://www.codewars.com/kata/56200d610758762fb0000002

fn add_five(num: i32) -> i32 {
    let total = num + 5;
    total
}

fn main() {
	println!(add_five(5)); // 10
	println!(add_five(0)); // 5
	println!(add_five(-5)); // 0
}
