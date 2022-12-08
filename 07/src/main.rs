use std::{collections::BTreeMap, fs::read_to_string, cmp::min};


#[derive(Clone, Debug)]
enum FsItem {
    Directory(BTreeMap<String, FsItem>),
    File(usize)
}

impl FsItem {
    fn get_size(&self) -> usize {
        match self {
            FsItem::Directory(d) => d.values().map(|c| c.get_size()).sum(),
            FsItem::File(s) => *s,
        }
    }

    fn pretty_print(&self, name: &str, indent: usize) {
        match self {
            FsItem::Directory(d) => {
                println!("{}{}/  {}", "  ".repeat(indent), name, self.get_size());
                d.iter().for_each(|(name, item)|item.pretty_print(name, indent + 2))
            },
            FsItem::File(s) => println!("{}{}  {}", "  ".repeat(indent), name, self.get_size()),
        }
    }
}

fn navigate_path<'a>(dir: &'a mut BTreeMap<String, FsItem>, path: &[String]) -> &'a mut BTreeMap<String, FsItem> {
    let mut current_dir = dir;
    for item in path {
        if let FsItem::Directory(dir_map) = current_dir.get_mut(item).unwrap() {
            current_dir = dir_map;
        } else {
            panic!("Not a directory");
        }
    }
    current_dir
}

fn rec_1(dir: &BTreeMap<String, FsItem>) -> usize {
    let mut sum = 0;
    for item in dir.values() {
        if let FsItem::Directory(d) = item {
            let current_size = item.get_size();
            if current_size <= 100000 {
                sum += current_size;
            }
            sum += rec_1(d);
        }
    }
    sum
}

fn rec_2(dir: &BTreeMap<String, FsItem>, at_least: usize) -> usize {
    let mut smallest = usize::MAX;
    for item in dir.values() {
        if let FsItem::Directory(d) = item {
            let current_size = item.get_size();
            if current_size >= at_least {
                println!("{}", current_size);
                smallest = min(smallest, current_size);
            }
            smallest = min(rec_2(d, at_least), smallest);
        }
    }
    smallest
}

fn main() {
    let input_str = read_to_string("input.txt").unwrap();

    let mut root_dir = BTreeMap::<String, FsItem>::new();
    let mut current_path = Vec::<String>::new();
    for command_with_result in input_str.split("$").filter(|l| !l.is_empty()) {
        let (command, result) = command_with_result.split_once("\n").unwrap();
        if command.starts_with(" cd") {
            if command.starts_with(" cd ..") {
                current_path.pop();
            } else if command.starts_with(" cd /") {
                current_path.clear();
            } else {
                current_path.push(String::from(&command[4..command.len()]));
            }
        } else { // ls
            let current_dir = &mut navigate_path(&mut root_dir, &current_path);
            result
                .lines()
                .map(|l| l.split_once(" ").unwrap())
                .map(|(size_str, name)| {
                    if size_str.starts_with("dir") {
                        (FsItem::Directory(BTreeMap::new()), name)
                    } else {
                        (FsItem::File(size_str.parse().unwrap()), name)
                    }
                })
                .for_each(|(item, name)| {
                    if current_dir.contains_key(name) {
                        panic!("duplicate entry");
                    } else {
                        assert!(current_dir.insert(name.to_string(), item).is_none());
                    }})
        }
    }

    let result_1 = rec_1(&root_dir);
    let root_item = FsItem::Directory(root_dir.clone());
    let at_least = 30000000 - (70000000 - root_item.get_size());
    let result_2 = rec_2(&root_dir, at_least);

    let root_item = FsItem::Directory(root_dir);
    //println!("{:#?}", root_item);
    root_item.pretty_print("", 0);

    println!("{}", result_1);
    println!("{}", result_2);
}
