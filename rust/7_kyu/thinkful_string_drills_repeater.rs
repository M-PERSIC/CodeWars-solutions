/*
Michael Persico
Jul.29, 2022
Thinkful - String Drills: Repeater
https://www.codewars.com/kata/585a1a227cb58d8d740001c3
*/

fn repeater(string: &str, n: u32) -> String {
    std::iter::repeat(string).take(n as usize).collect()
}

fn main() {
    println!("{}", repeater("a", 5)); // aaaaa
    println!("{}", repeater("Na", 16)); // NaNaNaNaNaNaNaNaNaNaNaNaNaNaNaNa
    println!("{}", repeater("Wub", 6)); // Wub Wub Wub Wub Wub Wub 
}
