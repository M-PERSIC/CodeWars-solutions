/*
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
 
SPDX-License-Identifier: MIT-0
*/

// https://www.codewars.com/kata/5174a4c0f2769dd8b1000003

fn sort_numbers(arr: &Vec<i32>) -> Vec<i32> {
    let mut result = arr.to_owned();
    result.sort();
    result
}

fn main() {
	println!("{:?}", sort_numbers(&vec![1, 2, 3, 10, 5])); // [1, 2, 3, 5, 10]
	println!("{:?}", sort_numbers(&vec![])); // []
	println!("{:?}", sort_numbers(&vec![20, 2, 10])); // [2, 10, 20]
	println!("{:?}", sort_numbers(&vec![2, 20, 10])); // [2, 10, 20]
	println!("{:?}", sort_numbers(&vec![2, 10, 20])); // [2, 10, 20]
}
