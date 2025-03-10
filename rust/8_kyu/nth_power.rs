/*
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
 
SPDX-License-Identifier: MIT-0
*/

// https://www.codewars.com/kata/57d814e4950d8489720008db

fn index(nums: &[u64], n: usize) -> Option<u64> {
    nums.into_iter().nth(n).and_then(|&x| Some(x.pow(n as u32)))
}

fn main() {
	println!("{}", index(&[1, 2, 3, 4], 2)); // Some(9)
	println!("{}", index(&[1, 3, 10, 100], 3)); // Some(1000000)
	println!("{}", index(&[1, 2, 3, 4], 69)); // None
}
