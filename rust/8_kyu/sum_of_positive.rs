/*
Michael Persico
Jul.29, 2022
Sum of positive
https://www.codewars.com/kata/5715eaedb436cf5606000381
*/

fn positive_sum(slice: &[i32]) -> i32 {
    slice.iter().filter(|&&x| x > 0).sum()
}

fn main() {
	println!("{}", positive_sum(&[1,2,3,4,5]); // 15
	println!("{}", positive_sum(&[])); // 0
	println!("{}", positive_sum(&[-1,-2,-3,-4,-5])); // 0
}
