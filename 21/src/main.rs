use core::panic;
use std::{fs::read_to_string, collections::BTreeMap};

#[derive(Clone, Debug, PartialEq, Eq)]
enum Job {
    Number(i64),
    Operation(MathOperation, String, String)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MathOperation {
    Add,
    Sub,
    Mul,
    Div,
    Eq
}

impl MathOperation {
    fn apply(&self, o1: i64, o2: i64) -> i64 {
        match self {
            MathOperation::Add => o1 + o2,
            MathOperation::Sub => o1 - o2,
            MathOperation::Mul => o1 * o2,
            MathOperation::Div => o1 / o2,
            MathOperation::Eq => if o1 == o2 { 1 } else { 0 }
        }
    }
}

enum OpTree {
    Add(Box<OpTree>, Box<OpTree>),
    Sub(Box<OpTree>, Box<OpTree>),
    Mul(Box<OpTree>, Box<OpTree>),
    Div(Box<OpTree>, Box<OpTree>),
    Eq(Box<OpTree>, Box<OpTree>),
    Var,
    Literal(i64)
}

impl OpTree {
    fn eval(&self) -> Option<i64> {
        match self {
            OpTree::Add(o1, o2) => Some(o1.eval()? + o2.eval()?),
            OpTree::Sub(o1, o2) => Some(o1.eval()? - o2.eval()?),
            OpTree::Mul(o1, o2) => Some(o1.eval()? * o2.eval()?),
            OpTree::Div(o1, o2) => Some(o1.eval()? / o2.eval()?),
            OpTree::Eq(o1, o2) => Some(if o1.eval()? == o2.eval()? {1} else {0}),
            OpTree::Var => None,
            OpTree::Literal(l) => Some(*l),
        }
    }

    fn construct(input: &BTreeMap<String, Job>, key: &str) -> OpTree {
        if key == "humn" {
            return OpTree::Var;
        }
        let job = &input[key];
        match job {
            Job::Number(l) => OpTree::Literal(*l),
            Job::Operation(op, o1, o2)=> {
                let o1_tree = Box::new(Self::construct(input, o1));
                let o2_tree = Box::new(Self::construct(input, o2));
                match op {
                    MathOperation::Add => OpTree::Add(o1_tree, o2_tree),
                    MathOperation::Sub => OpTree::Sub(o1_tree, o2_tree),
                    MathOperation::Mul => OpTree::Mul(o1_tree, o2_tree),
                    MathOperation::Div => OpTree::Div(o1_tree, o2_tree),
                    MathOperation::Eq => OpTree::Eq(o1_tree, o2_tree),
                }
            },
        }
    }

    fn find_var(&self, res: i64) -> i64 {
        match self {
            OpTree::Add(o1, o2) => {
                if let Some(re) = o1.eval() {
                    o2.find_var(res - re)
                } else {
                    o1.find_var(res - o2.eval().unwrap())
                }
            },
            OpTree::Sub(o1, o2) => {
                if let Some(re) = o1.eval() {
                    o2.find_var(re - res)
                } else {
                    o1.find_var(res + o2.eval().unwrap())
                }
            },
            OpTree::Mul(o1, o2) => {
                if let Some(re) = o1.eval() {
                    o2.find_var(res / re)
                } else {
                    o1.find_var(res / o2.eval().unwrap())
                }
            },
            OpTree::Div(o1, o2) => {
                if let Some(re) = o1.eval() {
                    o2.find_var(re / res)
                } else {
                    o1.find_var(o2.eval().unwrap() * res)
                }
            },
            OpTree::Eq(o1, o2) => {
                if let Some(re) = o1.eval() {
                    o2.find_var(re)
                } else {
                    o1.find_var(o2.eval().unwrap())
                }
            },
            OpTree::Var => res,
            OpTree::Literal(_) => panic!(),
        }
    }
}

fn main() {
    let input_str = read_to_string("input.txt").unwrap();
    
    let mut input = input_str.lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let name = l[0..4].to_string();
            let job = if let Some((op1, rest)) = l[6..l.len()].split_once(" ") {
                let (operator, op2) = rest.split_once(" ").unwrap();
                Job::Operation(match operator {
                    "+" => MathOperation::Add,
                    "-" => MathOperation::Sub,
                    "*" => MathOperation::Mul,
                    "/" => MathOperation::Div,
                    _ => panic!()
                }, op1.to_string(), op2.to_string())
            } else {
                Job::Number(l[6..l.len()].parse().unwrap())
            };
            (name, job)
        })
        .collect::<BTreeMap<String, Job>>();
    let mut input2 = input.clone();

    let mut has_ops = true;
    while has_ops {
        has_ops = false;
        for key in input.clone().keys() {
            if let Job::Operation(op, o1, o2) = &input[key] {
                if let Job::Number(on1) = &input[o1] {
                    if let Job::Number(on2) = &input[o2] {
                        let op_result = op.apply(*on1, *on2);
                        input.insert(key.to_owned(), Job::Number(op_result));
                    } else {
                        has_ops = true;
                    }
                } else {
                    has_ops = true;
                }
            }
        }
    }

    println!("{:#?}", input["root"]); 
    
    
    if let Job::Operation(_, o1, o2) = input2["root"].clone() {
        input2.insert("root".to_string(), Job::Operation(MathOperation::Eq, o1, o2));
    }

    let tree = OpTree::construct(&input2, "root");

    println!("{}", tree.find_var(1));
}
