use std::{fs::read_to_string, collections::{HashSet, BTreeSet}};

fn main() {
    let input_str = read_to_string("input.txt").unwrap();

    let input = input_str
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut num_iter = l.split(",").map(|n| n.parse::<i32>().unwrap());
            (num_iter.next().unwrap(), num_iter.next().unwrap(), num_iter.next().unwrap())
        })
        .collect::<HashSet<(i32, i32, i32)>>();

    let mut sum = 0;
    for cube in &input {
        for x in [-1, 1] {
            if !input.contains(&(cube.0+x, cube.1, cube.2)) {
                sum += 1;
            }
        }
        for y in [-1, 1] {
            if !input.contains(&(cube.0, cube.1+y, cube.2)) {
                sum += 1;
            }
        }
        for z in [-1, 1] {
            if !input.contains(&(cube.0, cube.1, cube.2+z)) {
                sum += 1;
            }
        }
    }

    println!("{}", sum);

    let mut air_sets: Vec<BTreeSet<(i32, i32, i32)>> = Vec::new();
    let mut sum = 0;

    for z in -1..=20 {
        for y in -1..=20 {
            for x in -1..=20 {
                if !input.contains(&(x,y,z)) {
                    let mut neighbors: BTreeSet<usize> = BTreeSet::new();
                    for (i, set) in air_sets.iter().enumerate() {
                        for x_diff in [-1, 1] {
                            if set.contains(&(x + x_diff, y, z)) {
                                neighbors.insert(i);
                            }
                        }
                        for y_diff in [-1, 1] {
                            if set.contains(&(x, y + y_diff, z)) {
                                neighbors.insert(i);
                            }
                        }
                        for z_diff in [-1, 1] {
                            if set.contains(&(x, y, z + z_diff)) {
                                neighbors.insert(i);
                            }
                        }
                    }
                    if neighbors.len() == 1 {
                        let n1_index = *neighbors.iter().next().unwrap();
                        let neighbor = &mut air_sets[n1_index];
                        neighbor.insert((x,y,z));
                    } else if neighbors.len() > 1 {
                        let n1_index = *neighbors.iter().next().unwrap();
                        let mut neighbor = air_sets[n1_index].clone();
                        for n in neighbors.iter().skip(1) {
                            neighbor.extend(air_sets[*n].iter());
                        }
                        neighbor.insert((x,y,z));
                        air_sets[n1_index] = neighbor;
                        for n in (neighbors.iter().skip(1)).rev() {
                            air_sets.remove(*n);
                        }
                    } else {
                        air_sets.push(BTreeSet::from([(x,y,z)]));
                    }
                }
                assert!(input.contains(&(x,y,z)) || air_sets.iter().any(|s| s.contains(&(x,y,z))));
            }
        }
    }

    let outer = air_sets.iter().find(|m| m.contains(&(0,0,0))).unwrap().clone();
    // println!("{:#?}", &air_sets);
    for cube in &input {
        for x in [-1, 1] {
            if !input.contains(&(cube.0+x, cube.1, cube.2)) && outer.contains(&(cube.0+x, cube.1, cube.2)) {
                sum += 1;
            }
        }
        for y in [-1, 1] {
            if !input.contains(&(cube.0, cube.1+y, cube.2)) && outer.contains(&(cube.0, cube.1+y, cube.2)) {
                sum += 1;
            }
        }
        for z in [-1, 1] {
            if !input.contains(&(cube.0, cube.1, cube.2+z)) && outer.contains(&(cube.0, cube.1, cube.2+z)) {
                sum += 1;
            }
        }
    }

    println!("{}", sum);
}
