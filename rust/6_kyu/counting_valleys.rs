/*
Michael Persico
Aug.29, 2022
Counting Valleys
https://www.codewars.com/kata/5da9973d06119a000e604cb6
*/

fn counting_valleys(s: &str) -> u32 {
    let mut valleys: u32 = 0;
    s.chars()
    .fold(0, |acc, letter| match letter {
        'U' => if acc + 1 == 0 {valleys += 1; acc + 1} else {acc + 1},
        'D' => acc - 1,
        _ => {acc},
    });
    valleys
}

fn main() {
	println!("{}", counting_valleys("UFFDDFDUDFUFU")); // 1
	println!("{}", counting_valleys("UFFDDFDUDFUFUUFFDDUFFDDUFFDDUDUDUDUDUDUUUUUUUUU")); // 3
	println!("{}", counting_valleys("UFFDDFDUDFUFUUFFDDFDUDFUFUUFFDDFDUDFUFUUFFDDFDUDFUFUUFFDDFDUDFUFUUFFDDFDUDFUFU")); // 6
	println!("{}", counting_valleys("UFFDDFDUDFUFUUFFDDFDUDFUFU")); // 2
	println!("{}", counting_valleys("UFFDDFDUDFUFUUFFDDFDUDFUFUUFFDDFDUDFUFUUFFDDFDUDFUFU")); // 4
	println!("{}", counting_valleys("DFFFU")); // 1
	println!("{}", counting_valleys("UFFFD")); // 0
	println!("{}", counting_valleys("DFFFD")); // 0
	println!("{}", counting_valleys("UFFFU")); // 0
}
