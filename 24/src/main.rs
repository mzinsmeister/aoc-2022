use std::{fs::read_to_string, collections::{BTreeSet, VecDeque, BinaryHeap, HashSet}, cmp::{max, min, Ordering}};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct PosState {
    pos: (usize, usize),
    end: (usize, usize),
    time: usize,
}

// The priority queue depends on `Ord`.
impl Ord for PosState {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_dist = self.end.0.abs_diff(self.pos.0) + self.end.1.abs_diff(self.pos.1);
        let other_dist = self.end.0.abs_diff(other.pos.0) + self.end.1.abs_diff(other.pos.1);
        other_dist.cmp(&self_dist).then_with(|| other.time.cmp(&self.time))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for PosState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone)]
struct State {
    blizzards_up: VecDeque<u128>,
    blizzards_down: VecDeque<u128>,
    blizzards_left: Vec<u128>,
    blizzards_right: Vec<u128>,
    width: usize,
    height: usize
}

impl State {
    fn time_step(&self) -> State {
        let mut blizzards_left = self.blizzards_left.clone();
        let mut blizzards_right = self.blizzards_right.clone();
        let mut blizzards_up = self.blizzards_up.clone();
        let mut blizzards_down = self.blizzards_down.clone();
        blizzards_down.rotate_right(1);
        blizzards_up.rotate_left(1);
        let mask = 1u128 << (self.width - 1);
        // Least significant bit first encoding = left is right and right is left
        blizzards_right.iter_mut().for_each(|r| {
            *r <<= 1;
            let bit = (*r & mask) >> (self.width - 2);
            *r |= bit;
        });
        blizzards_left.iter_mut().for_each(|r| {
            *r >>= 1;
            let bit = (*r & 1) << (self.width - 2);
            *r |= bit;
        });


        State {
            blizzards_down,
            blizzards_left,
            blizzards_right,
            blizzards_up,
            width: self.width,
            height: self.height
        }
    }

    fn check_position(&self, pos: (usize, usize)) -> bool {
        if pos.1 == 0 || pos.1 == self.height - 1 {
            return true;
        }
        self.blizzards_down[pos.1-1] & 1 << pos.0 == 0
        && self.blizzards_up[pos.1-1] & 1 << pos.0 == 0
        && self.blizzards_left[pos.1-1] & 1 << pos.0 == 0
        && self.blizzards_right[pos.1-1] & 1 << pos.0 == 0
    }
}

fn search(state_cache: &mut Vec<State>, start_pos: (usize, usize), end_pos: (usize, usize), start_time: usize) -> usize {
    let mut done_set: HashSet<PosState> = HashSet::new();
    let mut workset: BinaryHeap<PosState> = BinaryHeap::from([PosState{pos: start_pos, end: end_pos, time: start_time}]);

    let mut current_min = 100_000;
    while !workset.is_empty() {
        let pos_state = workset.pop().unwrap();
        if done_set.contains(&pos_state) {
            continue;
        } else {
            done_set.insert(pos_state);
        }
        let state = if let Some(state) = state_cache.get(pos_state.time + 1) {
            state
        } else {
            let new_state = state_cache[pos_state.time].time_step();
            state_cache.push(new_state);
            &state_cache[pos_state.time + 1]
        };
        assert!(pos_state.pos.0 > 0);
        assert!(pos_state.pos.0 < state.width - 1);
        assert!(pos_state.pos.1 > 0 || pos_state.pos.0 == 1);
        assert!(pos_state.pos.1 < state.height - 1 || pos_state.pos.0 == state.width - 2);
        assert!(state_cache[pos_state.time].check_position(pos_state.pos));
        if pos_state.pos == end_pos {
            current_min = min(current_min, pos_state.time);
        }
        if pos_state.time < current_min {
            //println!("{},{}   {}", pos_state.pos.0, pos_state.pos.1, pos_state.time);
            if pos_state.pos.1 < state.height - 2 || (pos_state.pos.0 == state.width - 2 && pos_state.pos.1 == state.height - 2) {
                // Try moving down
                let check_pos = (pos_state.pos.0, pos_state.pos.1 + 1);
                if state.check_position(check_pos) {
                    workset.push(PosState { pos: check_pos, end: end_pos, time: pos_state.time + 1 })
                }
            }
            if pos_state.pos.1 > 0 && pos_state.pos.0 < state.width - 2 && pos_state.pos.1 < state.height - 1 {
                // Try moving right
                let check_pos = (pos_state.pos.0 + 1, pos_state.pos.1);
                if state.check_position(check_pos) {
                    workset.push(PosState { pos: check_pos, end: end_pos, time: pos_state.time + 1 })
                }
            }
            if pos_state.pos.1 > 0 && pos_state.pos.0 > 1 && pos_state.pos.1 < state.height - 1 {
                // Try moving left
                let check_pos = (pos_state.pos.0 - 1, pos_state.pos.1);
                if state.check_position(check_pos) {
                    workset.push(PosState { pos: check_pos, end: end_pos, time: pos_state.time + 1 })
                }
            }
            if pos_state.pos.1 > 1 || (pos_state.pos.0 == 1 && pos_state.pos.1 == 1) {
                // Try moving up
                let check_pos = (pos_state.pos.0, pos_state.pos.1 - 1);
                if state.check_position(check_pos) {
                    workset.push(PosState { pos: check_pos, end: end_pos, time: pos_state.time + 1 })
                }
            }
            // Try doing nothing
            if state.check_position(pos_state.pos) {
                workset.push(PosState { pos: pos_state.pos, end: end_pos, time: pos_state.time + 1 })
            }
        }
    }

    current_min
}

fn main() {
    let input_str = read_to_string("input.txt").unwrap();
    let input = input_str.lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let width = input[0].len();
    let height = input.len();

    let mut blizzards_left: Vec<u128> = Vec::new();
    let mut blizzards_right: Vec<u128> = Vec::new();
    let mut blizzards_up: VecDeque<u128> = VecDeque::new();
    let mut blizzards_down: VecDeque<u128> = VecDeque::new();
    
    for line in input_str.lines().filter(|l| !l.is_empty()).map(|l| l.chars()).skip(1).take(height-2) {
        let mut left_row = 0u128;
        let mut right_row = 0u128;
        let mut up_row = 0u128;
        let mut down_row = 0u128;
        for (i, char) in line.enumerate() {
            match char {
                '>' => right_row |= 1 << i,
                '<' => left_row |= 1 << i,
                '^' => up_row |= 1 << i,
                'v' => down_row |= 1 << i,
                '.' => (),
                '#' => (),
                _ => panic!()
            }
        }
        blizzards_left.push(left_row);
        blizzards_right.push(right_row);
        blizzards_up.push_back(up_row);
        blizzards_down.push_back(down_row);
    }

    let initial_state = State {
        blizzards_up,
        blizzards_down,
        blizzards_left,
        blizzards_right,
        width,
        height
    };

    let mut state_cache: Vec<State> = Vec::from([initial_state]);

    let result_1 = search(&mut state_cache, (1,0), (width - 2, height - 1), 0);

    println!("{}", result_1);

    let mut result_2 = search(&mut state_cache, (width - 2, height - 1), (1,0), result_1);
    result_2 = search(&mut state_cache, (1,0), (width - 2, height - 1), result_2);
    println!("{}", result_2);
}
