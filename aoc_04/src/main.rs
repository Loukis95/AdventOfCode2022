use std::{env, fs};

fn main() {
    let args : Vec<_> = env::args().collect();
    let input_path = &args[1];
    let raw_input = fs::read(input_path).unwrap();
    let raw_string = String::from_utf8_lossy(&raw_input);
    let input : Vec<_> = raw_string.lines().collect();

    let total_score: usize = input.iter()
                                .filter(|line| {
                                    let assignments = line.split(',').collect::<Vec<_>>();
                                    let range1 = assignments[0].split('-').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
                                    let range2 = assignments[1].split('-').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
                                    (range1[0] >= range2[0] && range1[1] <= range2[1]) || (range2[0] >= range1[0] && range2[1] <= range1[1])
                                }).count();

    println!("Total score: {}", total_score);
}
