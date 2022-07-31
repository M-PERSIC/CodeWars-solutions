/*
Michael Persico
Jul.31, 2022
The highest profit wins!
https://www.codewars.com/kata/559590633066759614000063
*/

fn min_max(lst: &[i32]) -> (i32, i32) {
    (*lst.iter().min().unwrap(), *lst.iter().max().unwrap())
}

fn main() {
	println!("{}", min_max(&[1,2,3,4,5])); // (1,5)
	println!("{}", min_max(&[2334454,5])); // (5,2334454)
	println!("{}", min_max(&[1])); // (1,1)
}
