use std::{env, fs};

enum Values<T> {
    Skip,
    Value(T)
}

fn main() {
    let args : Vec<_> = env::args().collect();
    let input_path = &args[1];
    let raw_input = fs::read(input_path).unwrap();
    let raw_string = String::from_utf8_lossy(&raw_input);
    let input : Vec<_> = raw_string.lines().collect();

    let mut max_values = input.iter()
                        .scan(0, |state: &mut usize, &x| {
                            if x.is_empty() {
                                let ret = Some(Values::Value(*state));
                                *state = 0;
                                ret
                            } else {
                                *state += x.parse::<usize>().unwrap();
                                Some(Values::Skip)
                            }
                        })
                        .filter_map(|x| {
                            match x {
                                Values::Skip => None,
                                Values::Value(v) => Some(v),
                            }
                        })
                        .collect::<Vec<_>>();

    max_values.sort_unstable();
    
    println!("Sorted: {}, {}, {}", max_values[0], max_values[1], max_values[2]);

    let sum_of_top_three: usize = max_values.iter()
                        .rev()
                        .take(3)
                        .sum();

    println!("Value: {}", sum_of_top_three);
}
