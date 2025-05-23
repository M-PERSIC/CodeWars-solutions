/*
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
 
SPDX-License-Identifier: MIT-0
*/

// https://www.codewars.com/kata/58ca658cc0d6401f2700045f

fn find_multiples(n: u32, limit: u32) -> Vec<u32> {
    (1..=limit).filter(|x| x % n == 0).collect()
}

fn main() {
    println!("{:?}", find_multiples(1,2)); // [1,2]
    println!("{:?}", find_multiples(5,7)); // [5]
    println!("{:?}", find_multiples(4,27)); // [4, 8, 12, 16, 20, 24]
    println!("{:?}", find_multiples(11,54)); // [11, 22, 33, 44]
    println!("{:?}", find_multiples(5,25)); // [5, 10, 15, 20, 25]
}
