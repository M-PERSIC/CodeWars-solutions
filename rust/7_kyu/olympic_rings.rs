/*
Michael Persico
Aug. 17, 2022
Olympic Rings
https://www.codewars.com/kata/57d06663eca260fe630001cc
*/

fn olympic_ring(s: &str) -> String {
    let m: i32 = s.chars()
    .filter(|&x| x == 'o' 
        || x == 'b' 
        || x == 'a' 
        || x == 'p' 
        || x == 'e' 
        || x == 'd'
        || x == 'P'
        || x == 'R'
        || x == 'D'
        || x == 'B'
        || x == 'q'
        || x == 'A'
        || x == 'g'
        || x == 'O'
        || x == 'Q')
        .map(|x| if x != 'B' { 1 } else { 2 })
        .sum();
    match m {
        m if m / 2 == 2 => String::from("Bronze!"),
        m if m / 2 == 3 => String::from("Silver!"),
        m if m / 2 > 3 => String::from("Gold!"),
        _ => String::from("Not even a medal!")
    }
}
fn main() {
    println!("{}", olympic_ring("wHjMudLwtoPGocnJ")); // Bronze!
    println!("{}", olympic_ring("eCEHWEPwwnvzMicyaRjk")); // Bronze!	
    println!("{}", olympic_ring("JKniLfLW")); // Not even a medal!	
    println!("{}", olympic_ring("EWlZlDFsEIBufsalqof")); // Silver!
    println!("{}", olympic_ring("IMBAWejlGRTDWetPS")); // Gold!
}
