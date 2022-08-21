/*
Michael Persico
Aug.21, 2022
Build a square
https://www.codewars.com/kata/59a96d71dbe3b06c0200009c
*/

fn generate_shape(n: i32) -> String {
    let mut square = ("+".to_string().repeat(n as usize) + "\n").repeat(n as usize);
    square.pop();
    square
}

fn main() {
    println!("{}", generate_shape(3));
}
