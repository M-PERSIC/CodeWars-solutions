/*
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
 
SPDX-License-Identifier: MIT-0
*/

// https://www.codewars.com/kata/5266876b8f4bf2da9b000362

fn likes(names: &[&str]) -> String {
    match names {
        [] => String::from("no one likes this"),
        [x] => format!("{} likes this", x),
        [x,y] => format!("{} and {} like this", x, y),
        [x,y,z] => format!("{}, {} and {} like this", x, y, z),
        _ => format!("{}, {} and {} others like this", names[0], names[1], names.len() - 2)
    }
}

fn main() {
    println!("{}", likes(&[])); // no one likes this
    println!("{}", likes(&["Peter"])); // Peter likes this
    println!("{}", likes(&["Jacob", "Alex"])); // Jacob and Alex like this
    println!("{}", likes(&["Max", "John", "Mark"])); // Max, John and Mark like this
    println!("{}", likes(&["Alex", "Jacob", "Mark", "Max"])); // Alex, Jacob and 2 others like this
}
