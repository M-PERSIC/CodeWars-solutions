/*
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
 
SPDX-License-Identifier: MIT-0
*/

// https://www.codewars.com/kata/554ca54ffa7d91b236000023

fn delete_nth(lst: &[u8], n: usize) -> Vec<u8> {
    let mut list: Vec<u8> = vec![];
    for num in lst {
        if list.iter().filter(|&&x| x == *num).count() < n {
            list.push(*num);
        }
    }
    list
}

fn main() {
	println!("{:?}", delete_nth(&[20,37,20,21], 1)); // [20,37,21]
	println!("{:?}", delete_nth(&[1,1,3,3,7,2,2,2,2], 3)); // [1, 1, 3, 3, 7, 2, 2, 2]	
}

