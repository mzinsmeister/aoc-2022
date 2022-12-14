use std::fs::read_to_string;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Particle {
    Empty,
    Stone,
    Sand
}

impl Particle {
    fn print(&self) {
        match self {
            Particle::Empty => print!("."),
            Particle::Stone => print!("#"),
            Particle::Sand => print!("o"),
        }
    }
}

fn main() {
    
    let input_str = read_to_string("input.txt").unwrap();

    let input = input_str.lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.split(" -> ")
                        .map(|t| {
                            let (x_s, y_s) = t.split_once(",").unwrap();
                            (x_s.parse::<usize>().unwrap(), y_s.parse::<usize>().unwrap())
                        })
                        .collect::<Vec<(usize, usize)>>()
            )
        .collect::<Vec<Vec<(usize, usize)>>>();

    let max_y = *input.iter()
        .map(|v| v.iter().map(|(x,y)| y).max().unwrap()).max().unwrap() + 1;
    let min_y = 0;
    let max_x = 500 + max_y + 10;
    let min_x = 500 - max_y - 10;
    

    let source_x = 500 - min_x;
    
    let mut field: Vec<Vec<Particle>>  = Vec::new();

    for _ in min_y..=max_y {
        let mut row = Vec::new();
        for _ in min_x..=max_x {
            row.push(Particle::Empty);
        }
        field.push(row);
    }

    for lines in &input {
        let mut last_point = lines[0];
        for next_point in lines.iter().skip(1) {
            if last_point.0 < next_point.0 {
                for next_x in last_point.0..=next_point.0 {
                    field[next_point.1 - min_y][next_x - min_x] = Particle::Stone;
                }
            } else if last_point.0 > next_point.0 {
                for next_x in next_point.0..=last_point.0 {
                    field[next_point.1 - min_y][next_x - min_x] = Particle::Stone;
                }
            } else if last_point.1 < next_point.1 {
                for next_y in last_point.1..=next_point.1 {
                    field[next_y - min_y][next_point.0 - min_x] = Particle::Stone;
                }
            } else {
                for next_y in next_point.1..=last_point.1 {
                    field[next_y - min_y][next_point.0 - min_x] = Particle::Stone;
                }
            }
            last_point = *next_point;
        }
    }

    let mut result1 = 0;
    let mut done = false;
    while !done {
        let (mut x, mut y) = (500,0);
        loop {
            if field[y + 1][x - min_x] == Particle::Empty {
                y = y + 1;
            } else if field[y + 1][x - min_x - 1] == Particle::Empty {
                y = y + 1;
                x = x - 1;
            } else if field[y + 1][x - min_x + 1] == Particle::Empty {
                y = y + 1;
                x = x + 1;
            } else {
                // nowhere to move. Write to field
                field[y][x - min_x] = Particle::Sand;
                result1 += 1;
                break;
            }
            
            if y == max_y {
                done = true;
                break;
            }
        }
    }

    println!("{}", result1);

    let mut field2 = field.clone();

    let mut result2 = 0;
    let mut done2 = false;
    while !done2 {
        let (mut x, mut y) = (500,0);
        loop {
            if field2[y + 1][x - min_x] == Particle::Empty {
                y = y + 1;
            } else if field2[y + 1][x - min_x - 1] == Particle::Empty {
                y = y + 1;
                x = x - 1;
            } else if field2[y + 1][x - min_x + 1] == Particle::Empty {
                y = y + 1;
                x = x + 1;
            } else {
                // nowhere to move. Write to field
                field2[y][x - min_x] = Particle::Sand;
                if y == 0 {
                    done2 = true;
                }
                break;
            }

            if y == max_y {
                field2[y][x - min_x] = Particle::Sand;
                break;
            }
        }
        result2 += 1;
    }

    println!("{}", result2 + result1);

    /*for y in 0..field2.len() {
        for x in 0..field2[0].len() {
            field2[y][x].print();
        }
        print!("\n");
    }*/

}
