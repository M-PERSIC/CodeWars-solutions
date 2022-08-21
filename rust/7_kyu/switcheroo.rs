/*
Michael Persico
Aug.19, 2022
Switcheroo
https://www.codewars.com/kata/57f759bb664021a30300007d
*/

fn switcheroo(s: &str) -> String {
    s.chars()
    .map(|c| match c {
        'a' => 'b',
        'b' => 'a',
        _ => c
    })
    .collect()
}

fn main() {
    println!("{}", switcheroo("abc")); // bac
    println!("{}", switcheroo("aaabcccbaaa")); // bbbacccabbb
    println!("{}", switcheroo("ccccc")); // ccccc
    println!("{}", switcheroo("abababababababab")); // babababababababa
    println!("{}", switcheroo("aaaaa")); // bbbbb    
}
