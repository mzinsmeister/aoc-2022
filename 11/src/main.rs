use std::{fs::read_to_string, collections::VecDeque};

#[derive(Clone, Copy)]
enum Operation {
    Add(u64),
    Mul(u64),
    Square
}

impl Operation {
    fn execute(&self, old: u64) -> u64 {
        match self {
            Operation::Add(n) => old + n,
            Operation::Mul(n) => old * n,
            Operation::Square => old * old
        }
    }

    fn execute2(&self, old: &Vec<u16>, mods: &Mods) -> Vec<u16> {
        match self {
            Operation::Add(n) => mods.add(old, *n),
            Operation::Mul(n) => mods.mul(old, *n),
            Operation::Square => mods.square(old),
        }
    }

    fn parse(input: &str) -> Operation {
        let (operator_str, second_operand_str) = input.split_once(" ").unwrap();
        match operator_str {
            "+" => Operation::Add(second_operand_str.parse().unwrap()),
            "*" => match second_operand_str {
                "old" => Operation::Square,
                _ => Operation::Mul(second_operand_str.parse().unwrap()),
            }
            _ => panic!("illegal operator {}", operator_str)
        }
    }
}

struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    test_divisible_by: u64,
    test_true_throw_to: usize,
    test_false_throw_to: usize,
}

impl Monkey {
    fn parse(input: &str) -> Monkey {
        let mut lines_iter = input.lines();
        lines_iter.next();
        let items = lines_iter.next().unwrap()
                                .split_once(": ").unwrap().1
                                .split(", ")
                                .map(|n| n.parse::<u64>().unwrap())
                                .collect::<VecDeque<u64>>();
        let line = lines_iter.next().unwrap();
        let operation = Operation::parse(&line[23..line.len()]);
        let line = lines_iter.next().unwrap();
        let test_divisible_by = line[21..line.len()].parse().unwrap();
        let line = lines_iter.next().unwrap();
        let test_true_throw_to = line[29..line.len()].parse().unwrap();
        let line = lines_iter.next().unwrap();
        let test_false_throw_to = line[30..line.len()].parse().unwrap();
        Monkey { items, operation, test_divisible_by, test_true_throw_to, test_false_throw_to }
    }
}

struct Mods {
    mods: Vec<u16>
}

impl Mods {
    fn construct(&self, value: u64) -> Vec<u16> {
        let mut result: Vec<u16> = Vec::with_capacity(self.mods.len());
        for &m in self.mods.iter() {
            result.push((value % m as u64) as u16);
        }
        result
    }

    fn add(&self, v_pre: &Vec<u16>, value: u64) -> Vec<u16> {
        let mut result: Vec<u16> = Vec::with_capacity(v_pre.len());
        for (&v, &m) in v_pre.iter().zip(self.mods.iter()) {
            result.push(((v as u64 + value) % m as u64) as u16);
        }
        result
    }

    fn mul(&self, v_pre: &Vec<u16>, value: u64) -> Vec<u16> {
        let mut result: Vec<u16> = Vec::with_capacity(v_pre.len());
        for (&v, &m) in v_pre.iter().zip(self.mods.iter()) {
            result.push(((v as u64 * value) % m as u64) as u16);
        }
        result
    }

    fn square(&self, v_pre: &Vec<u16>) -> Vec<u16> {
        let mut result: Vec<u16> = Vec::with_capacity(v_pre.len());
        for (&v, &m) in v_pre.iter().zip(self.mods.iter()) {
            result.push((v * v) % m);
        }
        result
    }
}

struct Monkey2 {
    items: VecDeque<Vec<u16>>,
    operation: Operation,
    test_true_throw_to: usize,
    test_false_throw_to: usize,
}

impl Monkey2 {
    fn from_monkey(monkey: &Monkey, mods: &Mods) -> Monkey2 {
        Monkey2 {
            items: monkey.items.iter().map(|i| mods.construct(*i)).collect(), 
            operation: monkey.operation, 
            test_true_throw_to: monkey.test_true_throw_to, 
            test_false_throw_to: monkey.test_false_throw_to 
        }
    }
}

fn main() {
    let input_str = read_to_string("input.txt").unwrap();

    let mut monkeys = input_str.split("\n\n")
        .filter(|b| !b.is_empty())
        .map(|b| Monkey::parse(b))
        .collect::<Vec<Monkey>>();

    let mut inspected_items = vec![0usize; monkeys.len()];
    
    for _ in 0..20 { // rounds
        for i in 0..monkeys.len() {
            while !monkeys[i].items.is_empty() {
                inspected_items[i] += 1;
                let old_level = monkeys[i].items.pop_front().unwrap();
                let new_level = monkeys[i].operation.execute(old_level) / 3;
                if &new_level % &monkeys[i].test_divisible_by == 0 {
                    let new_monkey = monkeys[i].test_true_throw_to;
                    monkeys[new_monkey].items.push_back(new_level);
                } else {
                    let new_monkey = monkeys[i].test_false_throw_to;
                    monkeys[new_monkey].items.push_back(new_level);
                }
            }
        }
    }

    inspected_items.sort_by(|a, b| b.cmp(a));

    println!("{} and {} : {}", inspected_items[0], inspected_items[1], inspected_items[0] * inspected_items[1]);

    // PART 2

    let monkeys2 = input_str.split("\n\n")
        .filter(|b| !b.is_empty())
        .map(|b| Monkey::parse(b))
        .collect::<Vec<Monkey>>();

    let mods = Mods { mods: monkeys2.iter().map(|m| m.test_divisible_by as u16).collect() };
    
    let mut monkeys2 = monkeys2.iter().map(|m| Monkey2::from_monkey(m, &mods)).collect::<Vec<Monkey2>>();

    let mut inspected_items2 = vec![0usize; monkeys2.len()];

    for _ in 0..10000 { // rounds
        for i in 0..monkeys2.len() {
            while !monkeys2[i].items.is_empty() {
                inspected_items2[i] += 1;
                let old_level = monkeys2[i].items.pop_front().unwrap();
                let new_level = monkeys2[i].operation.execute2(&old_level, &mods);
                if new_level[i] == 0 {
                    let new_monkey = monkeys2[i].test_true_throw_to;
                    monkeys2[new_monkey].items.push_back(new_level);
                } else {
                    let new_monkey = monkeys2[i].test_false_throw_to;
                    monkeys2[new_monkey].items.push_back(new_level);
                }
            }
        }
    }

    inspected_items2.sort_by(|a, b| b.cmp(a));
   
    println!("{} and {} : {}", inspected_items2[0], inspected_items2[1], inspected_items2[0] * inspected_items2[1]);
}
