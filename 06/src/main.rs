use std::{fs::read_to_string, collections::BTreeSet};

fn main() {
    let input_str = read_to_string("input.txt").unwrap();

    let input_chars = input_str.chars().collect::<Vec<char>>();

    let mut result = input_chars.len();
    for (i, c) in input_chars.iter().enumerate().skip(3) {
        let mut last4: BTreeSet<char> = BTreeSet::new();
        for j in 0..4 {
            last4.insert(input_chars[i - j]);
        }
        if last4.len() == 4 {
            result = i + 1;
            break;
        }
    }

    println!("{}", result);

    let mut result2 = input_chars.len();
    for (i, c) in input_chars.iter().enumerate().skip(13) {
        let mut last4: BTreeSet<char> = BTreeSet::new();
        for j in 0..14{
            last4.insert(input_chars[i - j]);
        }
        if last4.len() == 14 {
            result2 = i + 1;
            break;
        }
    }

    println!("{}", result2);
}
