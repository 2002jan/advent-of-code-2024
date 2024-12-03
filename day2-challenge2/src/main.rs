use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("assets/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut safe_count: u32 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let mut line = line.split(" ");
        let numbers: Vec<_> = line.map(|x| x.parse::<u32>().unwrap()).collect();

        let mut one_safe: bool = false;


        for i in 0..numbers.len() + 1 {
            let start = if i != 0 { 0 } else { 1 };

            let mut safe = true;
            let mut prev_number = numbers[start];
            let mut direction = 0;

            for j in start+1..numbers.len() {
                if j == i {
                    continue;
                }

                let next_number: u32 = numbers[j];
                if next_number == prev_number {
                    safe = false;
                    break;
                }

                let new_direction: i32;

                if next_number > prev_number {
                    new_direction = 1;
                } else {
                    new_direction = -1;
                }

                if new_direction.abs_diff(direction) == 2 {
                    safe = false;
                    break;
                }

                if next_number.abs_diff(prev_number) > 3 {
                    safe = false;
                    break;
                }

                direction = new_direction;
                prev_number = next_number;
            }

            if safe {
                one_safe = true;
                break;
            }
        }


        if one_safe {
            safe_count += 1;
        }
    }

    print!("{safe_count}");
}
