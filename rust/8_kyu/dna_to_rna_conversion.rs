/*
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>
 
SPDX-License-Identifier: MIT-0
*/

// https://www.codewars.com/kata/5556282156230d0e5e000089

fn dna_to_rna(dna: &str) -> String {
    dna.chars().map(|x| match x {
        'T' => 'U',
        _ => x
        }
    ).collect()
}

fn main() {
	println!("{}", dna_to_rna("TTTT")); // UUUU
	println!("{}", dna_to_rna("GCAT")); // GCAU	
}
