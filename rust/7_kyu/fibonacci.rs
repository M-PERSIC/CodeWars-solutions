/*
Michael Persico
Jul.31, 2022
Fibonacci
https://www.codewars.com/kata/57a1d5ef7cb1f3db590002af
*/

fn fib(n:u32)->u32{
    match n {
        0 => 0,
        1 | 2 => 1,
        _ => fib(n - 1) + fib(n - 2)
    }
}

fn main() {
    println!("{}", fib(0)); // 0
    println!("{}", fib(3)); // 2
    println!("{}", fib(5)); // 5
}
