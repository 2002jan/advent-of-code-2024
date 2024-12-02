use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("assets/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut number_counts: HashMap<u32, u32> = HashMap::new();
    let mut numbers: Vec<u32> = Vec::new();

    for line in reader.lines(){
        let line = line.unwrap();
        let mut line = line.split("   ");
        let n1 = line.next().unwrap();
        let n2 = line.next().unwrap();

        number_counts.insert(n1.parse().unwrap(), 0);
        numbers.push(n2.parse().unwrap());
    }

    while let Some(n2) = numbers.pop(){
        if let Some(n1_count) = number_counts.get(&n2) {
            number_counts.insert(n2, n1_count+1);
        }
    }

    let mut sum = 0;

    for (n1, n2) in number_counts.iter() {
        sum += n1 * n2;
    }

    println!("{sum}");
}
