use std::io::{self, Write};

use permutations::*;

fn main() {
    println!("\tWelcome to Permutation Generator");
    loop {
        println!("Please Enter your set in the following format: [Hello, World, Foo, bar, ]");
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();
        if !input.starts_with('[') || !input.ends_with("]") {
            eprintln!("\nWrong Format!\n");
            continue;
        }

        let content = &input[1..input.len()-1];
        let list:Vec<&str> = content
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        let permutations = permutate(&list);
        println!("\nPermutations: ");
        print_perms(&permutations);
        println!("\nPermutation Count: {}\n", permutations.len());
    }
}
