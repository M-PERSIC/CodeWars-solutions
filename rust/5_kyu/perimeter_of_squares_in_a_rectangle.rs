/*
Michael Persico
Aug.11, 2022
Perimeter of squares in a rectangle
https://www.codewars.com/kata/559a28007caad2ac4e000083
*/

fn perimeter(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut fib: Vec<u64> = vec![1,1];
            for num in 2..=n {
                fib.push(fib.get(num as usize - 1).unwrap() + fib.get(num as usize - 2).unwrap());
            }
            fib.iter().sum::<u64>() * 4 
        }
    }
}


fn main() {
    println!("{}", perimeter(5)); // 80
    println!("{}", perimeter(7)); // 216
    println!("{}", perimeter(20)); // 114624
    println!("{}", perimeter(30)); // 14098308
}
