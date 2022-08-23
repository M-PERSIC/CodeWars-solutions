/*
Michael Persico
Aug.23, 2022
Count by X
https://www.codewars.com/kata/5513795bd3fafb56c200049e
*/

fn count_by(x: u32, n: u32) -> Vec<u32> {
    (x..).step_by(x as usize).take(n as usize).collect()
}

fn main() {
  println!("{:?}", count_by(1,10)); // [1,2,3,4,5,6,7,8,9,10]
  println!("{:?}", count_by(2,5)); // [2,4,6,8,10]
}
