/*
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
 
SPDX-License-Identifier: MIT-0
*/

// https://www.codewars.com/kata/56bc28ad5bdaeb48760009b0

pub fn remove_char(s: &str) -> String {
    String::from(&s[1..s.len() - 1])
}

fn main() {
	println!("{}", remove_char("eloquent")); // loquen
	println!("{}", remove_char("country")); // ountr
	println!("{}", remove_char("person")); // erso
	println!("{}", remove_char("place")); // lac
	println!("{}", remove_char("ok")); // ""
	println!("{}", remove_char("ooopsss")); // oopss
}
