
use std::cmp::{max, min};
use std::collections::{VecDeque, HashMap};
use std::fs::read_to_string;

use crate::Shape::*;


#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
enum Shape {
    Horizontal,
    Plus,
    ReverseL,
    Vertical,
    Block,
}

impl Shape {
    fn get_height(&self) -> usize {
        match self {
            Horizontal => 1,
            Plus => 3,
            ReverseL => 3,
            Vertical => 4,
            Block => 2,
        }
    }

    fn get_width(&self) -> usize {
        match self {
            Horizontal => 4,
            Plus => 3,
            ReverseL => 3,
            Vertical => 1,
            Block => 2,
        }
    }

    fn get_shape(&self) -> Vec<u8> {
        match self {
            Horizontal => vec![0b01111000],
            Plus => vec![0b00100000, 0b01110000, 0b00100000],
            ReverseL => vec![0b00010000, 0b00010000, 0b01110000],
            Vertical => vec![0b01000000, 0b01000000, 0b01000000, 0b01000000],
            Block => vec![0b01100000, 0b01100000],
        }
    }

    fn check_collision(&self, layers: &VecDeque<u8>, position: usize, depth: i32) -> bool {
        let shape_bytes = self.get_shape().iter()
            .map(|l| l >> position)
            .collect::<Vec<u8>>();
        let skip_layers = max(0, -depth) as usize;
        let depth_layers = max(0, depth) as usize;
        for (i, shape_layer) in shape_bytes.iter().skip(skip_layers).enumerate() {
            if (layers[i + depth_layers] & shape_layer) != 0 {
                return true;
            }
        }
        false
    }
}

fn print_buffer(rock_buffer: &VecDeque<u8>) {
    for line in rock_buffer {
        for i in (0..7).rev() {
            if (line & 1 << i) != 0 {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}

fn main() {
    const SHAPE_ORDER: [Shape; 5] = [Horizontal, Plus, ReverseL, Vertical, Block];

    let input_str = read_to_string("input.txt").unwrap();
    let jet_pattern = input_str.lines().next().unwrap().chars()
        .map(|c| match c {
            '<' => -1,
            '>' => 1,
            _ => panic!("unknown char {}", c)
        })
        .collect::<Vec<i32>>();

    let mut rock_buffer: VecDeque<u8> = VecDeque::new();

    rock_buffer.push_front(0xff);

    let mut jet_pattern_pos = 0;

    for i in 0..2022 {
        let stone = SHAPE_ORDER[i % 5];

        let mut depth = -3 - stone.get_height() as i32;
        let mut pos = 2usize;

        loop {
            let movement = jet_pattern[jet_pattern_pos];
            jet_pattern_pos = (jet_pattern_pos + 1) % jet_pattern.len();
            let mut new_pos = pos as i32 + movement;
            if new_pos + stone.get_width() as i32 > 7 || new_pos < 0 {
                new_pos = pos as i32;
            }
            if !stone.check_collision(&rock_buffer, new_pos as usize, depth) {
                pos = new_pos as usize;
            }
            if stone.check_collision(&rock_buffer, pos, depth + 1) {
                break;
            } else {
                depth += 1;
            }
        }
        let stone_bytes = stone.get_shape().iter().map(|l| l >> pos).collect::<Vec<u8>>();
        let already_done = max(0, -depth);
        for i in max(depth, 0)..depth+stone_bytes.len() as i32 {
            rock_buffer[i as usize] = rock_buffer[i as usize] | stone_bytes[(i - max(depth, 0) + already_done) as usize];
        }
        for i in (0..-depth).rev() {
            rock_buffer.push_front(stone_bytes[i as usize])
        }
        //print_buffer(&rock_buffer);
        //println!("\n");
    }

    println!("{}", rock_buffer.len() - 1);

    let mut rock_buffer: VecDeque<u8> = VecDeque::new();
    let mut memozation_table: HashMap<([u8;3], usize, Shape), (usize, usize)> = HashMap::new();

    rock_buffer.push_front(0xff);
    rock_buffer.push_front(0xff);
    rock_buffer.push_front(0xff);
    rock_buffer.push_front(0xff);
    rock_buffer.push_front(0xff);


    let mut jet_pattern_pos = 0;
    let mut deleted = 0usize;
    let mut delete_until = 0usize;
    let mut counter = 0;
    let mut cycle_start = 0;
    let mut cycle_start_size = 0;
    let mut cycle_length = 0;
    let mut cycle_size = 0;
    // let mut highest_set: [usize; 7] = [0,0,0,0,0,0,0];

    for i in 0..1000000000000 {
        let stone = SHAPE_ORDER[i % 5];

        let mut depth = -3 - stone.get_height() as i32;
        let mut pos = 2usize;

        loop {
            let movement = jet_pattern[jet_pattern_pos];
            jet_pattern_pos = (jet_pattern_pos + 1) % jet_pattern.len();
            let mut new_pos = pos as i32 + movement;
            if new_pos + stone.get_width() as i32 > 7 || new_pos < 0 {
                new_pos = pos as i32;
            }
            if !stone.check_collision(&rock_buffer, new_pos as usize, depth) {
                pos = new_pos as usize;
            }
            if stone.check_collision(&rock_buffer, pos, depth + 1) {
                break;
            } else {
                depth += 1;
            }
        }
        let stone_bytes = stone.get_shape().iter().map(|l| l >> pos).collect::<Vec<u8>>();
        let already_done = max(0, -depth);
        for i in max(depth, 0)..depth+stone_bytes.len() as i32 {
            rock_buffer[i as usize] = rock_buffer[i as usize] | stone_bytes[(i - max(depth, 0) + already_done) as usize];
            if (rock_buffer[i as usize] | rock_buffer[(i + 1) as usize] | rock_buffer[(i + 2) as usize]) & 0b01111111 == 0b01111111 {
                delete_until = max(rock_buffer.len() + deleted - i as usize - 4, delete_until);
                /*if i == 0 {
                    counter += 1;
                    println!("{}", counter);
                }*/
            }
        }
        for i in (0..-depth).rev() {
            rock_buffer.push_front(stone_bytes[i as usize]);
        }
        
        if deleted < delete_until {
            while deleted < delete_until {
                rock_buffer.pop_back();
                deleted += 1;
            }
        }

        if rock_buffer[0] | rock_buffer[1] | rock_buffer[2] == 0b01111111 || rock_buffer[1] | rock_buffer[2] | rock_buffer[3] == 0b01111111 {
            let key = ([rock_buffer[0], rock_buffer[1], rock_buffer[2]], jet_pattern_pos, stone);
            let last = memozation_table.insert(key, (i, deleted + rock_buffer.len()));
            if let Some(last) = last {
                cycle_start = last.0;
                cycle_start_size = last.1;
                cycle_length = i - last.0;
                cycle_size = (deleted + rock_buffer.len()) -last.1;
                break;
            }
        }
        
        //print_buffer(&rock_buffer);
        //println!("\n");
    }

        let cycle_times = (1000000000000 - cycle_start) / cycle_length;
        let cycles_size = cycle_times * cycle_size;
        let total_size_1 = cycles_size + cycle_start_size - rock_buffer.len();
        let rest = (1000000000000 - cycle_start) % cycle_length;

        for i in (1000000000000-rest + 1)..1000000000000 {
            let stone = SHAPE_ORDER[i % 5];
    
            let mut depth = -3 - stone.get_height() as i32;
            let mut pos = 2usize;
    
            loop {
                let movement = jet_pattern[jet_pattern_pos];
                jet_pattern_pos = (jet_pattern_pos + 1) % jet_pattern.len();
                let mut new_pos = pos as i32 + movement;
                if new_pos + stone.get_width() as i32 > 7 || new_pos < 0 {
                    new_pos = pos as i32;
                }
                if !stone.check_collision(&rock_buffer, new_pos as usize, depth) {
                    pos = new_pos as usize;
                }
                if stone.check_collision(&rock_buffer, pos, depth + 1) {
                    break;
                } else {
                    depth += 1;
                }
            }
            let stone_bytes = stone.get_shape().iter().map(|l| l >> pos).collect::<Vec<u8>>();
            let already_done = max(0, -depth);
            for i in max(depth, 0)..depth+stone_bytes.len() as i32 {
                rock_buffer[i as usize] = rock_buffer[i as usize] | stone_bytes[(i - max(depth, 0) + already_done) as usize];
            }
            for i in (0..-depth).rev() {
                rock_buffer.push_front(stone_bytes[i as usize])
            }
            //print_buffer(&rock_buffer);
            //println!("\n");
        }

    println!("{}", total_size_1 + rock_buffer.len() - 5);
}
