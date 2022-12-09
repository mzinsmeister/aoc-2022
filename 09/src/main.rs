use std::{fs::read_to_string, str::FromStr, error::Error, string::ParseError, collections::HashSet, hash::Hash};

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Left,
    Right,
    Down
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Self::Right),
            "L" => Ok(Self::Left),
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            _ => Err(())
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Instruction {
    direction: Direction,
    steps: usize,
}

impl FromStr for Instruction {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir_str, steps_str) = s.split_once(" ").unwrap();
        Ok(Self{direction: dir_str.parse().unwrap(), steps: steps_str.parse()?})
    }
}

fn pretty_print(positions: [(i32,i32); 10], min: (i32, i32), max: (i32, i32)) {
    for y in (min.1..=max.1).rev() {
        for x in min.0..=max.0 {
            if let Some((index, _)) = positions.iter().enumerate().find(|(_, &i)| i == (x,y)) {
                if index == 0 {
                    print!("H");
                } else if index == 9 {
                    print!("T");
                } else {
                    print!("{}", index);
                }
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
    println!("\n");
}

fn main() {
    let input_str = read_to_string("input.txt").unwrap();

    let inputs: Vec<Instruction> = input_str.lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect();

    let mut seen_positions: HashSet<(i32, i32)> = HashSet::new();
    seen_positions.insert((0,0));

    let (mut head_x, mut head_y): (i32, i32) = (0, 0);
    let (mut tail_x, mut tail_y): (i32, i32) = (0, 0);

    for instruction in &inputs {
        for _ in 0..instruction.steps {
            let (previous_head_x, previous_head_y) = (head_x, head_y);
            match instruction.direction {
                Direction::Up => head_y += 1,
                Direction::Left => head_x -= 1,
                Direction::Right => head_x += 1,
                Direction::Down => head_y -= 1,
            }
            if head_x.abs_diff(tail_x) > 1 || head_y.abs_diff(tail_y) > 1 {
                (tail_x, tail_y) = (previous_head_x, previous_head_y);
                seen_positions.insert((tail_x, tail_y));
            }
        }
    }


    println!("{}", seen_positions.len());

    let mut seen_positions2: HashSet<(i32, i32)> = HashSet::new();
    seen_positions2.insert((0,0));

    let mut positions: [(i32, i32); 10] = [(0,0); 10];

    for (_, instruction) in inputs.iter().enumerate() { // enumerate for debugging
        for _ in 0..instruction.steps {
            let (mut move_x, mut move_y) = (0,0);
            match instruction.direction {
                Direction::Up => move_y = 1,
                Direction::Left => move_x = -1,
                Direction::Right => move_x = 1,
                Direction::Down => move_y = -1,
            }
            positions[0].0 += move_x;
            positions[0].1 += move_y;
            for i in 1..10 {
                if positions[i - 1].0.abs_diff(positions[i].0) > 1 || positions[i - 1].1.abs_diff(positions[i].1) > 1 {
                    let diff_x = (positions[i - 1].0 - positions[i].0).min(1).max(-1);
                    let diff_y = (positions[i - 1].1 - positions[i].1).min(1).max(-1);
                    positions[i].0 += diff_x;
                    positions[i].1 += diff_y;
                } else {
                    break;
                }
            }
            seen_positions2.insert(positions[9]);
            //pretty_print(positions, (0, 0), (5, 4))
        }
        //pretty_print(positions, (-11, -5), (14, 15))
    }

    println!("{}", seen_positions2.len());

}
