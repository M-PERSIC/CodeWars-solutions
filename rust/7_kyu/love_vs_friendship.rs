/*
Michael Persico
Aug.17, 2022
Love vs friendship
https://www.codewars.com/kata/59706036f6e5d1e22d000016
*/

fn words_to_marks(s: &str) -> u32 {
    s.chars()
    .map(|x| x as u32 - 96)
    .sum()
    
}

fn main() {
    println!("{}", words_to_marks("attitude")); // 100
    println!("{}", words_to_marks("friends")); // 75    
    println!("{}", words_to_marks("family")); // 66    
    println!("{}", words_to_marks("selfness")); // 99
    println!("{}", words_to_marks("knowledge")); // 96
}
