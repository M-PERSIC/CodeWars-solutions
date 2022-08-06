/*
Michael Persico
Aug.06, 2022
Bit Counting
https://www.codewars.com/kata/526571aae218b8ee490006f4
*/

fn count_bits(n: i64) -> u32 {
  format!("{n:b}").chars().filter(|&x| x == '1').count() as u32
}

fn main() {
    println!("{}", count_bits(0)); // 0
    println!("{}", count_bits(4)); // 1
    println!("{}", count_bits(7)); // 3
    println!("{}", count_bits(9)); // 2
    println!("{}", count_bits(10)); // 2
}
