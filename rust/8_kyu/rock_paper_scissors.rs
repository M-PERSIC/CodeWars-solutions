/*
Michael Persico
Jul.28, 2022
Rock Paper Scissors!
https://www.codewars.com/kata/5672a98bdbdd995fad00000f
*/

fn rps(p1: &str, p2: &str) -> &'static str  {
    match p1 {
        p1 if p1 == p2 => "Draw!",
        "scissors" if p2 == "paper" => "Player 1 won!",
        "rock" if p2 == "scissors" => "Player 1 won!",
        "paper" if p2 == "rock" => "Player 1 won!",
        _ => "Player 2 won!",
    }
}

fn main() {
	println!("{}", rps("scissors", "paper")); // Player 2 won!
}

