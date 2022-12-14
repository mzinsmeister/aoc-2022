use std::{fs::read_to_string, cmp::max};

fn main() {
    let input_str = read_to_string("input.txt").unwrap();
    
    let input = input_str
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>();

    /*
     A more efficient solution would be to go through each row and each colum and always remember
     at what position you saw a tree with a particular height for each of the 9 different heights
     which would then allow you to directly calculate the visibility/viewing distance from a 
     for each new tree you see in the direction you came from. This would allow you to only
     look at a number of trees linear to the number of trees in the input (4 * n).

     ... but Rust is fast and while it's maybe not the most beautiful solution, it's the most
     straightforward one so i did this for today.
    */

    let mut counter = 0;
    let mut max_scenic_score = 0;
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            let height = input[y][x];
            let mut visible_top = true;
            let mut range_top = 0;
            let mut visible_bottom = true;
            let mut range_bottom = 0;
            let mut visible_left = true;
            let mut range_left = 0;
            let mut visible_right = true;
            let mut range_right = 0;
            for y_test in (0..y).rev() {
                range_top += 1;
                if input[y_test][x] >= height {
                    visible_top = false;
                    break;
                }
            }
            for y_test in (y+1)..input.len() {
                range_bottom += 1;
                if input[y_test][x] >= height {
                    visible_bottom = false;
                    break;
                }
            }
            for x_test in (0..x).rev() {
                range_left += 1;
                if input[y][x_test] >= height {
                    visible_left = false;
                    break;
                }
            }
            for x_test in (x+1)..input[0].len() {
                range_right += 1;
                if input[y][x_test] >= height {
                    visible_right = false;
                    break;
                }
            }
            if visible_bottom || visible_left || visible_right || visible_top {
                counter += 1;
            }
            let scenic_score = range_bottom * range_left * range_top * range_right;
            max_scenic_score = max(scenic_score, max_scenic_score);
        }
    }

    println!("{}", counter);
    println!("{}", max_scenic_score);
}
