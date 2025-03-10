/*
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
 
SPDX-License-Identifier: MIT-0
*/

// https://www.codewars.com/kata/53369039d7ab3ac506000467

fn bool_to_word(value: bool) -> &'static str {
    match value {
        true => "Yes",
        false => "No",
    }
}

fn main() {
	println!("{}", bool_to_word(true)); / Yes
	println!("{}", bool_to_word(false)); / No	
}

