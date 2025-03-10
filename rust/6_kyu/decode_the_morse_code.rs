/*
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
 
SPDX-License-Identifier: MIT-0
*/

// https://www.codewars.com/kata/54b724efac3d5402db00065e

use regex::Regex;
mod preloaded;
use preloaded::MORSE_CODE;
// MORSE_CODE is `HashMap<String, String>`. e.g. ".-" -> "A".

fn decode_morse(encoded: &str) -> String {
    Regex::new(r" ")
    .unwrap()
    .split(encoded)
    .fold("".to_string(), |acc, letter| if let Some(i) = MORSE_CODE.get(letter) {acc + i} else  {acc + " "} )
    .to_string()
    .replace("  ", " ")
    .trim()
    .to_string()
}
