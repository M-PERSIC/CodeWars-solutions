/*
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
 
SPDX-License-Identifier: MIT-0
*/

// https://www.codewars.com/kata/58f5c63f1e26ecda7e000029

fn wave(s: &str) -> Vec<String> {
    (0..s.len()).map(|x| "".to_string() 
        + &s[0..x] 
        + &s[x..x+1].to_uppercase() 
        + &s[x+1..])
    .filter(|x| x.chars().any(|y| y.is_uppercase()))
    .collect()
}

fn main() {
    println!("{}", wave("hello")); // ["Hello", "hEllo", "heLlo", "helLo", "hellO"]
}
