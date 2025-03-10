/*
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
 
SPDX-License-Identifier: MIT-0
*/

// https://www.codewars.com/kata/5a9c35e9ba1bb5c54a0001ac

fn add(x: i32, y: i32) -> i32 {
  let carry = (x & y) << 1;
  let sum = x^y;
    if carry == 0 {
        return sum;
    }
    add(carry, sum)
}

fn main() {
    println!("{}", add(1,2)); // 3
    println!("{}", add(-14,-16)); // -30
    println!("{}", add(-13,13)); // 0
}
