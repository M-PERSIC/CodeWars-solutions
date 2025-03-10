/*
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
 
SPDX-License-Identifier: MIT-0
*/

// https://www.codewars.com/kata/55a2d7ebe362935a210000b2

fn find_smallest_int(arr: &[i32]) -> i32 {
    arr.iter().min().unwrap().to_owned()
}

fn main() {
	println!("{}", find_smallest_int(&[34, 15, 88, 2])); // 2
	println!("{}", find_smallest_int(&[34, -345, -1, 100])); // -345	
}
