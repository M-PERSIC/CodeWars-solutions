/*
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
 
SPDX-License-Identifier: MIT-0
*/

// https://www.codewars.com/kata/5262119038c0985a5b00029f

fn is_prime(x: i64) -> bool {
    match x {
        x if x <= 0 || x == 1 => false,
        _ => (2..(x as f64).sqrt() as i64 + 1).all(|num| x % num != 0)
    }    
}

fn main() {
    println!("{}", is_prime(4)); // false
    println!("{}", is_prime(6)); // false
    println!("{}", is_prime(8)); // false
    println!("{}", is_prime(9)); // false
    println!("{}", is_prime(45)); // false
    println!("{}", is_prime(-5)); // false
    println!("{}", is_prime(-8)); // false
    println!("{}", is_prime(-41)); // false
    println!("{}", is_prime(0)); // false
    println!("{}", is_prime(3)); // true
    println!("{}", is_prime(73)); // true
    println!("{}", is_prime(4)); // false
    println!("{}", is_prime(2)); // true
}
