/*
Michael Persico
Aug.24, 2022
Maximum Length Difference
https://www.codewars.com/kata/5663f5305102699bad000056
*/

fn mx_dif_lg(a1: Vec<&str>, a2: Vec<&str>) -> i32 {
    let mut max_length: i32 = -1;
    a1.iter().for_each(|worda1| for worda2 in a2.iter() {
        if i32::abs(worda1.len() as i32 - worda2.len() as i32) > max_length {
            max_length = i32::abs(worda1.len() as i32 - worda2.len() as i32);
        } else {
            
        }
    });
    max_length as i32
}

fn main() {
    let s1 = vec!["hoqq", "bbllkw", "oox", "ejjuyyy", "plmiis", "xxxzgpsssa", "xxwwkktt", "znnnnfqknaz", "qqquuhii", "dvvvwz"];
    let s2 = vec!["cccooommaaqqoxii", "gggqaffhhh", "tttoowwwmmww"];
    println!("{}", mx_dif_lg(s1, s2)); // 13
}
