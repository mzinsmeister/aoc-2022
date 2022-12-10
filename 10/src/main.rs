use std::fs::read_to_string;

#[derive(Clone, Copy, Debug)]
enum Instruction {
    AddX(i32),
    Noop
}

impl Instruction {
    fn new(l: &str) -> Instruction {
        if l.starts_with("noop") {
            Instruction::Noop
        } else {
            Instruction::AddX(l[5..l.len()].parse().unwrap())
        }
    }
}

fn main() {
    let input_str = read_to_string("input.txt").unwrap();

    let instructions = input_str
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| Instruction::new(l))
        .collect::<Vec<Instruction>>();

    let mut x: i32 = 1;
    let mut cycle = 1;
    let mut signal_strength_sum = 0;
    for instruction in instructions {
        if (cycle - 1) % 40 == 0 {
            print!("\n");
        }
        if x.abs_diff((cycle - 1) % 40) <= 1 {
            print!("#");
        } else {
            print!(".");
        }
        match instruction {
            Instruction::AddX(n) => {
                cycle += 1;
                if (cycle - 1) % 40 == 0 {
                    print!("\n");
                }
                if x.abs_diff((cycle - 1) % 40) <= 1 {
                    print!("#");
                } else {
                    print!(".");
                }
                if cycle % 40 == 20 {
                    //println!("{}", x * cycle);
                    signal_strength_sum += x * cycle;
                }
                cycle += 1;
                x += n;
            },
            Instruction::Noop => {
                cycle += 1;
            },
        }
        if cycle % 40 == 20 {
            //println!("{}", x * cycle);
            signal_strength_sum += x * cycle;
        }
    }
    println!("");
    println!("{}", signal_strength_sum);
}
