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

    let max_value = input.iter()
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
                        .max().unwrap();

    println!("Value: {}", max_value);
}
