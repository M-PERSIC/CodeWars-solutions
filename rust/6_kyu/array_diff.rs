/*
Michael Persico
Jul.31, 2022
Array.diff
https://www.codewars.com/kata/523f5d21c841566fde000009
*/

fn array_diff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    let mut result = vec![];
    for elem in a {
        result.push(elem)
    }
    result.retain(|x| !b.contains(x));
    result
}

fn main() {
    println!("{:?}", array_diff(vec![1,2], vec![1])); // [1]
    println!("{:?}", array_diff(vec![1,2,2], vec![1])); // [2,2]  
    println!("{:?}", array_diff(vec![1,2,2], vec![2])); // [2,2]  
    println!("{:?}", array_diff(vec![1,2,2], vec![])); // [1,2,2]  
    println!("{:?}", array_diff(vec![], vec![1,2])); // []
    println!("{:?}", array_diff(vec![], vec![1,2])); // [3]
}
