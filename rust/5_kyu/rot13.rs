/*
Michael Persico
Aug.25, 2022
Rot13
https://www.codewars.com/kata/530e15517bc88ac656000716
*/

fn rot13(message: &str) -> String {
    message.chars()
    .map(|x| match x {
        x if x.is_uppercase() => char::from_u32(((x as u32 - 52) % 26) + 65).unwrap(),
        x if x.is_lowercase() => char::from_u32(((x as u32 - 84) % 26) + 97).unwrap(),
        _ => x,
    })
    .collect()
}

fn main() {
    println!("{}", rot13("test")); // grfg
    println!("{}", rot13("Test")); // Grfg
}

