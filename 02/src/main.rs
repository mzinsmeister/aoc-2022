use std::fs::read_to_string;


#[derive(Clone, Copy, Debug)]
enum Shape {
    Rock, Paper, Scissors
}

impl Shape {
    fn parse(input: &str) -> Shape {
        match input {
            "A" => Shape::Rock,
            "X" => Shape::Rock,
            "B" => Shape::Paper,
            "Y" => Shape::Paper,
            "C" => Shape::Scissors,
            "Z" => Shape::Scissors,
            _ => panic!("unexpected shape input"),
        }
    }

    fn get_score(&self) -> u32{
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3
        }
    }

    fn get_score_with(&self, other: &Shape) -> u32 {
        let other_score = other.get_score() as i32;
        let self_score = self.get_score() as i32;
        let total_score = self_score - other_score;
        if total_score == 0 {
            3
        } else if (total_score < 0 && total_score != -2) || total_score == 2 {
            0
        } else {
            6
        }
    }

    fn get_for_result(&self, result: &str) -> Shape {
        let shapes_list = vec![Shape::Rock, Shape::Paper, Shape::Scissors];
        let current_index = (self.get_score() - 1) as i32;
        match result {
            "X" => shapes_list[((current_index - 1).rem_euclid(3)) as usize],
            "Y" => self.clone(),
            "Z" => shapes_list[((current_index + 1).rem_euclid(3)) as usize],
            _ => panic!("Unknown input")
        }
    }
}

struct Round {
    opponent_play: Shape,
    your_play: Shape
}

impl Round {
    fn get_round_score(&self) -> u32 {
        self.your_play.get_score() + self.your_play.get_score_with(&self.opponent_play)
    }
}

fn main() {
    let input_str = read_to_string("input.txt").unwrap();

    let input: Vec<Round> = input_str
    .split("\n")
    .filter(|l| !l.is_empty())
    .map(|l| {
        let mut split_iter = l.split(" ");
        Round{
            opponent_play: Shape::parse(split_iter.next().unwrap()),
            your_play: Shape::parse(split_iter.next().unwrap())
        }
    })
    .collect();

    let result = input.iter().map(|i| i.get_round_score()).sum::<u32>();

    println!("{}", result);

    let input_2: Vec<Round> = input_str
    .split("\n")
    .filter(|l| !l.is_empty())
    .map(|l| {
        let mut split_iter = l.split(" ");
        let opponent_play = Shape::parse(split_iter.next().unwrap());
        let your_play = opponent_play.get_for_result(split_iter.next().unwrap());
        Round{opponent_play, your_play}
    })
    .collect();

    let result_2 = input_2.iter().map(|i| i.get_round_score()).sum::<u32>();

    println!("{}", result_2);

}
