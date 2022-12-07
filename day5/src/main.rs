use std::{fs::File, io::Read, collections::HashMap};

fn parse_line(line: &str) -> Vec<char> {
    let res = format!("{} ", line).replace("    ", " [ ] ");
    let mut items = Vec::new();
    
    let mut push_next = false;
    for (_, char) in res.chars().enumerate() {
        if push_next {
            items.push(char);
            push_next = false;
        }
        if char == '[' {
            push_next = true;
        }
    }

    return items;
}

fn parse_stacks(nums: &Vec<i32>, parsed_lines: &Vec<Vec<char>>) -> HashMap<i32, Vec<char>> {
    let mut map: HashMap<i32, Vec<char>> = HashMap::new();
    for (_, num) in nums.iter().enumerate() {
        let mut vec = Vec::new();
        map.insert(*num, vec);
    }

    println!("{:?}", parsed_lines);
    for (_, line) in parsed_lines.iter().enumerate() {
        for (i, char) in line.iter().enumerate() {
            let mut vec = map.get(&(i as i32 + 1)).unwrap().to_vec();
            if *char != ' ' {
                vec.insert(0, *char);
                map.insert(i as i32 + 1, vec);
            }
        }
    }

    return map;
}

fn process_move(map: &mut HashMap<i32, Vec<char>>, num: i32, from: i32, to: i32, preserveOrder: bool) {
    let mut vecFrom = map.get(&(from)).unwrap().to_vec();
    let mut vecTo = map.get(&(to)).unwrap().to_vec();
    let lastIndex = if vecTo.len() > 0 { vecTo.len() } else { 0 };
    println!("pre {}/{lastIndex}: {:?} -> {:?}", num, vecFrom, vecTo);
    for _ in 0..num {
        if preserveOrder {
            vecTo.insert(lastIndex, vecFrom.pop().unwrap());
        } else {
            vecTo.push(vecFrom.pop().unwrap());
        }
    }
    println!("pos {}: {:?} -> {:?}", num, vecFrom, vecTo);

    map.insert(from, vecFrom);
    map.insert(to, vecTo);
    println!("{:?}", map);
}

fn parse_move(line: &str) -> Vec<i32> {
    return line
        .replace("move", "")
        .replace("from", "")
        .replace("to", "")
        .split(' ')
        .filter(|x| x.trim().len() > 0)
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
}

fn pt1(lines: Vec<&str>) {
    let mut parsedLines: Vec<Vec<char>> = lines.iter()
        .filter(|x| x.contains("["))
        .map(|x| parse_line(x))
        .collect();

    let mut parsedMoves = lines.iter()
        .filter(|x| x.contains("move"))
        .map(|x| parse_move(x))
        .collect::<Vec<_>>();

    let nums = lines.iter()
        .find(|x| !x.contains('[') && x.len() > 0)
        .unwrap()
        .split(' ')
        .filter(|x| x.trim().len() > 0)
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut map = parse_stacks(&nums, &parsedLines);

    for (_, m) in parsedMoves.iter().enumerate() {
        process_move(&mut map, m[0], m[1], m[2], true);
    }

    for (_, num) in nums.iter().enumerate() {
        match map.get(num).unwrap().last() {
            Some(x) => print!("{}", x),
            None => print!(""),
        }
    }
}


fn main() {
    let mut file = File::open("input.txt").expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let lines: Vec<&str> = contents.split('\n').collect();
    pt1(lines);
}