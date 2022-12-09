use std::{env, fs};

#[derive(Copy, Clone, PartialEq, Debug)]
struct Coord {
    x: isize,
    y: isize,
}

fn left(coord: &Coord) -> Coord {
    Coord{x:coord.x-1, y:coord.y}
}
fn right(coord: &Coord) -> Coord {
    Coord{x:coord.x+1, y:coord.y}
}
fn up(coord: &Coord) -> Coord {
    Coord{x:coord.x, y:coord.y-1}
}
fn down(coord: &Coord) -> Coord {
    Coord{x:coord.x, y:coord.y+1}
}

fn print_state(rope: &[Coord], positions: &[Coord], min_x: isize, min_y: isize, max_x: isize, max_y: isize) {
    let shift_x = min_x * (-1 as isize);
    let shift_y = min_y * (-1 as isize);
    let m = (max_y+shift_y+1) as usize;
    let n = (max_x+shift_x+1) as usize;
    let mut grid = vec![vec!['.';n];m];
    for coord in positions {
        grid[(coord.y+shift_y) as usize][(coord.x+shift_x) as usize] = '#';
    }
    grid[shift_y as usize][shift_x as usize] = 's';
    for i in 9..0 as usize {
        grid[(rope[i].y+shift_y) as usize][(rope[i].x+shift_x) as usize] = format!("{}", i).chars().nth(0).unwrap();
    }
    grid[(rope[0].y+shift_y) as usize][(rope[0].x+shift_x) as usize] = 'H';
    for j in 0..m {
        for i in 0..n {
            print!("{}", grid[j][i]);
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

    let mut min_x: isize = 0;
    let mut min_y: isize = 0;
    let mut max_x: isize = 0;
    let mut max_y: isize = 0;

    let mut rope = [Coord{x:0, y:0}; 10];

    let mut positions = Vec::<Coord>::new();
    positions.push(rope[9].clone());

    // print_state(&rope, &positions, min_x, min_y, max_x, max_y);

    for line in input.iter() {
        println!("==== {} ====", line);
        let direction = line.split_whitespace().nth(0).unwrap();
        let repeat = line.split_whitespace().nth(1).unwrap().parse::<usize>().unwrap();
        for _i in 0..repeat {
            // Move head
            match direction {
                "R" => rope[0] = right(&rope[0]),
                "L" => rope[0] = left(&rope[0]),
                "D" => rope[0] = down(&rope[0]),
                "U" => rope[0] = up(&rope[0]),
                _ => panic!("Unsupported direction")
            }
            if rope[0].x > max_x { max_x = rope[0].x; }
            if rope[0].y > max_y { max_y = rope[0].y; }
            if rope[0].x < min_x { min_x = rope[0].x; }
            if rope[0].y < min_y { min_y = rope[0].y; }
            for i in 1..10 as usize {
                let head = &rope[i-1];
                // Move tail
                let diffx = head.x - rope[i].x;
                let diffy = head.y - rope[i].y;
                if (diffx.abs() > 1 && diffy != 0) || (diffy.abs() > 1 && diffx != 0) {
                    // Diagonale
                    rope[i].y += diffy.signum()*1;
                    rope[i].x += diffx.signum()*1;
                }
                else if diffx.abs() > 1 {
                    // Horizontal
                    rope[i].x += diffx.signum()*1;
                }
                else if diffy.abs() > 1 {
                    // Vertical
                    rope[i].y += diffy.signum()*1;
                }
            }

            // Save new positions
            if !positions.contains(&rope[9]) {
                positions.push(rope[9].clone());
            }

            // print_state(&rope, &positions, min_x, min_y, max_x, max_y);
        }
    }

    println!("unique positions: {}", positions.len());
}
