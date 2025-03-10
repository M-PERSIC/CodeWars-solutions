/*
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
 
SPDX-License-Identifier: MIT-0
*/

// https://www.codewars.com/kata/59a96d71dbe3b06c0200009c

fn generate_shape(n: i32) -> String {
    let mut square = ("+".to_string().repeat(n as usize) + "\n").repeat(n as usize);
    square.pop();
    square
}

fn main() {
    println!("{}", generate_shape(3));
}
