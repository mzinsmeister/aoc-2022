use std::{fs::read_to_string, collections::BTreeSet};

fn main() {
    let input_str = read_to_string("input.txt").unwrap();

    let input: Vec<(Vec<char>, Vec<char>)> = input_str
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| {
                let mut chars1 = l[0..(l.len() / 2)].chars().collect::<Vec<char>>();
                let mut chars2 = l[(l.len() / 2)..l.len()].chars().collect::<Vec<char>>();
                chars1.sort();
                chars2.sort();
                (chars1, chars2)
            })
        .collect();

    let result = input.iter().map(|(left, right)| {
            let mut inters: BTreeSet<char> = BTreeSet::new();
            for c in left {
                if right.contains(c) {
                    inters.insert(*c);
                }
            }
            inters
        })
        .map(|i| i.iter().map(|c| if (*c as u32) > ('a' as u32) {
            (*c as u32 - ('a' as u32)) + 1
        } else {
            (*c as u32 - ('A' as u32)) + 27
        }).sum::<u32>())
        .sum::<u32>();

    println!("{}", result);

    let input_lines = input_str
        .split("\n")
        .filter(|l| !l.is_empty())
        .collect::<Vec<&str>>();

    let result_2 = input_lines
        .chunks(3)
        .map(|ls| {
            for c in ls[0].chars() {
                if ls[1].contains(c) && ls[2].contains(c) {
                    return c
                }
            }
            panic!("not found");
        })
        .map(|c|
            if (c as u32) > ('a' as u32) {
                (c as u32 - ('a' as u32)) + 1
            } else {
                (c as u32 - ('A' as u32)) + 27
            }
        )
        .sum::<u32>();

    println!("{}", result_2);

}
