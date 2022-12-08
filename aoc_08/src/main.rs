use std::{env, fs};

fn main() {
    let args : Vec<_> = env::args().collect();
    let input_path = &args[1];
    let raw_input = fs::read(input_path).unwrap();
    let raw_string = String::from_utf8_lossy(&raw_input);
    let input : Vec<_> = raw_string.lines().collect();
    let line = input.first().unwrap();

    let mut octopus = input.iter()
    .map(|line| {
        line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    

}