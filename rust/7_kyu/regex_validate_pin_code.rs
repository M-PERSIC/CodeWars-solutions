https://www.codewars.com/kata/55f8a9c06c018a0d6e000132/train/rust
/*
Michael Persico
Jul.29, 2022
Regex validate PIN code
https://www.codewars.com/kata/55f8a9c06c018a0d6e000132
*/

use regex::Regex;

fn validate_pin(pin: &str) -> bool {
    Regex::new("^([0-9]{4}|[0-9]{6})$").unwrap().is_match(pin)
}

fn main() {
    println!("{}", validate_pin("1")); // false
    println!("{}", validate_pin("12345")); // false
    println!("{}", validate_pin("-1234")); // false  
    println!("{}", validate_pin("1.234")); // false  
    println!("{}", validate_pin("-1.234")); // false 
    println!("{}", validate_pin("a234")); // false 
    println!("{}", validate_pin(".234")); // false   
    println!("{}", validate_pin("1234")); // true   
    println!("{}", validate_pin("123456")); // true   
    println!("{}", validate_pin("090909")); // true   
}
