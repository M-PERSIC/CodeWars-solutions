/*
Michael Persico
Jul.30, 2022
Difference of Volumes of Cuboids
https://www.codewars.com/kata/58cb43f4256836ed95000f97
*/

fn find_difference(a: &[i32; 3], b: &[i32; 3]) -> i32 {
    i32::abs(a.iter().product::<i32>() - b.iter().product::<i32>())
}

fn main() {
	println!("{}", find_difference(&[3, 2, 5], &[1, 4, 4])); // 14
	println!("{}", find_difference(&[9, 7, 2], &[5, 2, 2])); // 106	
	println!("{}", find_difference(&[11, 2, 5], &[1, 10, 8])); // 30
	println!("{}", find_difference(&[4, 4, 7], &[3, 9, 3])); // 31
	println!("{}", find_difference(&[15, 20, 25], &[10, 30, 25])); // 0
}
