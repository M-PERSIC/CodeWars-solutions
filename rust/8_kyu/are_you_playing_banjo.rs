/*
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
 
SPDX-License-Identifier: MIT-0
*/

// https://www.codewars.com/kata/53af2b8861023f1d88000832

fn are_you_playing_banjo(name: &str) -> String {
    match name.chars().nth(0).unwrap() {
        'R' | 'r' => String::from(name) + " plays banjo",
        _ => String::from(name) + " does not play banjo",
    }
}

fn main() {
	println!("{}", are_you_playing_banjo("Martin")); // Martin does not play banjo
	println!("{}", are_you_playing_banjo("Rikke")); // Rikke plays banjo
	println!("{}", are_you_playing_banjo("bravo")); // bravo does not play banjo
	println!("{}", are_you_playing_banjo("rolf")); // rolf plays banjo
}
