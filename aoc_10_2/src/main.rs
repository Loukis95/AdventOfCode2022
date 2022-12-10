use std::{env, fs};

#[derive(Debug, Clone, Copy)]
struct ProcessorState {
    cycles: usize,
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

fn print_screen(screen: &Vec<Vec<char>>) {
    for j in 0..6 {
        for i in 0..40 {
            print!("{}", screen[j][i]);
        }
        println!("");
    }
    println!("");
}

fn main() {
    let args : Vec<_> = env::args().collect();
    let input_path = &args[1];
    let raw_input = fs::read(input_path).unwrap();
    let raw_string = String::from_utf8_lossy(&raw_input);
    let input : Vec<_> = raw_string.lines().collect();

    let mut state = ProcessorState::new();
    let mut screen = vec![vec![' '; 40]; 6];

    input.iter()
        .for_each(|line| {
            let mut it = line.split_whitespace();
            let instruction = it.next().unwrap();
            match instruction {
                "noop" => {
                    state.cycles += 1;

                    let y = (state.cycles-1)/40;
                    let x = (state.cycles-1)%40;
                    if (x as isize) >= state.X-1 && (x as isize) <= state.X+1 {
                        screen[y][x] = '#';
                    } else {
                        screen[y][x] = '.';
                    }
                },
                "addx" => {
                    state.cycles += 1;

                    let y = (state.cycles-1)/40;
                    let x = (state.cycles-1)%40;
                    if (x as isize) >= state.X-1 && (x as isize) <= state.X+1 {
                        screen[y][x] = '#';
                    } else {
                        screen[y][x] = '.';
                    }
                    
                    state.cycles += 1;
                    
                    let y = (state.cycles-1)/40;
                    let x = (state.cycles-1)%40;
                    if (x as isize) >= state.X-1 && (x as isize) <= state.X+1 {
                        screen[y][x] = '#';
                    } else {
                        screen[y][x] = '.';
                    }
                    
                    state.X += it.next().unwrap().parse::<isize>().unwrap();
                    
                },
                _ => panic!("Unexpected instruction"),
            }
        });

    print_screen(&screen);

}
