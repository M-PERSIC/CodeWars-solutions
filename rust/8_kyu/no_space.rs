/*
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
 
SPDX-License-Identifier: MIT-0
*/

// https://www.codewars.com/kata/57eae20f5500ad98e50002c5

fn no_space(x : String) -> String{
    x.replace(" ", "")
}

fn main() {
    println!("{}", no_space("8 j 8   mBliB8g  imjB8B8  jl  B".to_string()));
}
