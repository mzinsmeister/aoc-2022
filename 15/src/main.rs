use std::{fs::read_to_string, collections::HashSet, cmp::{min, max}};

#[derive(Clone, Copy)]
struct Sensor {
    coords: (i32, i32),
    closest_beacon: (i32, i32)
}

fn parse_sensor_line(input: &str) -> Sensor {
    let (x_str, rest) = input[12..input.len()].split_once(", y=").unwrap();
    let (y_str, rest) = rest.split_once(": closest beacon is at x=").unwrap();
    let (beacon_x_str, beacon_y_str) = rest.split_once(", y=").unwrap();
    Sensor { 
        coords: (x_str.parse().unwrap(), y_str.parse().unwrap()), 
        closest_beacon: (beacon_x_str.parse().unwrap(), beacon_y_str.parse().unwrap()) 
    }
}

fn main() {
    // It's not fast and wasn't quick to code either but i was super tired...
    let input_str = read_to_string("input.txt").unwrap();

    let input: Vec<Sensor> = input_str.lines()
        .filter(|l| !l.is_empty())
        .map(|l| parse_sensor_line(l))
        .collect();

    
    let mut covered_coords: HashSet<(i32, i32)> = HashSet::new();
    let mut beacon_coords: HashSet<(i32, i32)> = HashSet::new();

    for &sensor in &input {
        let x_dist = sensor.coords.0.abs_diff(sensor.closest_beacon.0) as i32;
        let y_dist = sensor.coords.1.abs_diff(sensor.closest_beacon.1) as i32;
        beacon_coords.insert(sensor.closest_beacon);
        for y in (sensor.coords.1 - (x_dist + y_dist))..=(sensor.coords.1 + (x_dist + y_dist)) {
            if y == 2000000 {
                for x in (sensor.coords.0 - (x_dist + y_dist))..=(sensor.coords.0 + (x_dist + y_dist)) {
                    if x.abs_diff(sensor.coords.0) as i32 + y.abs_diff(sensor.coords.1) as i32 <= x_dist + y_dist {
                        covered_coords.insert((x,y));
                    }
                }
            }
        }
    }

    let result1 = covered_coords.iter().filter(|&&(x,y)| !beacon_coords.contains(&(x,y))).count();
    
    println!("{}", result1);

    const LIMIT: i32 = 4000000;
    

    let mut ranges: Vec<Vec<(i32, i32)>> = Vec::new();

    for _ in 0..=LIMIT {
        ranges.push(Vec::new());
    }


    for &sensor in &input {
        let x_dist = sensor.coords.0.abs_diff(sensor.closest_beacon.0) as i32;
        let y_dist = sensor.coords.1.abs_diff(sensor.closest_beacon.1) as i32;
        for y in max(0, sensor.coords.1 - (x_dist + y_dist))..=min(sensor.coords.1 + (x_dist + y_dist), LIMIT) {
            let diff = (x_dist + y_dist) - y.abs_diff(sensor.coords.1) as i32;
            let range = (sensor.coords.0 - diff as i32, sensor.coords.0 + diff as i32);
            ranges[y as usize].push(range); 
        }
    }


    'outer:
    for (i, line) in ranges.iter().enumerate() {
        for (lower, upper) in line {
            let candidatel = lower - 1;
            if candidatel >= 0 && line.iter().all(|(lower, upper)| !(lower <= &candidatel && upper >= &candidatel)) {
                println!("{},{}", candidatel, i);
                println!("{}", (candidatel * 4000000) + i as i32);
                break 'outer;
            }
            let candidateu = upper + 1;
            if candidateu <= LIMIT && line.iter().all(|(lower, upper)| !(lower <= &candidateu && upper >= &candidateu)) {
                println!("{},{}", candidateu, i);
                println!("{}", (candidateu as i64 * 4000000i64) + i as i64);
                break 'outer;
            }
        }
    }

}
