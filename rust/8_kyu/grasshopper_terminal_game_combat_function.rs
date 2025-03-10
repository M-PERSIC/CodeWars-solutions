/*
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
 
SPDX-License-Identifier: MIT-0
*/

// https://www.codewars.com/kata/586c1cf4b98de0399300001d

fn combat(health: f32, damage: f32) -> f32 {
     std::ops::Sub::sub(health, damage).clamp(0.0, f32::MAX) 
}

fn main() {
	println!("{}", combat(100.0, 5.0)); // 95.0
	println!("{}", combat(92.0, 8.0)); // 84.0
	println!("{}", combat(20.0, 30.0)); // 0.0
}
