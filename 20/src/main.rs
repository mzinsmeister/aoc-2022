use std::{fs::read_to_string, collections::VecDeque};

fn main() {
    let input_str = read_to_string("input.txt").unwrap();

    let mut list = input_str
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| (l.parse().unwrap(), false))
        .collect::<VecDeque<(i32, bool)>>();

    let mut i = 0;
    let len = list.len() as i32;
    for _ in 0..len {
        let (mut num, mut done) = list[i];
        while done {
            i += 1;
            (num, done) = list[i];
        }
        let new_pos = (i as i32 + num - 1).rem_euclid(len - 1) as usize + 1;
        list.remove(i).unwrap();
        list.insert(new_pos, (num, true));
        if new_pos <= i {
            i += 1;
        }
    }

    let (index0, _) = list.iter().enumerate().find(|(_, &e)| e == (0, true)).unwrap();
    let result = list[(index0 + 2000) % list.len()].0 + list[(index0 + 3000) % list.len()].0 + list[(index0 + 1000) % list.len()].0;
    println!("{}", result);
    
    let mut list2 = input_str
        .lines()
        .filter(|l| !l.is_empty())
        .enumerate()
        .map(|(i, l)| (l.parse::<i64>().unwrap(), i))
        .map(|(n, i)| (n * 811589153, i))
        .collect::<VecDeque<(i64, usize)>>();

        let len = list2.len() as i64;
        for _ in 0..10 {
            for i in 0..len as usize {
                let index = list2.iter()
                .enumerate()
                .find(|(_, (_, index))| *index == i)
                .map(|(i, _)| i)
                .unwrap();
                let (num, orig_index) = list2.remove(index).unwrap();
                let new_pos = (index as i64 + num - 1).rem_euclid(len - 1) as usize + 1;
                list2.insert(new_pos, (num, orig_index));
            }
        }

    let (index0, _) = list2.iter().enumerate().find(|(_, &e)| e.0 == 0).unwrap();
    let result = list2[(index0 + 2000) % list2.len()].0 + list2[(index0 + 3000) % list2.len()].0 + list2[(index0 + 1000) % list2.len()].0;
    println!("{}", result);

}
