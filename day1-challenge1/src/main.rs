use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("assets/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut nums1: BinaryHeap<u32> = BinaryHeap::new();
    let mut nums2: BinaryHeap<u32> = BinaryHeap::new();

    for line in reader.lines(){
        let line = line.unwrap();
        let mut line = line.split("   ");
        let n1 = line.next().unwrap();
        let n2 = line.next().unwrap();

        nums1.push(n1.parse::<u32>().unwrap());
        nums2.push(n2.parse::<u32>().unwrap());
    }

    let mut sum = 0;

    while let Some(num1) = nums1.pop(){
        if let Some(num2) = nums2.pop(){
            sum += num1.abs_diff(num2);
        }
    }

    println!("{sum}");
}
