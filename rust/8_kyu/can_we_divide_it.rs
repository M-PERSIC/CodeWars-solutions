/*
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
 
SPDX-License-Identifier: MIT-0
*/

// https://www.codewars.com/kata/5a2b703dc5e2845c0900005a

fn is_divide_by(number: i32, a: i32, b: i32) -> bool {
    number % a ==0 && number % b == 0
}

fn main() {
	println!("{}", is_divide_by(8,2,4)); // true
	println!("{}", is_divide_by(12, -3, 4)); // true
	println!("{}", is_divide_by(8, 3, 4)); // false
	println!("{}", is_divide_by(48, 2, -5)); // false
	println!("{}", is_divide_by(-100, -25, 10)); // true
	println!("{}", is_divide_by(10000, 5, -3)); // false
	println!("{}", is_divide_by(4, 4, 2)); // true
	println!("{}", is_divide_by(5, 2, 3)); // false
	println!("{}", is_divide_by(-96, 25, 17)); // false
	println!("{}", is_divide_by(33, 1, 33)); // true
}
