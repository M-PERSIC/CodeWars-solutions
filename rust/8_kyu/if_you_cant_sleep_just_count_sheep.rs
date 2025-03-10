/*
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
 
SPDX-License-Identifier: MIT-0
*/

// https://www.codewars.com/kata/5b077ebdaf15be5c7f000077

fn count_sheep(n: u32) -> String {
    (1..=n).fold("".to_string(), |acc, item| acc + &item.to_string() + " sheep...")
}

fn main() {
	println!("{}", count_sheep(0)); // ""
	println!("{}", count_sheep(1)); // 1 sheep...
	println!("{}", count_sheep(2)); // 1 sheep...2 sheep...
	println!("{}", count_sheep(3)); // 1 sheep...2 sheep...3 sheep...
}
