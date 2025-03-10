/*
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
 
SPDX-License-Identifier: MIT-0
*/

// https://www.codewars.com/kata/56c5847f27be2c3db20009c3

fn subtract_sum(mut n: u32) -> &'static str {
    while n > 0 {
        let sum = n.to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .sum();
        n = u32::checked_sub(n, sum).unwrap_or(0);
        match n {
            01 | 03 | 24 | 26 | 47 | 49 | 68 | 70 | 91 | 93 => return "kiwi",
            02 | 21 | 23 | 42 | 44 | 46 | 65 | 67 | 69 | 88 => return "pear",
            04 | 06 | 25 | 29 | 48 | 50 | 71 | 73 | 92 | 94 | 96 => return "banana",
            05 | 07 | 28 | 30 | 32 | 51 | 53 | 74 | 76 | 95 | 97 => return "melon",
            08 | 10 | 12 | 31 | 33 | 52 | 56 | 75 | 77 | 79 | 98 | 100 => return "pineapple",
            09 | 18 | 27 | 36 | 45 | 54 | 63 | 72 | 81 | 90 | 99 => return "apple",
            11 | 13 | 34 | 55 | 57 | 59 | 78 | 80 => return "cucumber",
            14 | 16 | 35 | 37 | 39 | 58 | 60 | 83 => return "orange",
            15 | 17 | 19 | 38 | 40 | 61 | 82 | 84 | 86 => return "grape",
            20 | 22 | 41 | 43 | 62 | 64 | 66 | 85 | 87 | 89 => return "cherry",
            _ => {},
        }
    }
    ""
}

fn main() {
    println!("{}", subtract_sum(10)); // apple
}


