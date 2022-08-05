/*
Michael Persico
Aug.05, 2022
Is It Negative Zero (-0)?
https://www.codewars.com/kata/5c5086287bc6600001c7589a/train/rust
*/

fn is_negative_zero(n: f32) -> bool {
    n == 0.0 && n.is_sign_negative()
}

fn main() {
	println!("{}", is_negative_zero(f32::NEG_INFINITY)); // false
	println!("{}", is_negative_zero(-1.0)); // false
	println!("{}", is_negative_zero(-0.0)); // true
	println!("{}", is_negative_zero(0.0)); // false
}
