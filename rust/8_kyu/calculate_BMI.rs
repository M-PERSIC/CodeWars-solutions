/*
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
 
SPDX-License-Identifier: MIT-0
*/

// https://www.codewars.com/kata/57a429e253ba3381850000fb

https://www.codewars.com/kata/57a429e253ba3381850000fb/train/rust
fn bmi(weight: u32, height: f32) -> &'static str {
    let bmi: f32 = weight as f32 / height.powf(2.0);
    match bmi {
        bmi if bmi <= 18.5 => "Underweight",
        bmi if bmi <= 25.0 => "Normal",
        bmi if bmi <= 30.0 => "Overweight",
        _ => "Obese",
    }
}

fn main() {
    println!("{}", bmi(50, 1.80)); // Underweight
    println!("{}", bmi(80, 1.80)); // Normal
    println!("{}", bmi(90, 1.80)); // Overweight
    println!("{}", bmi(90, 1.80)); // Obese
}

