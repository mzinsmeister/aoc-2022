use std::fs::read_to_string;
use std::vec::Vec;

fn main() {
    let input = read_to_string("input.txt").unwrap();


    let output = input.split("\n\n").map(|e| e.split("\n").filter(|l| !l.is_empty()).map(|e| e.parse::<i32>().unwrap()).sum::<i32>()).max();

    println!("{}", output.unwrap());

    //Part 2

    let mut vec: Vec<i32> = input.split("\n\n").map(|e| e.split("\n").filter(|l| !l.is_empty()).map(|e| e.parse::<i32>().unwrap()).sum::<i32>()).collect();

    vec.sort();

    println!("{}", vec.iter().rev().take(3).sum::<i32>());
}
