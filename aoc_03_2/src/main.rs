use std::{env, fs};

fn priority(c: &char) -> usize {
    match c {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 27,
        'B' => 28,
        'C' => 29,
        'D' => 30,
        'E' => 31,
        'F' => 32,
        'G' => 33,
        'H' => 34,
        'I' => 35,
        'J' => 36,
        'K' => 37,
        'L' => 38,
        'M' => 39,
        'N' => 40,
        'O' => 41,
        'P' => 42,
        'Q' => 43,
        'R' => 44,
        'S' => 45,
        'T' => 46,
        'U' => 47,
        'V' => 48,
        'W' => 49,
        'X' => 50,
        'Y' => 51,
        'Z' => 52,
        _ => { println!("error"); 0 }
    }
}

fn main() {
    let args : Vec<_> = env::args().collect();
    let input_path = &args[1];
    let raw_input = fs::read(input_path).unwrap();
    let raw_string = String::from_utf8_lossy(&raw_input);
    let input : Vec<_> = raw_string.lines().collect();

    let total_score: usize = input.iter()
                        .zip(input.iter().skip(1))
                        .zip(input.iter().skip(2))
                        .step_by(3)
                        .map(|((e1, e2), e3)| {
                            println!("{} - {} - {}", e1, e2, e3);
                            let mut v1 = e1.chars().collect::<Vec<_>>();
                            let mut v2 = e2.chars().collect::<Vec<_>>();
                            let mut v3 = e3.chars().collect::<Vec<_>>();
                            v1.sort_unstable();
                            v2.sort_unstable();
                            v3.sort_unstable();
                            v1.dedup();
                            v2.dedup();
                            v3.dedup();
                            for c in v1.iter() {
                                let r2 = v2.binary_search(c);
                                if r2.is_ok() {
                                    let r3 = v3.binary_search(c);
                                    if r3.is_ok() {
                                        return priority(c);
                                    }
                                }
                            }
                            println!("ERROR");
                            0
                        })
                        .sum();

    println!("Total score: {}", total_score);
}
