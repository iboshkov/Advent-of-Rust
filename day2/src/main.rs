use std::{fs::File, io::Read};

fn getOutcome(me: &String, opponent: &String) -> String {
    if me == "X" {
        if opponent == "A" {
            return String::from("draw");
        } else if opponent == "B" {
            return String::from("lose");
        } else if opponent == "C" {
            return String::from("win");
        }
    }
    if me == "Y" {
        if opponent == "A" {
            return String::from("win");
        } else if opponent == "B" {
            return String::from("draw");
        } else if opponent == "C" {
            return String::from("lose");
        }
    }
    if me == "Z" {
        if opponent == "A" {
            return String::from("lose");
        } else if opponent == "B" {
            return String::from("win");
        } else if opponent == "C" {
            return String::from("draw");
        }
    }
    return String::from("unknown");
}


fn getHandForCondition(opponent: &String, condition: &String) -> String {
    if opponent == "A" { // ROCK
        if condition == "Z" {
            return String::from("B");
        } else if condition == "X" {
            return String::from("C");
        } else if condition == "Y" {
            return String::from("A");
        }
    }
    if opponent == "B" {
        if condition == "Z" {
            return String::from("C");
        } else if condition == "X" {
            return String::from("A");
        } else if condition == "Y" {
            return String::from("B");
        }
    }
    if opponent == "C" {
        if condition == "Z" {
            return String::from("A");
        } else if condition == "X" {
            return String::from("B");
        } else if condition == "Y" {
            return String::from("C");
        }
    }
    return String::from("unknown");
}

fn getPtsPt1(outcome: &String, myMove: &String) -> i32 {
    let mut pts = 0;

    if outcome == "win" {
        pts += 6;
    } else if outcome == "draw" {
        pts += 3;
    }

    if myMove == "X" {
        pts += 1;
    } else if myMove == "Y" {
        pts += 2;
    } else if myMove == "Z" {
        pts += 3;
    }
    return pts;
}

fn getPtsPt2(opponent: &String, expectedOutcome: &String) -> i32 {
    let hand = getHandForCondition(opponent, expectedOutcome);
    let mut pts = 0;

    if expectedOutcome == "Z" {
        pts += 6;
    } else if expectedOutcome == "Y" {
        pts += 3;
    }

    if hand == "A" {
        pts += 1;
    } else if hand == "B" {
        pts += 2;
    } else if hand == "C" {
        pts += 3;
    }
    /* 
    X - LOSE
    Y - DRAW
    Z - WIN

    */
    println!("{} - {} - {} - {}", opponent, expectedOutcome, hand, pts);
    return pts;
}

fn main() {
    let mut file = File::open("input.txt").expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let lines: Vec<&str> = contents.split('\n').collect();

    let mut sum = 0;
    let mut sumPt2 = 0;

    for (index, line) in lines.iter().enumerate() {
        let v: Vec<String> = line.split(' ').map(|s| s.to_string()).collect();
        if let [opponent, me] = &v[..] {
            let res = getOutcome(&me.to_string(), &opponent.to_string());

            sum += getPtsPt1(&res.to_string(), &me.to_string());
            sumPt2 += getPtsPt2(&opponent.to_string(), &me.to_string());
            
       }
    }
    println!("{}", sum);
    println!("{}", sumPt2);

}