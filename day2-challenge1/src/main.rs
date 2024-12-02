use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("assets/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut safe_count: u32 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let mut line = line.split(" ");

        let mut safe: bool = true;
        let mut prev_number = line.next().unwrap().parse::<u32>().unwrap();
        let mut direction = 0;

        while let Some(next_number) = line.next() {
            let next_number: u32 = next_number.parse().unwrap();
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

            if new_direction.abs_diff(direction) == 2{
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
            safe_count += 1;
        }
    }

    print!("{safe_count}");
}
