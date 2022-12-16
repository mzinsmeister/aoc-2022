use std::{fs::read_to_string, collections::{BTreeMap, HashMap}, cmp::max};

#[derive(Clone)]
struct Room {
    num: usize,
    flow: usize,
    id: u16,
    connections: Vec<u16>
}

struct Search {
    num_valves: usize,
    rooms: BTreeMap<u16, Room>,
    bounds: BTreeMap<((u16, u16), u16), usize>,
    memoization_table: HashMap<((u16,u16), u16, u64), usize>
}

impl Search {
    fn search(&mut self, current_flow: usize, current_pos: u16, current_time: u16, used_valves: u64, last: u16) -> usize {
        if current_time <= 1 {
            return current_flow;
        }
        let mut max_score = 0;
        let room = self.rooms[&current_pos].clone();
        if room.flow != 0 && (used_valves & 1 << room.num) == 0 {
            let new_flow = current_flow + (current_time as usize) * room.flow;
            let new_used_values = used_valves | (1 << room.num);
            max_score = max(max_score, self.search(new_flow, current_pos, current_time - 1, new_used_values, current_pos));
        }
        for &conn in &room.connections {
            if conn != last {
                max_score = max(max_score, self.search(current_flow, conn, current_time - 1, used_valves, current_pos));
            }
        }
        max_score
    }

    fn search2(&mut self, current_flow: usize, current_pos: (u16, u16), current_time: u16, used_valves: u64, last: (u16, u16)) -> usize {
        if let Some(v) = self.memoization_table.get(&(current_pos, current_time, used_valves)) {
            return *v;
        }
        if let Some(b) = self.bounds.get(&(current_pos, current_time)) {
            if *b >= current_flow {
                return 0;
            }
        } else {
            for t in (0..=current_time).rev() {
                if let Some(b) = self.bounds.get(&(current_pos, t)) {
                    if *b < current_flow {
                        self.bounds.insert((current_pos, t), current_flow);
                    } else {
                        break;
                    }
                } else {
                    self.bounds.insert((current_pos, t), current_flow);
                }
            }
        }
        if current_time <= 1 || used_valves.count_ones() as usize == self.num_valves {
            return current_flow;
        }
        let mut max_score = 0;
        let room0 = self.rooms[&current_pos.0].clone();
        let room1 = self.rooms[&current_pos.1].clone();
        if room0.flow != 0 && (used_valves & 1 << room0.num) == 0 {
            let new_flow = current_flow + (current_time as usize) * room0.flow;
            let new_used_values = used_valves | (1 << room0.num);
            if room1.flow != 0 && (new_used_values & 1 << room1.num) == 0 {
                let new_flow = new_flow + (current_time as usize) * room1.flow;
                let new_used_values = new_used_values | (1 << room1.num);
                max_score = max(max_score, self.search2(new_flow, current_pos, current_time - 1, new_used_values, current_pos));
            }
            for &conn in &room1.connections {
                if conn != last.1 {
                    max_score = max(max_score, self.search2(new_flow, (current_pos.0, conn), current_time - 1, new_used_values, current_pos));
                }
            }
        }
        for &conn0 in &room0.connections {
            if conn0 != last.0 {
                if room1.flow != 0 && (used_valves & 1 << room1.num) == 0 {
                    let new_flow = current_flow + (current_time as usize) * room1.flow;
                    let new_used_values = used_valves | (1 << room1.num);
                    max_score = max(max_score, self.search2(new_flow, (conn0, current_pos.1), current_time - 1, new_used_values, current_pos));
                }
                for &conn in &room1.connections {
                    if conn != last.1 {
                        max_score = max(max_score, self.search2(current_flow, (conn0, conn), current_time - 1, used_valves, current_pos));
                    }
                }
            }
        }
        self.memoization_table.insert((current_pos, current_time, used_valves), max_score);
        max_score
    }
}

fn main() {
    let input_str = read_to_string("input.txt").unwrap();

    let rooms = input_str.replace("tunnel leads to valve", "tunnels lead to valves").lines()
        .filter(|l| !l.is_empty())
        .enumerate()
        .map(|(i,l)| {
            let (name, rest) = l[6..l.len()].split_once(" has flow rate=").unwrap();
            let (flow_str, rest) = rest.split_once("; tunnels lead to valves ").unwrap();
            let connections = rest.split(", ").map(|c| u16::from_be_bytes(c.chars().map(|c| c as u8).collect::<Vec<u8>>().try_into().unwrap())).collect::<Vec<u16>>();
            let flow: usize = flow_str.parse().unwrap();
            let id = u16::from_be_bytes(name.chars().map(|c| c as u8).collect::<Vec<u8>>().try_into().unwrap());
            (id, Room {
                num: i,
                id,
                flow,
                connections
            })
        })
        .collect::<BTreeMap<u16, Room>>();

    let num_valves = rooms.iter().filter(|r| r.1.flow > 0).count();

    let bounds = BTreeMap::new();
    let memoization_table = HashMap::new();

    let mut search = Search {rooms, bounds, num_valves, memoization_table};

    let result = search.search(0, u16::from_be_bytes(['A' as u8,'A' as u8]), 29, 0, 0);

    println!("{}", result);

    let result2 = search.search2(0, (u16::from_be_bytes(['A' as u8,'A' as u8]), u16::from_be_bytes(['A' as u8,'A' as u8])), 25, 0, (0,0));

    println!("{}", result2);
}
