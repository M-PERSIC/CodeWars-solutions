/*
Michael Persico
Jul.31, 2022
Beginner - Reduce but Grow
https://www.codewars.com/kata/57f780909f7e8e3183000078
*/

fn grow(nums: Vec<i32>) -> i32 {
    nums.iter().product()
}

fn main() {
	println!("{}", grow(vec![1, 2, 3])); // 6
	println!("{}", grow(vec![4, 1, 1, 1, 4])); // 16
	println!("{}", grow(vec![2, 2, 2, 2, 2, 2])); // 64
}
