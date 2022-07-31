/*
Michael Persico
Jul.29, 2022
A wolf in sheep's clothing
https://www.codewars.com/kata/5c8bfa44b9d1192e1ebd3d15
*/

fn warn_the_sheep(queue: &[&str]) -> String {
    match queue.iter().last() {
        Some(&"wolf") => String::from("Pls go away and stop eating my sheep"),
        _ => String::from("Oi! Sheep number " ) + 
        &format!("{}! You are about to be eaten by a wolf!", queue.len() - queue.iter().position(|&x| x == "wolf").unwrap() - 1),
        }
}

fn main() {
	println!("{}", warn_the_sheep(&["sheep", "sheep", "sheep", "sheep", "sheep", "wolf", "sheep", "sheep"])); // Oi! Sheep number 2! You are about to be eaten by a wolf!
	println!("{}", warn_the_sheep(&["sheep", "wolf", "sheep", "sheep", "sheep", "sheep", "sheep"])); // Oi! Sheep number 6! You are about to be eaten by a wolf!
	println!("{}", warn_the_sheep(&["sheep", "wolf", "sheep"])); // Oi! Sheep number 1! You are about to be eaten by a wolf!
	println!("{}", warn_the_sheep(&["sheep", "sheep", "wolf"])); // Pls go away and stop eating my sheep
}
