/*
Michael Persico
Jul.29, 2022
String repeat
https://www.codewars.com/kata/57a0e5c372292dd76d000d7e
*/

use std::iter;

fn repeat_str(src: &str, count: usize) -> String {
    iter::repeat(src).take(count).collect()
}

fn main() {
    println!("{}", repeat_str("a", 4)); // aaaa
    println!("{}", repeat_str("hello", 3)); // hello hello hello 
    println!("{}", repeat_str("abc", 2)); // abcabc
}
