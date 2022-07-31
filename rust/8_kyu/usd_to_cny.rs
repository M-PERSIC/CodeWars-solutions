/*
Michael Persico
Jul.29, 2022
USD => CNY
https://www.codewars.com/kata/5977618080ef220766000022
*/

fn usdcny(usd: u16) -> String {
    format!("{:.02} Chinese Yuan", usd as f32 * 6.75)
}

fn main() {
	println!("{}", usdcny(15)); // 101.25 Chinese Yuan
	println!("{}", usdcny(465)); // 3138.75 Chinese Yuan
}
