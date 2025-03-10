/*
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
 
SPDX-License-Identifier: MIT-0
*/

// https://www.codewars.com/kata/5949481f86420f59480000e7

fn odd_or_even(numbers: Vec<i32>) -> String {
    match numbers.iter().sum::<i32>() {
        numbers if numbers % 2 == 0 => "even".to_string(),
        _ => "odd".to_string(),
    }
}

fn main() {
	println!("{}", odd_or_even(vec![])); // even
	println!("{}", odd_or_even(vec![0])); // even
	println!("{}", odd_or_even(vec![1])); // odd
	println!("{}", odd_or_even(vec![0, 1, 5])); // even
	println!("{}", odd_or_even(vec![0, 1, 4])); // odd	
	println!("{}", odd_or_even(vec![0, -1, -5])); // even
	println!("{}", odd_or_even(vec![0, -1, 2])); // odd
}
