/*
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
 
SPDX-License-Identifier: MIT-0
*/

// https://www.codewars.com/kata/545a4c5a61aa4c6916000755

fn gimme(input_array: [i32;3]) -> usize {
  input_array.iter().position(|x| x != input_array.iter().min().unwrap() && x != input_array.iter().max().unwrap()).unwrap()
}

fn main() {
    println!("{}", gimme([2,3,1])); // 0
    println!("{}", gimme([-2,-3,-1])); // 0
    println!("{}", gimme([5,10,14])); // 1
}
