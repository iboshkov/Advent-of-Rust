use std::{fs::File, io::Read, collections::HashSet};

use itertools::Itertools;

fn parseRange(range: &str) -> std::ops::Range<i32>{
    let split = range.split("-").collect::<Vec<&str>>();
    let start = split[0].parse::<i32>().unwrap();
    let end = split[1].parse::<i32>().unwrap();
    return start..end;
}

fn fullyContainsOtherRange(range1: &std::ops::Range<i32>, range2: &std::ops::Range<i32>) -> bool {
    return range1.start <= range2.start && range1.end >= range2.end;
}

fn rangeOverlaps(range1: &std::ops::Range<i32>, range2: &std::ops::Range<i32>) -> bool {
    return range1.start <= range2.start && range1.end >= range2.start || range1.start <= range2.end && range1.end >= range2.end;
}

fn pt1(lines: Vec<&str>) -> i32 {
    let mut sum = 0;
    let mut sumOverlap = 0;
    for (index, line) in lines.iter().enumerate() {
        let split = line.split(",").collect::<Vec<&str>>();
        let range1 = parseRange(split[0]);
        let range2 = parseRange(split[1]);

        let contains = fullyContainsOtherRange(&range1, &range2) || fullyContainsOtherRange(&range2, &range1);
        let overlaps = rangeOverlaps(&range1, &range2) || rangeOverlaps(&range2, &range1);
        if contains {
            sum += 1;
        }
        if overlaps {
            sumOverlap += 1;
        }

        // println!("{}: {}-{} or {}-{}", contains, range1.start, range1.end, range2.start, range2.end);
        println!("{}: {}-{} or {}-{}", overlaps, range1.start, range1.end, range2.start, range2.end);
    }
    println!("Sum: {}", sum);
    println!("Overlaps: {}", sumOverlap);
    return sum;
}


fn main() {
    let mut file = File::open("input.txt").expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let lines: Vec<&str> = contents.split('\n').collect();
    pt1(lines);
}