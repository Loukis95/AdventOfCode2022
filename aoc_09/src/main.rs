use std::{env, fs};

#[derive(Clone, PartialEq, Debug)]
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

fn print_state(tail: &Coord, head: &Coord, positions: &[Coord], min_x: isize, min_y: isize, max_x: isize, max_y: isize) {
    let shift_x = min_x * (-1 as isize);
    let shift_y = min_y * (-1 as isize);
    let m = (max_y+shift_y+1) as usize;
    let n = (max_x+shift_x+1) as usize;
    let mut grid = vec![vec!['.';n];m];
    for coord in positions {
        grid[(coord.y+shift_y) as usize][(coord.x+shift_x) as usize] = '#';
    }
    grid[shift_y as usize][shift_x as usize] = 's';
    grid[(tail.y+shift_y) as usize][(tail.x+shift_x) as usize] = 'T';
    grid[(head.y+shift_y) as usize][(head.x+shift_x) as usize] = 'H';
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

    let mut head: Coord = Coord{x:0, y:0};
    let mut tail: Coord = Coord{x:0, y:0};

    let mut positions = Vec::<Coord>::new();
    positions.push(tail.clone());

    print_state(&tail, &head, &positions, min_x, min_y, max_x, max_y);

    for line in input.iter() {
        println!("==== {} ====", line);
        let direction = line.split_whitespace().nth(0).unwrap();
        let repeat = line.split_whitespace().nth(1).unwrap().parse::<usize>().unwrap();
        for _i in 0..repeat {
            // Move head
            match direction {
                "R" => head = right(&head),
                "L" => head = left(&head),
                "D" => head = down(&head),
                "U" => head = up(&head),
                _ => panic!("Unsupported direction")
            }
            if head.x > max_x { max_x = head.x; }
            if head.y > max_y { max_y = head.y; }
            if head.x < min_x { min_x = head.x; }
            if head.y < min_y { min_y = head.y; }
            // Move tail
            let diffx = head.x - tail.x;
            let diffy = head.y - tail.y;
            if (diffx.abs() > 1 && diffy != 0) || (diffy.abs() > 1 && diffx != 0) {
                // Diagonale
                tail.y += diffy.signum()*1;
                tail.x += diffx.signum()*1;
            }
            else if diffx.abs() > 1 {
                // Horizontal
                tail.x += diffx.signum()*1;
            }
            else if diffy.abs() > 1 {
                // Vertical
                tail.y += diffy.signum()*1;
            }

            // Save new positions
            if !positions.contains(&tail) {
                positions.push(tail.clone());
            }

            print_state(&tail, &head, &positions, min_x, min_y, max_x, max_y);
        }
    }

    println!("unique positions: {}", positions.len());
}
