/*
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
 
SPDX-License-Identifier: MIT-0
*/

// https://www.codewars.com/kata/551186edce486caa61000f5c

fn count_squares (n: u32) -> usize {
    (0..n).rev().map(|x| (n as u64 - x as u64).pow(2)).sum::<u64>() as usize
}

fn main() {
    println!("{}", count_squares(1)); // 1
    println!("{}", count_squares(2)); // 5
    println!("{}", count_squares(3)); // 14
    println!("{}", count_squares(5)); // 55
    println!("{}", count_squares(8)); // 204
    println!("{}", count_squares(15)); // 1240
}
