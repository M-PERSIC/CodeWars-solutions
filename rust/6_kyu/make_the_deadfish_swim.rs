/*
Michael Persico
Aug.27, 2022
Make the Deadfish Swim
https://www.codewars.com/kata/51e0007c1f9378fa810002a9
*/

fn parse(code: &str) -> Vec<i32> {
    let mut v: Vec<i32> = vec![];
    code.chars().fold(0, |acc, x| match x {
      'i' => acc + 1,
      'd' => acc - 1,
      's' => i32::pow(acc, 2),
      'o' => {v.push(acc); acc},
      _ => acc,
    });
    v
}

fn main() {
    println!("{}", parse("iiisdoso")); // [8,64]
    println!("{}", parse("iiisdosodddddiso")); // [8, 64, 3600]
}
