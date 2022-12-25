use std::fs::read_to_string;



fn convert_snafu_to_decimal(snafu: &str) -> i64 {
    let mut num = 0;
    for (i,c) in snafu.chars().rev().enumerate() {
        let factor = 5i64.pow(i as u32);
        match c {
            '0' => (),
            '1' => num += factor,
            '2' => num += 2*factor,
            '-' => num -= factor,
            '=' => num -= 2* factor,
            _ => panic!()
        }
    }
    num
}

fn convert_decimal_to_snafu(mut num: i64) -> String {
    let mut snafu = String::new();
    while num != 0 {
        let mut factor = num % 5;
        num /= 5;
        if factor >= 3 {
            num += 1;
        }
        let snafu_char = match factor {
            3 => "=".to_string(),
            4 => "-".to_string(),
            _ => factor.to_string()
        };
        snafu = snafu_char + &snafu;
    }
    snafu
}

fn main() {
    let input_str = read_to_string("input.txt").unwrap();

    let decimals_sum = input_str.lines()
        .filter(|l| !l.is_empty())
        .map(convert_snafu_to_decimal)
        .sum::<i64>();

    let result = convert_decimal_to_snafu(decimals_sum);

    println!("{}", result);
}
