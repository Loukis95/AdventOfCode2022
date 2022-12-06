use std::{env, fs};

fn main() {
    let args : Vec<_> = env::args().collect();
    let input_path = &args[1];
    let raw_input = fs::read(input_path).unwrap();
    let raw_string = String::from_utf8_lossy(&raw_input);
    let input : Vec<_> = raw_string.lines().collect();
    let line = input.first().unwrap();

    let iter1 = line.chars();
    let iter2 = line.chars().skip(1);
    let iter3 = line.chars().skip(2);
    let iter4 = line.chars().skip(3);

    let (pos, _) = iter1.zip(iter2)
                            .zip(iter3)
                            .zip(iter4)
                            .map(|(((a, b), c), d)| (a, b, c, d))
                            .enumerate()
                            .find(|(_, (a,b,c,d))| {
                                a != b && a != c && a != d && b != c && b != d && c != d
                            })
                            .unwrap();
    println!("position: {}", pos+4);
}
