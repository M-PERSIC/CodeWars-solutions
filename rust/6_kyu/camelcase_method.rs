/*
Michael Persico
Jul.29, 2022
CamelCase Method
https://www.codewars.com/kata/587731fda577b3d1b0001196
*/

fn camel_case(str: &str) -> String {
    str.split_whitespace().map(|x| x[0..1].to_uppercase() + &x[1..]).collect()
}

fn main() {
	println!("{}", camel_case("test case")); // TestCase
	println!("{}", camel_case("camel case method")); // CamelCaseMethod
	println!("{}", camel_case("say hello ")); // SayHello
	println!("{}", camel_case(" camel case word")); // CamelCaseWord
	println!("{}", camel_case("")); // ""
}
