/*
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
 
SPDX-License-Identifier: MIT-0
*/

// https://www.codewars.com/kata/58fa273ca6d84c158e000052

fn digits(n: u64) -> usize {
  n.to_string().chars().count()
}

fn main() {
    println!("{}", digits(5)); // 1
    println!("{}", digits(12345)); // 5
    println!("{}", digits(9876543210)); // 10
}
