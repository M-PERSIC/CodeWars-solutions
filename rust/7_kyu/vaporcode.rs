/*
Michael Persico
Aug.13, 2022
V A P O R C O D E
https://www.codewars.com/kata/5966eeb31b229e44eb00007a
*/

fn vaporcode(s: &str) -> String {
    s.replace(" ", "")
    .chars()
    .map(|x| String::from(x.to_ascii_uppercase()))
    .collect::<Vec<String>>()
    .join("  ")
}

fn main() {
    println!("{}", vaporcode("Lets go to the movies")); // L  E  T  S  G  O  T  O  T  H  E  M  O  V  I  E  S
}
