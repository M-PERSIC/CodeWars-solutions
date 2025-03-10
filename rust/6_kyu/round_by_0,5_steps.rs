/*
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
 
SPDX-License-Identifier: MIT-0
*/

// https://www.codewars.com/kata/51f1342c76b586046800002a

fn solution(n: f64) -> f64 {
    match n {
        n if n % 1.0 < 0.25 => f64::floor(n),
        n if n % 1.0 < 0.5 || n % 1.0 < 0.75 => f64::floor(n) + 0.5,
        _ => f64::ceil(n)
    }
}

fn main() {
    println!("{:.1}", solution(4.2)); // 4.0
    println!("{:.1}", solution(4.4)); // 4.5
    println!("{:.1}", solution(4.6)); // 4.5
    println!("{:.1}", solution(4.8)); // 5.0
}
