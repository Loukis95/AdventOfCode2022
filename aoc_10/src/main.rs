use std::{env, fs};

#[derive(Debug, Clone, Copy)]
struct ProcessorState {
    cycles: isize,
    X: isize,
}

impl ProcessorState {
    fn new() -> Self {
        Self {
            cycles: 0,
            X: 1,
        }
    }
}

fn main() {
    let args : Vec<_> = env::args().collect();
    let input_path = &args[1];
    let raw_input = fs::read(input_path).unwrap();
    let raw_string = String::from_utf8_lossy(&raw_input);
    let input : Vec<_> = raw_string.lines().collect();

    let mut state = ProcessorState::new();

    let sum: isize = input.iter()
        .filter_map(|line| {
            let mut ret: Option<isize> = None;
            let mut it = line.split_whitespace();
            let instruction = it.next().unwrap();
            match instruction {
                "noop" => {
                    state.cycles += 1;

                    if (state.cycles-20)%40 == 0 {
                        ret = Some(state.cycles*state.X);
                        println!("{}: X:{} => {}", state.cycles, state.X, state.cycles*state.X);
                    }
                },
                "addx" => {
                    state.cycles += 1;

                    if (state.cycles-20)%40 == 0 {
                        ret = Some(state.cycles*state.X);
                        println!("{}: X:{} => {}", state.cycles, state.X, state.cycles*state.X);
                    }
                    
                    state.cycles += 1;
                    
                    if (state.cycles-20)%40 == 0 {
                        ret = Some(state.cycles*state.X);
                        println!("{}: X:{} => {}", state.cycles, state.X, state.cycles*state.X);
                    }
                    
                    state.X += it.next().unwrap().parse::<isize>().unwrap();
                    
                },
                _ => panic!("Unexpected instruction"),
            }
            ret
        })
        .sum();

    println!("Value: {}", sum);
}
