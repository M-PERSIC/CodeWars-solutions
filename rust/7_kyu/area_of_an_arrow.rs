/*
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
 
SPDX-License-Identifier: MIT-0
*/

// https://www.codewars.com/kata/589478160c0f8a40870000bc

fn arrow_area(a: u32, b: u32) -> f64 {
    (a as f64 * (b as f64)/2.0)/2.0
}

fn main() {
    println!("{}", arrow_area(7,6)); // 10.5
}

