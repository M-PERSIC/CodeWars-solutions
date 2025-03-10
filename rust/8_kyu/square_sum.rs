/*
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
 
SPDX-License-Identifier: MIT-0
*/

// https://www.codewars.com/kata/515e271a311df0350d00000f

fn square_sum(vec: Vec<i32>) -> i32 {
    return vec.iter().map(|x| x.pow(2)).sum();
}

fn main() {
    println!("{}",square_sum(vec![1,2])); // 5
}
