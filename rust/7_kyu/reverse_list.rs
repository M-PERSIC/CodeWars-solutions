/*
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
 
SPDX-License-Identifier: MIT-0
*/

// https://www.codewars.com/kata/57a04da9e298a7ee43000111

fn reverse_list(lst: &[i32]) -> Vec<i32> {
    lst.iter().rev().map(|&x| x).collect()
}

fn main() {
    println!("{:?}", reverse_list(&[])); // []
	println!("{:?}", reverse_list(&[1,2,3,4])); // [4,3,2,1]
	println!("{:?}", reverse_list(&[2,3,4,5,6])); // [6,5,4,3,2]
}

