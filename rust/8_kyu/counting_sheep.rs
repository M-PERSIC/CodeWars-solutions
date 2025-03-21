/*
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
 
SPDX-License-Identifier: MIT-0
*/

// https://www.codewars.com/kata/54edbc7200b811e956000556

fn count_sheep(sheep: &[bool]) -> u8 {
  sheep.iter().filter(|&&x| x).count() as u8
}

fn main() {
	println!("{}", count_sheep(&[false])); // 0
	println!("{}", count_sheep(&[true])); // 1
	println!("{}", count_sheep(&[true, false])); // 1	
}
