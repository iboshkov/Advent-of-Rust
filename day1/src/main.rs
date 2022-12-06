use std::{fs::File, io::Read};

fn main() {
    let mut file = File::open("input.txt").expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let lines: Vec<&str> = contents.split('\n').collect();

    let mut vec: Vec<i32> = Vec::new();

    let mut max = 0;
    let mut sum = 0;
    for (_, line) in lines.iter().enumerate() {
        if line.is_empty() {
            if (sum > max) {
                max = sum;
            }
            sum = 0;
            continue;
        }
        let parsed = line.parse::<i32>().unwrap();
        sum += parsed;
        vec.push(sum);
    }

    vec.sort();
    vec.reverse();

    // sum vector with reduce
    let maxSum = vec[..3].iter().copied().reduce(|acc, x| acc + x).unwrap();

    println!("{}", vec.get(0).unwrap());
    println!("{}", vec.get(1).unwrap());
    println!("{}", vec.get(2).unwrap());
    println!("max: {}", maxSum);
}
