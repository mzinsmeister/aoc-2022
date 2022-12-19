use std::{fs::read_to_string, collections::HashMap};

use regex::Regex;

#[derive(Clone, Copy)]
struct Blueprint {
    id: u16,
    ore_ore: u16,
    clay_ore: u16,
    obsidian_ore: u16,
    obsidian_clay: u16,
    geode_ore: u16,
    geode_obsidian: u16
}

impl Blueprint {
    fn can_produce_ore_robot(&self, state: &State) -> bool {
        state.ore >= self.ore_ore
    }

    fn can_produce_clay_robot(&self, state: &State) -> bool {
        state.ore >= self.clay_ore
    }

    fn can_produce_obsidian_robot(&self, state: &State) -> bool {
        state.ore >= self.obsidian_ore && state.clay >= self.obsidian_clay
    }

    fn can_produce_geode_robot(&self, state: &State) -> bool {
        state.ore >= self.geode_ore && state.obsidian >= self.geode_obsidian
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct MemState {
    time_left: u16,
    ore_robots: u16,
    clay_robots: u16,
    obsidian_robots: u16,
    ore: u16,
    clay: u16,
    obsidian: u16
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State {
    time_left: u16,
    ore_robots: u16,
    clay_robots: u16,
    obsidian_robots: u16,
    geode_robots: u16,
    ore: u16,
    clay: u16,
    obsidian: u16,
    geode: u16
}

impl State {
    fn to_mem_state(&self) -> MemState {
        MemState { time_left: self.time_left, obsidian_robots: self.obsidian_robots, ore_robots: self.ore_robots, clay_robots: self.clay_robots, ore: self.ore, clay: self.clay, obsidian: self.obsidian }
    }

    fn build_ore_robot(&mut self, blueprint: &Blueprint) {
        self.ore_robots += 1;
        self.ore -= blueprint.ore_ore;
    }

    fn build_clay_robot(&mut self, blueprint: &Blueprint) {
        self.clay_robots += 1;
        self.ore -= blueprint.clay_ore;
    }

    fn build_obsidian_robot(&mut self, blueprint: &Blueprint) {
        self.obsidian_robots += 1;
        self.ore -= blueprint.obsidian_ore;
        self.clay -= blueprint.obsidian_clay;
    }

    fn build_geode_robot(&mut self, blueprint: &Blueprint) {
        self.geode_robots += 1;
        self.ore -= blueprint.geode_ore;
        self.obsidian -= blueprint.geode_obsidian;
    }
    
    fn time_step(&mut self) {
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.geode += self.geode_robots;
        self.time_left -= 1;
    }
}

fn search(blueprint: &Blueprint, state: State, current_max: u16,mem_table: &mut HashMap<MemState, u16>, mut banned: (bool, bool, bool, bool)) -> u16 {
    if state.time_left == 0 {
        return state.geode;
    }
    if current_max > state.geode + state.geode_robots * state.time_left + (state.time_left * state.time_left + state.time_left) / 2 {
        return 0;
    }
    if let Some(mem) = mem_table.get(&state.to_mem_state()) {
        return state.time_left * state.geode_robots + mem + state.geode;
    }
    let mut max_geodes = 0;
    let mut try_all = true;
    if !banned.0 && blueprint.can_produce_geode_robot(&state) {
        let mut new_state = state.clone();
        new_state.time_step();
        new_state.build_geode_robot(blueprint);
        let m = search(blueprint, new_state, max_geodes, mem_table, banned);
        if m > max_geodes {
            max_geodes =m;
        }
        if m < current_max || m < max_geodes {
            banned.0 = true;
        }
        try_all = false;
    }
    if !banned.1 && blueprint.can_produce_obsidian_robot(&state) && state.obsidian_robots < blueprint.geode_obsidian {
        let mut new_state = state.clone();
        new_state.time_step();
        new_state.build_obsidian_robot(blueprint);
        let m = search(blueprint, new_state, max_geodes, mem_table, banned);
        if m > max_geodes {
            max_geodes =m;
        }
        if m < current_max || m < max_geodes {
            banned.1 = true;
        }
    }
    if !banned.2 && state.time_left >= 2 && blueprint.can_produce_clay_robot(&state) && state.clay_robots < blueprint.obsidian_clay{
        let mut new_state = state.clone();
        new_state.time_step();
        new_state.build_clay_robot(blueprint);
        let m = search(blueprint, new_state, max_geodes, mem_table, banned);
        if m > max_geodes {
            max_geodes =m;
        }
        if m < current_max || m < max_geodes {
            banned.2 = true;
        }
    }
    if !banned.3 && state.time_left >= 1 && blueprint.can_produce_ore_robot(&state) && state.ore_robots < blueprint.clay_ore.max(blueprint.obsidian_ore).max(blueprint.geode_ore)  {
        let mut new_state = state.clone();
        new_state.time_step();
        new_state.build_ore_robot(blueprint);
        let m = search(blueprint, new_state, max_geodes, mem_table, banned);
        if m > max_geodes {
            max_geodes =m;
        }
        if m < current_max || m < max_geodes {
            banned.3 = true;
        }
    }
    if try_all {
        let mut new_state = state.clone();
        new_state.time_step();
        let m = search(blueprint, new_state, max_geodes, mem_table, banned);
        if m > max_geodes {
            max_geodes =m;
        }
    }
    if let Some(mem) = mem_table.get(&state.to_mem_state()) {
        let m = max_geodes - state.geode_robots * state.time_left - state.geode;
        if *mem <  m{
            mem_table.insert(state.to_mem_state(), m);
        }
    } else {
        mem_table.insert(state.to_mem_state(), max_geodes - state.geode_robots * state.time_left - state.geode);
    }
    max_geodes
}

fn main() {
    let input_str = read_to_string("input.txt").unwrap();

    let re = Regex::new(r"Blueprint (?P<b_id>\d+): Each ore robot costs (?P<ore_ore>\d+) ore. Each clay robot costs (?P<clay_ore>\d+) ore. Each obsidian robot costs (?P<obsidian_ore>\d+) ore and (?P<obsidian_clay>\d+) clay. Each geode robot costs (?P<geode_ore>\d+) ore and (?P<geode_obsidian>\d+) obsidian.").unwrap();

    let input = input_str.lines()
        .filter(|l| !l.is_empty())
        .map(|l| re.captures(l).unwrap())
        .map(|c| Blueprint {
            id: c.name("b_id").unwrap().as_str().parse().unwrap(),
            ore_ore: c.name("ore_ore").unwrap().as_str().parse().unwrap(),
            clay_ore: c.name("clay_ore").unwrap().as_str().parse().unwrap(),
            obsidian_ore: c.name("obsidian_ore").unwrap().as_str().parse().unwrap(),
            obsidian_clay: c.name("obsidian_clay").unwrap().as_str().parse().unwrap(),
            geode_ore: c.name("geode_ore").unwrap().as_str().parse().unwrap(),
            geode_obsidian: c.name("geode_obsidian").unwrap().as_str().parse().unwrap()
        })
        .collect::<Vec<Blueprint>>();

    let mut max = 0;
    let mut max_blueprint: Option<Blueprint> = Option::None;
    let mut result = 0;
    for blueprint in input.iter() {
        let state = State { 
            time_left: 24, 
            ore_robots: 1, 
            clay_robots: 0, 
            obsidian_robots: 0, 
            geode_robots: 0, 
            ore: 0,
            clay: 0, 
            obsidian: 0, 
            geode: 0 
        };
        let mut mem_table = HashMap::new();
        let bmax = search(&blueprint, state, 0, &mut mem_table, (false, false, false, false));
        if bmax > max {
            max = bmax;
            max_blueprint = Some(*blueprint);
        }
        result += blueprint.id * bmax;
    }

    println!("{}, {}, {}", max_blueprint.map(|m| m.id).unwrap_or_default(), max, result);

    let mut max = 0;
    let mut max_blueprint: Option<Blueprint> = Option::None;
    let mut result = 1;
    for blueprint in input.iter().take(3) {
        let state = State { 
            time_left: 32, 
            ore_robots: 1, 
            clay_robots: 0, 
            obsidian_robots: 0, 
            geode_robots: 0, 
            ore: 0,
            clay: 0, 
            obsidian: 0, 
            geode: 0 
        };
        let mut mem_table = HashMap::new();
        let bmax = search(&blueprint, state, 0, &mut mem_table, (false, false, false, false));
        if bmax >= max {
            max = bmax;
            max_blueprint = Some(*blueprint);
        }
        result *= bmax;
    }

    println!("{}, {}, {}", max_blueprint.unwrap().id, max, result);
}
