use std::{env, fs};

fn move_n_from_x_to_y(stacks: &mut Vec<Vec<char>>, n: usize, x: usize, y: usize) {
    for _i in 0..n {
        if let Some(c) = stacks[x].pop() {
            stacks[y].push(c);
        }
    }
}

fn main() {
    let args : Vec<_> = env::args().collect();
    let input_path = &args[1];
    let raw_input = fs::read(input_path).unwrap();
    let raw_string = String::from_utf8_lossy(&raw_input);
    let input : Vec<_> = raw_string.lines().collect();

    let (pos, _) = input.iter().enumerate().find(|(_, line)| line.is_empty()).unwrap();
    let (stacks_str, program) = input.split_at(pos);
    let nb_stacks = stacks_str.last().unwrap().split_whitespace().last().unwrap().parse::<usize>().unwrap();

    println!("nb_stacks {}", nb_stacks);

    let mut stacks: Vec<Vec<char>> = vec![vec![]; nb_stacks];
    stacks_str.iter()
        .rev()
        .skip(1)
        .for_each(|line| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c != '[' && *c != ']' && *c != ' ')
                .for_each(|(pos, c)| {
                    let stack_nb = (pos-1)/4;
                    println!("{}: {} in stack {}", pos, c, stack_nb);
                    stacks[stack_nb].push(c);
                });
        });

    println!("{:?}", stacks);
    
    program.iter().skip(1)
        .for_each(|line| {
            println!("{}", line);
            let pr = line.split("move from to")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            println!("{:?}", pr);
            let (n, from, to) = (pr[0], pr[1]-1, pr[2]-1);
            move_n_from_x_to_y(&mut stacks, n, from, to);
            println!("{:?}", stacks);
        });
        
    print!("response: ");
    for i in 0..nb_stacks {
        if let Some(c) = stacks[i].last() {
            print!("{}", c);
        }
    }
    println!("");

}
