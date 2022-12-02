use std::{env, fs};

fn score_for_when_rock(r2: &str) -> usize {
    let r2_rock = r2.cmp("A");
    let r2_paper = r2.cmp("B");
    let r2_scissors = r2.cmp("C");

    match r2_rock {
        std::cmp::Ordering::Equal => 1+3,
        _ => match r2_paper {
            std::cmp::Ordering::Equal => 2+6,
            _ => match r2_scissors {
                std::cmp::Ordering::Equal => 3+0,
                _ => { println!("ERROR2"); 0 }
            }
        }
    }
}

fn score_for_when_paper(r2: &str) -> usize {
    let r2_rock = r2.cmp("A");
    let r2_paper = r2.cmp("B");
    let r2_scissors = r2.cmp("C");

    match r2_rock {
        std::cmp::Ordering::Equal => 1+0,
        _ => match r2_paper {
            std::cmp::Ordering::Equal => 2+3,
            _ => match r2_scissors {
                std::cmp::Ordering::Equal => 3+6,
                _ => { println!("ERROR2"); 0 }
            }
        }
    }
}

fn score_for_when_scissors(r2: &str) -> usize {
    let r2_rock = r2.cmp("A");
    let r2_paper = r2.cmp("B");
    let r2_scissors = r2.cmp("C");

    match r2_rock {
        std::cmp::Ordering::Equal => 1+6,
        _ => match r2_paper {
            std::cmp::Ordering::Equal => 2+0,
            _ => match r2_scissors {
                std::cmp::Ordering::Equal => 3+3,
                _ => { println!("ERROR2"); 0 }
            }
        }
    }
}

fn response_for_when_rock(r2: &str) -> &'static str {
    let r2_lose = r2.cmp("X");
    let r2_draw = r2.cmp("Y");
    let r2_win = r2.cmp("Z");

    match r2_lose {
        std::cmp::Ordering::Equal => "C",
        _ => match r2_draw {
            std::cmp::Ordering::Equal => "A",
            _ => match r2_win {
                std::cmp::Ordering::Equal => "B",
                _ => { println!("ERROR2"); 0 }
            }
        }
    }
}

fn response_for_when_paper(r2: &str) -> &'static str {
    let r2_lose = r2.cmp("X");
    let r2_draw = r2.cmp("Y");
    let r2_win = r2.cmp("Z");

    match r2_lose {
        std::cmp::Ordering::Equal => "A",
        _ => match r2_draw {
            std::cmp::Ordering::Equal => "B",
            _ => match r2_win {
                std::cmp::Ordering::Equal => "C",
                _ => { println!("ERROR2"); 0 }
            }
        }
    }
}

fn response_for_when_scissors(r2: &str) -> &'static str {
    let r2_lose = r2.cmp("X");
    let r2_draw = r2.cmp("Y");
    let r2_win = r2.cmp("Z");

    match r2_lose {
        std::cmp::Ordering::Equal => "B",
        _ => match r2_draw {
            std::cmp::Ordering::Equal => "C",
            _ => match r2_win {
                std::cmp::Ordering::Equal => "A",
                _ => { println!("ERROR2"); 0 }
            }
        }
    }
}

fn round_score(r1: &str, r2: &str) -> usize {
    let r1_rock = r1.cmp("A");
    let r1_paper = r1.cmp("B");
    let r1_scissors = r1.cmp("C");

    match r1_rock {
        std::cmp::Ordering::Equal => score_for_when_rock(response_for_when_rock(r2)),
        _ => match r1_paper {
            std::cmp::Ordering::Equal => score_for_when_paper(response_for_when_paper(r2)),
            _ => match r1_scissors {
                std::cmp::Ordering::Equal => score_for_when_scissors(response_for_when_scissors(r2)),
                _ => { println!("ERROR1"); 0 }
            }
        }
    }
}

fn main() {
    let args : Vec<_> = env::args().collect();
    let input_path = &args[1];
    let raw_input = fs::read(input_path).unwrap();
    let raw_string = String::from_utf8_lossy(&raw_input);
    let input : Vec<_> = raw_string.lines().collect();

    let total_score: usize = input.iter()
                        .map(|line| {
                            let mut it = line.split_whitespace();
                            let r1 = it.next().unwrap();
                            let r2 = it.next().unwrap();
                            round_score(r1, r2)
                        })
                        .sum();

    println!("Total score: {}", total_score);
}
