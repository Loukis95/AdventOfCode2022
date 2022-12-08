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

    let forest = input.iter()
        .map(|line| {
            line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect::<Vec<_>>()
        }).collect::<Vec<_>>();
    
    let m = forest.len();
    let n = forest[0].len();
    println!("Forest size: {}x{}", n, m);

    let mut tree_score = vec![vec![0 as usize; n]; m];

    for j in 0..m {
        for i in 0..n {
            let height = forest[j][i];
            let mut found = [0 as usize;4];
            let mut x = i;
            let mut y = j;
            while let Some((u, v)) = neighbour_left(x, y, n, m) {
                found[0] += 1;
                if forest[v][u] >= height {
                    break;
                }
                x = u;
                y = v;
            }
            x = i;
            y = j;
            while let Some((u, v)) = neighbour_right(x, y, n, m) {
                found[1] += 1;
                if forest[v][u] >= height {
                    break;
                }
                x = u;
                y = v;
            }
            x = i;
            y = j;
            while let Some((u, v)) = neighbour_up(x, y, n, m) {
                found[2] += 1;
                if forest[v][u] >= height {
                    break;
                }
                x = u;
                y = v;
            }
            x = i;
            y = j;
            while let Some((u, v)) = neighbour_down(x, y, n, m) {
                found[3] += 1;
                if forest[v][u] >= height {
                    break;
                }
                x = u;
                y = v;
            }
            tree_score[j][i] = found[0] * found[1] * found[2] * found[3];
        }
    }
    
    let highest_score = tree_score.iter().map(|line|{
        line.iter().max().unwrap()
    }).max().unwrap();

    println!("Highest scenic score: {}", highest_score);
}