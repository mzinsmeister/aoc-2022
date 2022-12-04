use std::fs::read_to_string;

fn main() {
    let input_str = read_to_string("input.txt").unwrap();

    let result = input_str
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l|l.split_once(",").unwrap())
        .map(|(left, right)| (left.split_once("-").unwrap(), right.split_once("-").unwrap()))
        .map(|pair| ((pair.0.0.parse::<u32>().unwrap(), pair.0.1.parse::<u32>().unwrap()), (pair.1.0.parse::<u32>().unwrap(), pair.1.1.parse::<u32>().unwrap())))
        .filter(|pair| (pair.0.0 <= pair.1.0 && pair.0.1 >= pair.1.1) ||(pair.0.0 >= pair.1.0 && pair.0.1 <= pair.1.1))
        .count();

    println!("{}", result);

    let result2 = input_str
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l|l.split_once(",").unwrap())
        .map(|(left, right)| (left.split_once("-").unwrap(), right.split_once("-").unwrap()))
        .map(|pair| ((pair.0.0.parse::<u32>().unwrap(), pair.0.1.parse::<u32>().unwrap()), (pair.1.0.parse::<u32>().unwrap(), pair.1.1.parse::<u32>().unwrap())))
        .filter(|pair| !(pair.0.0 > pair.1.1 || pair.0.1 < pair.1.0 || pair.1.0 > pair.0.1 || pair.1.1 < pair.0.0))
        .count();

    println!("{}", result2);
}
