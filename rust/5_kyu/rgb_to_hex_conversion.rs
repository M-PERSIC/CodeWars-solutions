/*
Michael Persico
Sep.01, 2022
RGB To Hex Conversion
https://www.codewars.com/kata/513e08acc600c94f01000001
*/

fn rgb(r: i32, g: i32, b: i32) -> String {
  format!("{:02X}{:02X}{:02X}", r.clamp(0, 255), g.clamp(0, 255), b.clamp(0, 255))
}

fn main() {
    println!("{}", rgb(0,0,0)); // 000000
    println!("{}", rgb(1,2,3)); // 010203
    println!("{}", rgb(255,255,255)); // FFFFFF
    println!("{}", rgb(254,253,252)); // FEFDFC
    println!("{}", rgb(-20,275,125)); // 00FF7D
}
