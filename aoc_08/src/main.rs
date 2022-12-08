use std::{env, fs};

fn neighbour_left(i: usize, j: usize, n: usize, m: usize) -> Option<(usize, usize)> {
    if i == 0 {
        None
    } else {
        Some((i-1, j))
    }
}
fn neighbour_right(i: usize, j: usize, n: usize, m: usize) -> Option<(usize, usize)> {
    if i == n-1 {
        None
    } else {
        Some((i+1, j))
    }
}
fn neighbour_up(i: usize, j: usize, n: usize, m: usize) -> Option<(usize, usize)> {
    if j == 0 {
        None
    } else {
        Some((i, j-1))
    }
}
fn neighbour_down(i: usize, j: usize, n: usize, m: usize) -> Option<(usize, usize)> {
    if j == m-1 {
        None
    } else {
        Some((i, j+1))
    }
}

fn main() {
    let args : Vec<_> = env::args().collect();
    let input_path = &args[1];
    let raw_input = fs::read(input_path).unwrap();
    let raw_string = String::from_utf8_lossy(&raw_input);
    let input : Vec<_> = raw_string.lines().collect();
    let line = input.first().unwrap();

    let forest = input.iter()
        .map(|line| {
            line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect::<Vec<_>>()
        }).collect::<Vec<_>>();
    
    let m = forest.len();
    let n = forest[0].len();
    println!("Forest size: {}x{}", n, m);

    let mut nb_hidden_trees : usize = m*2+n*2-4;

    for j in 1..m-1 {
        for i in 1..n-1 {
            let height = forest[j][i];
            let mut found = [false;4];
            let mut x = i;
            let mut y = j;
            while let Some((u, v)) = neighbour_left(x, y, n, m) {
                if forest[v][u] >= height {
                    found[0] = true;
                    break;
                }
                x = u;
                y = v;
            }
            x = i;
            y = j;
            while let Some((u, v)) = neighbour_right(x, y, n, m) {
                if forest[v][u] >= height {
                    found[1] = true;
                    break;
                }
                x = u;
                y = v;
            }
            x = i;
            y = j;
            while let Some((u, v)) = neighbour_up(x, y, n, m) {
                if forest[v][u] >= height {
                    found[2] = true;
                    break;
                }
                x = u;
                y = v;
            }
            x = i;
            y = j;
            while let Some((u, v)) = neighbour_down(x, y, n, m) {
                if forest[v][u] >= height {
                    found[3] = true;
                    break;
                }
                x = u;
                y = v;
            }
            if !found[0] || !found[1] || !found[2] || !found[3] { nb_hidden_trees += 1; println!("({}, {})",i,j); }
        }
    }

    println!("Hidden trees: {}", nb_hidden_trees);
}