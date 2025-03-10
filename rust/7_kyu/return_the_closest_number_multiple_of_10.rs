/*
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
 
SPDX-License-Identifier: MIT-0
*/

// https://www.codewars.com/kata/58249d08b81f70a2fc0001a4

fn closest_multiple_of_10(n: u32) -> u32 {
        let last_digit: u32 = n.to_string().chars().last().unwrap().to_digit(10).unwrap();
        match last_digit {
            last_digit if last_digit >= 5 => n + 10 - last_digit,
            _ => n - last_digit
    }
}

fn main() {
    println!("{}", closest_multiple_of_10(54)); // 50
    println!("{}", closest_multiple_of_10(55)); // 60
}

