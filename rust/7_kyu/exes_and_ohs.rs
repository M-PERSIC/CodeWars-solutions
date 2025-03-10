/*
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
 
SPDX-License-Identifier: MIT-0
*/

// https://www.codewars.com/kata/55908aad6620c066bc00002a

fn xo(string: &'static str) -> bool {
    string.to_lowercase().matches("x").count() == string.to_lowercase().matches("o").count() || false
}

fn main() {
	println!("{}", xo("xo")); // true
	println!("{}", xo("Xo")); // true
	println!("{}", xo("xxOo")); // true
	println!("{}", xo("xxxm")); // false
	println!("{}", xo("Oo")); // false
	println!("{}", xo("ooom")); // false
}
