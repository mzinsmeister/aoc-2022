use std::fs::read_to_string;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Item {
    List(Vec<Item>),
    Number(u32)
}

impl Item {
    fn parse_with_rest(input: &str) -> (Item, &str) {
        if input.chars().next().unwrap().is_numeric() {
            let next_delim = input.find(&[',', ']'][..]).unwrap();
            let num = &input[0..next_delim];
            let item = Item::Number(num.parse().unwrap());
            (item, &input[next_delim..input.len()])
        } else {
            if input.starts_with("[]") {
                (Item::List(vec![]), &input[2..input.len()])
            } else {
                let mut rest = input;
                let mut list: Vec<Item> = Vec::new();
                while !rest.starts_with("]") {
                    let (item, new_rest) = Item::parse_with_rest(&rest[1..rest.len()]);
                    rest = new_rest;
                    list.push(item);
                }
                (Item::List(list), &rest[1..rest.len()])
            }
        }
    }
    
    fn print(&self) {
        match self {
            Item::List(l) => {
                print!("[");
                l.get(0).map(|i| i.print());
                for i in l.iter().skip(1) {
                    print!(",");
                    i.print();
                }
                print!("]");
            },
            Item::Number(n) => print!("{}", n),
        }
    }
}

fn compare_items(i1: &Item, i2: &Item) -> Option<bool> {
    match i1 {
        Item::List(l1) => match i2 {
            Item::List(l2) => {
                for (i1, i2) in l1.iter().zip(l2.iter()) {
                    let result = compare_items(i1, i2);
                    if result.is_some() {
                        return result;
                    }
                }
                if l1.len() < l2.len() {
                    Some(true)
                } else if l1.len() > l2.len() {
                    Some(false)
                } else {
                    None
                }
            },
            Item::Number(_) => compare_items(i1, &Item::List(vec![i2.clone()])),
        },
        Item::Number(n1) => match i2 {
            Item::List(_) => compare_items(&Item::List(vec![i1.clone()]), i2),
            Item::Number(n2) => {
                if n1 < n2 {
                    Some(true)
                } else if n1 > n2 {
                    Some(false)
                } else {
                    None
                }
            },
        },
    }
}

fn main() {
    let input_str = read_to_string("input.txt").unwrap();

    let input = input_str.split("\n\n")
        .filter(|b| !b.is_empty())
        .map(|l| l.split_once("\n").unwrap())
        .map(|l| (Item::parse_with_rest(l.0).0, Item::parse_with_rest(l.1).0))
        .collect::<Vec<(Item, Item)>>();

    let result1 = input.iter().enumerate()
        .filter(|(i, (i1, i2))| compare_items(i1, i2).unwrap())
        .map(|(i, _)| i+1)
        .sum::<usize>();

    println!("{}", result1);

    let mut input2 = input.iter()
        .fold(Vec::with_capacity(input.len() * 2),
          |mut acc, p| { acc.extend([p.0.clone(), p.1.clone()]); acc });

    let i1 = Item::List(vec![Item::Number(2)]);
    input2.push(i1.clone());
    let i2 = Item::List(vec![Item::Number(6)]);
    input2.push(i2.clone());

    input2.sort_by(|a,b| {
        let result = compare_items(a, b);
        match result {
            Some(r) => match r {
                true => std::cmp::Ordering::Less,
                false => std::cmp::Ordering::Greater,
            },
            None => std::cmp::Ordering::Equal,
        }
    });

    let index1 = input2.iter().enumerate().find(|f| {
        if let Item::List(l) = f.1 {
            if l.len() == 1 {
                l[0] == Item::Number(2)
            } else {
                false
            }
        } else {
            false
        }
    }).unwrap().0 + 1;
    let index2 = input2.iter().enumerate().find(|f| {
        if let Item::List(l) = f.1 {
            if l.len() == 1 {
                l[0] == Item::Number(6)
            } else {
                false
            }
        } else {
            false
        }
    }).unwrap().0 + 1;
    let result2 = index1 * index2;

    println!("{}", result2);

}
