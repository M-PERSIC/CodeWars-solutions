/*
Michael Persico
Jul.29, 2022
Categorize New Member
https://www.codewars.com/kata/5502c9e7b3216ec63c0001aa
*/

fn open_or_senior(data: Vec<(i32, i32)>) -> Vec<String> {
  data.iter().map(|x| if x.0 >= 55 && x.1 > 7 {"Senior".to_string()} else {"Open".to_string()}).collect()
}

fn main() {
	println!("{:?}", open_or_senior(vec![(45, 12), (55,21), (19, -2), (104, 20)])); // "Open", "Senior", "Open", "Senior"
	println!("{:?}", open_or_senior(vec![(3, 12), (55,1), (91, -2), (54, 23)])); // "Open", "Open", "Open", "Open"
}
