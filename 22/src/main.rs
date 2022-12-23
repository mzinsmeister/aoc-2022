use std::{fs::read_to_string, collections::BTreeSet};

fn printmap(map: &Vec<(usize, Vec<bool>)>, pos: (usize, usize), dir: (i32, i32)) {
    let facing = match dir {
        (1,0) => '>',
        (0,1) => 'v',
        (-1,0) => '<',
        (0,-1) => '^',
        _ => panic!()
    };
    for (y, line) in map.iter().enumerate() {
        for _ in 0..line.0 {
            print!(" ");
        }
        for (x, &char) in line.1.iter().enumerate() {
            if x == pos.0 && y == pos.1 {
                print!("{}", facing);
            } else {
                if char {
                    print!("#");
                } else {
                    print!(".");
                }
            }
        }
        println!();
    }
    println!();
}

fn rotate_coords_right(mut curr: (usize, usize), size: usize, times: usize) -> (usize, usize) {
    for _ in 0..times {
        curr = (curr.1, (-(curr.0 as i64)).rem_euclid(size as i64) as usize)
    }
    curr
}

fn main() {
    let input_str = read_to_string("demo.txt").unwrap();
    
    let (map_str, instructions_str) = input_str.split_once("\n\n").unwrap();

    let map = map_str.lines()
        .map(|l| (l.chars().take_while(|c| *c == ' ').count(), 
                        l.chars().skip_while(|c| *c == ' ')
                            .map(|c| if c == '#' {true} else {false})
                            .collect()))
        .collect::<Vec<(usize, Vec<bool>)>>();

    let instructions = instructions_str.lines().next().unwrap()
        .split_inclusive(&['L', 'R'])
        .filter(|l| !l.is_empty())
        .map(|i| if i.ends_with(&['L', 'R']) { 
                (i[0..i.len()-1].parse().unwrap(), i[i.len()-1..i.len()].chars().next().unwrap())
            } else {
                (i.parse().unwrap(), ' ')
            })
        .collect::<Vec<(usize, char)>>();

    let (mut x, mut y): (usize, usize) = (map[0].1.iter().position(|e| !e).unwrap(), 0) ;
    let (mut dir_x, mut dir_y): (i32, i32) = (1, 0);
    for &(n, i) in instructions.iter() {
        //println!("{}, {}", n, i);
        //printmap(&map, (x,y), (dir_x, dir_y));
        //println!("\n\n");
        for _ in 0..n {
            let (next_x, next_y) = if dir_x == 0 {
                let mut next_y = (y as i32 + dir_y).rem_euclid(map.len() as i32) as usize;
                let x_shift = map[y].0 as i32 - map[next_y].0 as i32;
                let mut next_x = x as i32 + x_shift;
                if dir_y == -1 && (next_x < 0 || next_x >= map[next_y].1.len() as i32) {
                    next_y = y;
                    next_x = x as i32;
                    let mut next_x_2 = (map[next_y].0 as i32 - map[next_y + 1].0 as i32) + next_x;
                    while next_y < map.len() - 1 && next_x_2 >= 0 && next_x_2 < map[next_y + 1].1.len() as i32 {
                        let x_shift = map[next_y].0 as i32 - map[next_y + 1].0 as i32;
                        next_y += 1;
                        next_x = next_x as i32 + x_shift;
                        if next_y < map.len() - 1 {
                            next_x_2 = (map[next_y].0 as i32 - map[next_y + 1].0 as i32) + next_x;
                        }
                    }
                }
                if dir_y == 1 && (next_x < 0 || next_x >= map[next_y].1.len() as i32) {
                    next_x = x as i32;
                    next_y = y;
                    let mut next_x_2 = (map[next_y].0 as i32 - map[next_y - 1].0 as i32) + next_x;
                    while next_y > 0 && next_x_2 >= 0 && next_x_2 < map[next_y - 1].1.len() as i32 {
                        let x_shift = map[next_y].0 as i32 - map[next_y - 1].0 as i32;
                        next_y -= 1;
                        next_x = next_x as i32 + x_shift;
                        if next_y > 0 {
                            next_x_2 = (map[next_y].0 as i32 - map[next_y - 1].0 as i32) + next_x;
                        }
                    }
                }
                (next_x as usize, next_y)
            } else {
                ((x as i32 + dir_x).rem_euclid(map[y].1.len() as i32) as usize, y)
            };
            if map[next_y].1[next_x] {
                break;
            } else {
                (x, y) = (next_x, next_y);
            }
        }
        if i == 'R' {
            (dir_x, dir_y) = (-dir_y, dir_x);
        } else if i == 'L' {
            (dir_x, dir_y) = (dir_y, -dir_x);
        }
    }
    let facing = match (dir_x,dir_y) {
        (1,0) => 0,
        (0,1) => 1,
        (-1,0) => 2,
        (0,-1) => 3,
        _ => panic!()
    };
    let row = y + 1;
    let column = x + 1 + map[y].0;
    println!("{}", 1000 * row + 4 * column + facing );


    // Find cube patch borders
    let mut y_borders: Vec<usize> = Vec::new();
    y_borders.push(0);
    for i in 1..map.len() {
        if map[i].0 != map[i-1].0 || map[i].1.len() != map[i-1].1.len() {
            y_borders.push(i);
        }
    }
    let patch_size = y_borders[1];
    let mut patches: Vec<(usize, usize)> = Vec::new();
    for border in y_borders {
        let mut pos = map[border].0;
        while pos - map[border].0 < map[border].1.len() {
            patches.push((pos / patch_size, border / patch_size));
            pos += patch_size;
        }
    }

    // Find cube edges

    let mut edges: Vec<[usize; 4]> = Vec::new();
    for _ in patches.iter() {
        edges.push([usize::MAX; 4]);
    }
    for (i, patch) in patches.iter().enumerate() {
        for (j, patch2) in patches.iter().enumerate() {
            if patch.0 > patch2.0 && patch.0 - patch2.0 == 1 && patch.1 == patch2.1 {
                edges[i][2] = j;
                edges[j][0] = i;
            }
            if patch.0 < patch2.0 && patch2.0 - patch.0 == 1 && patch.1 == patch2.1 {
                edges[i][0] = j;
                edges[j][2] = i;
            }
            if patch.1 > patch2.1 && patch.1 - patch2.1 == 1 && patch.0 == patch2.0 {
                edges[i][3] = j;
                edges[j][1] = i;
            }
            if patch.1 < patch2.1 && patch2.1 - patch.1 == 1 && patch.0 == patch2.0 {
                edges[i][1] = j;
                edges[j][3] = i;
            }
        }
    }

    let mut open_edges = true;
    while open_edges {
        open_edges = false;
        for e0 in 0..patches.len() {
            for i in 0..4 {
                let e = edges[e0][i];
                if e != usize::MAX {
                    let rev = edges[e].iter().position(|et| *et == e0).unwrap();
                    for j in [(rev+1) % 4, (rev + 3) % 4] {
                        let e2 = edges[e][j];
                        if e2 != usize::MAX {
                            let rev2 = edges[e2].iter().position(|et| *et == e).unwrap();
                            let t = (j as i8 - (rev as i8 - ((i + 2) % 4) as i8)).rem_euclid(4) as usize;
                            edges[e0][t] = e2;
                            let t2 = (rev as i8 - (((j + 2) % 4) as i8 - rev2 as i8)).rem_euclid(4) as usize;
                            edges[e2][t2] = e0;
                        }
                    }
                } else {
                    open_edges = true;
                }
            }
        }
    }

    // do the actual operation

    let (mut x, mut y): (usize, usize) = (map[0].1.iter().position(|e| !e).unwrap(), 0) ;
    let (mut dir_x, mut dir_y): (i32, i32) = (1, 0);
    for (n, i) in instructions {
        //println!("{}, {}", n, i);
        //printmap(&map, (x,y), (dir_x, dir_y));
        //println!("\n\n");
        for _ in 0..n {
            let patch = patches.iter().position(|p| *p == ((x + map[y].0) / patch_size, y / patch_size)).unwrap();
            let (next_x, next_y) = if dir_x == 0 {
                let mut next_y = y as i32 + dir_y;
                let x_shift = map[y].0 as i32 - map[(next_y % map.len() as i32) as usize].0 as i32;
                let mut next_x = x as i32 + x_shift;
                if dir_y == -1 && next_y < 0 || (next_x < 0 || next_x >= map[next_y as usize].1.len() as i32) {
                    let next_patch = edges[patch][3];
                    let rev = edges[next_patch].iter().position(|p| *p == patch).unwrap();
                    let rotate_times = 3 - rev + if (3-rev) % 2 == 0 { 2 } else { 0 };
                    for _ in 0..rotate_times {
                        (dir_x, dir_y) = (-dir_y, dir_x);
                    }
                    let relative_coords = ((x + map[y].0) - patches[patch].0 * patch_size, patch_size - 1);
                    let new_relative_coords = rotate_coords_right(relative_coords, patch_size, rotate_times);
                    let new_y = (patches[next_patch].1 * patch_size + new_relative_coords.1) as i32;
                    (next_x, next_y) = ((patches[next_patch].0 * patch_size + new_relative_coords.0) as i32 - map[new_y as usize].0 as i32,
                                        new_y);
                }
                if dir_y == 1 && (next_y >= map.len() as i32 || (next_x < 0 || next_x >= map[next_y as usize].1.len() as i32)) {
                    let next_patch = edges[patch][1];
                    let rev = edges[next_patch].iter().position(|p| *p == patch).unwrap();
                    let rotate_times = (1 - rev as i32 + if (3-rev) % 2 == 0 { 2 } else { 0 }).rem_euclid(4) as usize;
                    for _ in 0..rotate_times {
                        (dir_x, dir_y) = (-dir_y, dir_x);
                    }
                    let relative_coords = ((x + map[y].0) - patches[patch].0 * patch_size, 0);
                    let new_relative_coords = rotate_coords_right(relative_coords, patch_size, rotate_times);
                    let new_y = (patches[next_patch].1 * patch_size + new_relative_coords.1) as i32;
                    (next_x, next_y) = ((patches[next_patch].0 * patch_size + new_relative_coords.0) as i32 - map[new_y as usize].0 as i32,
                                        new_y);
                }
                (next_x as usize, next_y as usize)
            } else {
                if x == 0 && dir_x == -1 {
                    let next_patch = edges[patch][2];
                    let rev = edges[next_patch].iter().position(|p| *p == patch).unwrap();
                    let rotate_times = (2 - rev as i32 + if (2-rev as i32) % 2 == 0 { 2 } else { 0 }).rem_euclid(4) as usize;
                    for _ in 0..rotate_times {
                        (dir_x, dir_y) = (-dir_y, dir_x);
                    }
                    let relative_coords = (patch_size - 1, y - patches[patch].1);
                    let new_relative_coords = rotate_coords_right(relative_coords, patch_size, rotate_times);
                    let new_y = (patches[next_patch].1 * patch_size + new_relative_coords.1) as usize;
                    ((patches[next_patch].0 * patch_size + new_relative_coords.0) as usize - map[new_y].0, new_y)
                } else if x == map[y].1.len() - 1 && dir_x == 1 {
                    let next_patch = edges[patch][0];
                    let rev = edges[next_patch].iter().position(|p| *p == patch).unwrap();
                    let rotate_times = (0 - rev as i32 + if (0-rev as i32) % 2 == 0 { 2 } else { 0 }).rem_euclid(4) as usize;
                    for _ in 0..rotate_times {
                        (dir_x, dir_y) = (-dir_y, dir_x);
                    }
                    let relative_coords = (0, y - patches[patch].1);
                    let new_relative_coords = rotate_coords_right(relative_coords, patch_size, rotate_times);
                    let new_y = (patches[next_patch].1 * patch_size + new_relative_coords.1) as usize;
                    ((patches[next_patch].0 * patch_size + new_relative_coords.0) as usize - map[new_y].0, new_y)
                } else {
                    ((x as i32 + dir_x) as usize, y)
                }
            };
            if map[next_y].1[next_x] {
                break;
            } else {
                (x, y) = (next_x, next_y);
            }
        }
        if i == 'R' {
            (dir_x, dir_y) = (-dir_y, dir_x);
        } else if i == 'L' {
            (dir_x, dir_y) = (dir_y, -dir_x);
        }
    }
    let facing = match (dir_x,dir_y) {
        (1,0) => 0,
        (0,1) => 1,
        (-1,0) => 2,
        (0,-1) => 3,
        _ => panic!()
    };
    let row = y + 1;
    let column = x + 1 + map[y].0;
    println!("{}", 1000 * row + 4 * column + facing );
}
