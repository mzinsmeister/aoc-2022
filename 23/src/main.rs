use std::{fs::read_to_string, collections::{BTreeSet, BTreeMap}};

enum Direction {
    North,
    West,
    South,
    East
}

impl Direction {
    fn get_dir(&self) -> (i32, i32) {
        match self {
            Direction::North => (0, -1),
            Direction::West => (-1, 0),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
        }
    }
    fn get_check_diffs(&self) -> [(i32, i32); 3] {
        match self {
            Direction::North => [(-1, -1), (0, -1), (1, -1)],
            Direction::West => [(-1, -1), (-1, 0), (-1, 1)],
            Direction::South => [(-1, 1), (0, 1), (1, 1)],
            Direction::East => [(1, -1), (1, 0), (1, 1)],
        }
    }
}

fn main() {
    let input_str = read_to_string("input.txt").unwrap();
    let mut positions: BTreeSet<(i32, i32)> = BTreeSet::new();
    for (y, l) in input_str.lines().filter(|l| !l.is_empty()).enumerate() {
        for (x, _) in l.chars().enumerate().filter(|c| c.1 == '#') {
            positions.insert((x as i32, y as i32));
        }
    }

    let dir_list = [Direction::North, Direction::South, Direction::West, Direction::East];

    let mut dir_list_start = 0;

    let mut has_new = true;
    //while has_new {
    for _ in 0..10 {
        has_new = false;
        let mut new_positions: BTreeMap<(i32, i32), Vec<(i32, i32)>> = BTreeMap::new();
        for p in positions.iter() {
            let mut no_move = true;
            'yloop:
            for y in [-1,0,1] {
                for x in [-1,0,1] {
                    if (y != 0 || x != 0) && positions.contains(&(p.0 + x, p.1 + y)) {
                        no_move = false;
                        has_new = true;
                        break 'yloop;
                    }
                }
            }
            let mut new_pos = *p;
            if !no_move {
                for i in 0..4 {
                    let ind = (i + dir_list_start) % 4;
                    let mut can_move = true;
                    for d in dir_list[ind].get_check_diffs() {
                        if positions.contains(&(p.0+d.0, p.1+d.1)) {
                            can_move = false;
                            break;
                        }
                    }
                    if can_move {
                        let d = dir_list[ind].get_dir();
                        new_pos = (p.0+d.0, p.1+d.1);
                        break;
                    }
                }
            }
            if !new_positions.contains_key(&new_pos) {
                new_positions.insert(new_pos, Vec::new());
            }
            new_positions.get_mut(&new_pos).unwrap().push(*p);
        }
        positions.clear();
        for (p, old) in new_positions {
            if old.len() == 1 {
                positions.insert(p);
            } else {
                for p in old {
                    positions.insert(p);
                }
            }
        }
        /*let min_x = positions.iter().map(|(x,_)| x).min().unwrap();
        let max_x = positions.iter().map(|(x,_)| x).max().unwrap();
        let min_y = positions.iter().map(|(_, y)| y).min().unwrap();
        let max_y = positions.iter().map(|(_, y)| y).max().unwrap();
        for y in 0..=11 {
            for x in 0..=13 {
                if positions.contains(&(x,y)) {
                    print!("#");
                } else {
                    print!(".")
                }
            }
            println!();
        }
        println!();*/
        dir_list_start = (dir_list_start + 1) % 4;
    }
    
    let min_x = positions.iter().map(|(x,_)| x).min().unwrap();
    let max_x = positions.iter().map(|(x,_)| x).max().unwrap();
    let min_y = positions.iter().map(|(_, y)| y).min().unwrap();
    let max_y = positions.iter().map(|(_, y)| y).max().unwrap();

    let area = (max_x - min_x + 1) * (max_y - min_y + 1);
    let result = area as usize - positions.len();

    println!("{}", result);


    let mut positions: BTreeSet<(i32, i32)> = BTreeSet::new();
    for (y, l) in input_str.lines().filter(|l| !l.is_empty()).enumerate() {
        for (x, _) in l.chars().enumerate().filter(|c| c.1 == '#') {
            positions.insert((x as i32, y as i32));
        }
    }

    let dir_list = [Direction::North, Direction::South, Direction::West, Direction::East];

    let mut dir_list_start = 0;

    let mut has_new = true;
    let mut round_counter = 0;

    while has_new {
        round_counter += 1;
        has_new = false;
        let mut new_positions: BTreeMap<(i32, i32), Vec<(i32, i32)>> = BTreeMap::new();
        for p in positions.iter() {
            let mut no_move = true;
            'yloop:
            for y in [-1,0,1] {
                for x in [-1,0,1] {
                    if (y != 0 || x != 0) && positions.contains(&(p.0 + x, p.1 + y)) {
                        no_move = false;
                        has_new = true;
                        break 'yloop;
                    }
                }
            }
            let mut new_pos = *p;
            if !no_move {
                for i in 0..4 {
                    let ind = (i + dir_list_start) % 4;
                    let mut can_move = true;
                    for d in dir_list[ind].get_check_diffs() {
                        if positions.contains(&(p.0+d.0, p.1+d.1)) {
                            can_move = false;
                            break;
                        }
                    }
                    if can_move {
                        let d = dir_list[ind].get_dir();
                        new_pos = (p.0+d.0, p.1+d.1);
                        break;
                    }
                }
            }
            if !new_positions.contains_key(&new_pos) {
                new_positions.insert(new_pos, Vec::new());
            }
            new_positions.get_mut(&new_pos).unwrap().push(*p);
        }
        positions.clear();
        for (p, old) in new_positions {
            if old.len() == 1 {
                positions.insert(p);
            } else {
                for p in old {
                    positions.insert(p);
                }
            }
        }
        /*let min_x = positions.iter().map(|(x,_)| x).min().unwrap();
        let max_x = positions.iter().map(|(x,_)| x).max().unwrap();
        let min_y = positions.iter().map(|(_, y)| y).min().unwrap();
        let max_y = positions.iter().map(|(_, y)| y).max().unwrap();
        for y in 0..=11 {
            for x in 0..=13 {
                if positions.contains(&(x,y)) {
                    print!("#");
                } else {
                    print!(".")
                }
            }
            println!();
        }
        println!();*/
        dir_list_start = (dir_list_start + 1) % 4;
    }
    println!("{}", round_counter);
}
