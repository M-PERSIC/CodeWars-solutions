/*
Michael Persico
Jul.28, 2022
Sum of Digits / Digital Root
https://www.codewars.com/kata/541c8630095125aba6000c00
*/
fn digital_root(n: i64) -> i64 {
    let num = n.to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .sum::<u32>() as i64;
    if num.to_string().len() > 1 {
        digital_root(num)
    } else {
        num
    }
}

fn main() {
	println!("{}", digital_root(16)); // 7
}

