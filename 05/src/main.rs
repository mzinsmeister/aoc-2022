use std::{fs::read_to_string, collections::VecDeque};


struct Move {
    from: usize,
    to: usize,
    quantity: usize
}
fn main() {
    let input_str = read_to_string("input.txt").unwrap();
    
    let (config_input_str, ops_input) = input_str.split_once("\n\n").unwrap();

    let config_input_chars = config_input_str
        .split("\n")
        .filter(|l| !l.starts_with(" 1"))
        .map(|l| l.chars()
                        .enumerate()
                        .filter(|(i, _)| i % 4 == 1)
                        .map(|(_, e)| e)
                        .map(|e| match e {
                            ' ' => Option::None,
                            c => Option::Some(c)
                        })
                        .collect::<Vec<Option<char>>>())
        .collect::<Vec<Vec<Option<char>>>>();

    let mut config: Vec<VecDeque<char>> = Vec::new();
    let mut config2: Vec<VecDeque<char>> = Vec::new();

    for _ in 0..config_input_chars[0].len() {
        config.push(VecDeque::new());
        config2.push(VecDeque::new());
    }

    for line in config_input_chars {
        for (i, char) in line.iter().enumerate() {
            if let Some(c) = char {
                config[i].push_front(*c);
                config2[i].push_front(*c);
            }
        }
    }

    ops_input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| {
            let l_numbers = l.replace("move ", "").replace("from ", "").replace("to ", "");
            let (first, rest) = l_numbers.split_once(" ").unwrap();
            let quantity = first.parse::<usize>().unwrap();
            let (next, rest) = rest.split_once(" ").unwrap();
            let from = next.parse::<usize>().unwrap() - 1;
            let to = rest.parse::<usize>().unwrap() - 1;
            Move{quantity, from, to}
        })
        .for_each(|m| {
            let mut elems: VecDeque<char> = VecDeque::new();
            for _ in 0..m.quantity {
                let elem =config[m.from].pop_back().unwrap();
                let elem2 = config2[m.from].pop_back().unwrap();
                config[m.to].push_back(elem);
                elems.push_front(elem2);
            }
            for c in elems {
                config2[m.to].push_back(c);
            }
        });

    let result = config.iter().map(|d| *d.back().unwrap()).collect::<String>();
    println!("{}", result);
    let result2 = config2.iter().map(|d| *d.back().unwrap()).collect::<String>();
    println!("{}", result2);

}
