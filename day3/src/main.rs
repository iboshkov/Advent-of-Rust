use std::{fs::File, io::Read, collections::HashSet};

use itertools::Itertools;

fn pt1(lines: Vec<&str>) -> i32 {
    let mut sum = 0;
    for (index, line) in lines.iter().enumerate() {
        let half = line.len() / 2;
        let first = &line[0..half];
        let second = &line[half..line.len()];
        let common = first
            .chars()
            .filter(|x| second.contains(*x))
            .map(|x| if x.is_lowercase() 
                 { (x as i32) - 'a' as i32 + 1  }
                else { 27 + (x as i32) - 'A' as i32 }
            )
            .collect::<HashSet<i32>>();

        let commonstr = common
            .iter()
            .map(|x| x.to_string() + ", ")
            .collect::<String>();

        sum += common.iter().sum::<i32>();
    }
    return sum;
}

fn pt2(lines: Vec<&str>) -> i32 {
    let mut sum = 0;
    // chunk lines into groups of 3
    let chunks = lines.chunks(3).into_iter().collect::<Vec<_>>();

    let mut sum = 0;
    for (index, line) in chunks.iter().enumerate() {
        let first = line[0];
        let second = line[1];
        let third = line[2];
        let common = first
            .chars()
            .filter(|x| first.contains(*x))
            .filter(|x| second.contains(*x))
            .filter(|x| third.contains(*x))
            .map(|x| if x.is_lowercase() 
                    { (x as i32) - 'a' as i32 + 1  }
                else { 27 + (x as i32) - 'A' as i32 }
            )
            .collect::<HashSet<i32>>();
        sum += common.iter().sum::<i32>();
        println!("{}: {:?}", index, common);
    }

    println!("sum: {}", sum);
    // for (index, line) in lines.iter().chunks(3).map(|x| x.collect::<Vec<Vec<&str>>>()).enumerate() {
    //     let first = line[0];
    //     let second = line[1];
    //     let third = line[2];

    //     let common = first
    //         .chars()
    //         .filter(|x| second.contains(*x))
    //         .filter(|x| second.contains(*x))
    //         .filter(|x| second.contains(*x))
    //         .map(|x| if x.is_lowercase() 
    //              { (x as i32) - 'a' as i32 + 1  }
    //             else { 27 + (x as i32) - 'A' as i32 }
    //         )
    //         .collect::<HashSet<i32>>();

    //     let commonstr = common
    //         .iter()
    //         .map(|x| x.to_string() + ", ")
    //         .collect::<String>();

    //     sum += common.iter().sum::<i32>();
    // }
    return sum;
}

fn main() {
    let mut file = File::open("input.txt").expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let lines: Vec<&str> = contents.split('\n').collect();
    // println!("{}", pt1(lines));
    pt2(lines);
}