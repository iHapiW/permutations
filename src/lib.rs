use std::collections::HashSet;
use std::hash::Hash;
use std::io::{self, Write};

fn _permutate<T>(list: &Vec<T>, holder: &mut Vec<T>, perms: &mut HashSet<Vec<T>>)
    where T: Clone + PartialEq + Eq + Hash
{
    if list.is_empty() {
        perms.insert(holder.clone());
        return;
    }

    for i in 0..list.len() {
        let mut list_copy = list.clone();
        holder.push(list_copy.remove(i));
        _permutate(&list_copy, holder, perms);
        holder.pop();
    }
}

pub fn permutate<T>(list: &Vec<T>) -> HashSet<Vec<T>>
    where T: Clone + PartialEq + Eq + Hash
{
    let mut _holder = Vec::new();
    let mut perms = HashSet::new();
    _permutate(list, &mut _holder, &mut perms);

    perms
}

fn _print_vec<T>(vec: &Vec<T>)
    where T: std::fmt::Display
{
    let n = vec.len();
    print!("[");
    for i in  0..n {
        print!("{}", vec[i]);
        if i != n-1 {
            print!(", ");
        }
    }
    print!("]");
    io::stdout().flush().unwrap();
}
pub fn print_perms<T>(perms: &HashSet<Vec<T>>)
    where T: std::fmt::Display
{
    println!("{{");
    for i in perms {
        print!("\t");
        _print_vec(i);
        println!();
    }
    println!("}}");
}