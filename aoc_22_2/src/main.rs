use std::{env, fs};

const SIDE_LENGTH: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
enum Face {
    Front = 0,
    Left,
    Right,
    Bottom,
    Top,
    Back,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn opposite(dir: Direction) -> Direction {
        match dir {
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Up => Direction::Down,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rotation {
    Clockwise,
    CounterClockwise,
}

impl Rotation {
    fn opposite(rot: Rotation) -> Rotation {
        match rot {
            Rotation::Clockwise => Rotation::CounterClockwise,
            Rotation::CounterClockwise => Rotation::Clockwise,
        }
    }
}

#[derive(Debug, Clone)]
struct Transform {
    x: usize,
    y: usize,
    rot: Rotation,
    apply_n: usize,
}

#[derive(Debug, Clone)]
enum Cell {
    Char(char),
    Teleport(Transform),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize,
    direction: Direction,
}

fn wrap_around_position(mut from: Position, map: &[Vec<char>], max_x: usize, max_y: usize) -> Position {
    match from.direction {
        Direction::Right => {
            while from.x > 0 && map[from.y][from.x-1] != ' ' { from.x -= 1; }
        },
        Direction::Down => {
            while from.y > 0 && map[from.y-1][from.x] != ' ' { from.y -= 1; }
        },
        Direction::Left => {
            while from.x < max_x-1 && map[from.y][from.x+1] != ' ' { from.x += 1; }
        },
        Direction::Up => {
            while from.y < max_y-1 && map[from.y+1][from.x] != ' ' { from.y += 1; }
        },
    };
    return from;
}

fn move_n(n: usize, mut from: Position, map: &[Vec<char>], max_x: usize, max_y: usize) -> Position {
    for _ in 0..n {
        match from.direction {
            Direction::Right => {
                if from.x >= max_x-1 || map[from.y][from.x+1] == ' ' {
                    // wrap around
                    let next = wrap_around_position(from, map, max_x, max_y);
                    if map[next.y][next.x] == '#' {
                        // After wrapping around, there is a wall
                        // Move ends here
                        return from;
                    } else {
                        // Otherwise move here
                        from = next;
                    }
                } else if map[from.y][from.x+1] == '#' {
                    // There is a wall, move ends here
                    return from;
                } else {
                    // Ok advance
                    from.x += 1;
                }
            },
            Direction::Down => {
                if from.y >= max_y-1 || map[from.y+1][from.x] == ' ' {
                    // wrap around
                    let next = wrap_around_position(from, map, max_x, max_y);
                    if map[next.y][next.x] == '#' {
                        // After wrapping around, there is a wall
                        // Move ends here
                        return from;
                    } else {
                        // Otherwise move here
                        from = next;
                    }
                } else if map[from.y+1][from.x] == '#' {
                    // There is a wall, move ends here
                    return from;
                } else {
                    // Ok advance
                    from.y += 1;
                }
            },
            Direction::Left => {
                if from.x == 0 || map[from.y][from.x-1] == ' ' {
                    // wrap around
                    let next = wrap_around_position(from, map, max_x, max_y);
                    if map[next.y][next.x] == '#' {
                        // After wrapping around, there is a wall
                        // Move ends here
                        return from;
                    } else {
                        // Otherwise move here
                        from = next;
                    }
                } else if map[from.y][from.x-1] == '#' {
                    // There is a wall, move ends here
                    return from;
                } else {
                    // Ok advance
                    from.x -= 1;
                }
            },
            Direction::Up => {
                if from.y == 0 || map[from.y-1][from.x] == ' ' {
                    // wrap around
                    let next = wrap_around_position(from, map, max_x, max_y);
                    if map[next.y][next.x] == '#' {
                        // After wrapping around, there is a wall
                        // Move ends here
                        return from;
                    } else {
                        // Otherwise move here
                        from = next;
                    }
                } else if map[from.y-1][from.x] == '#' {
                    // There is a wall, move ends here
                    return from;
                } else {
                    // Ok advance
                    from.y -= 1;
                }
            },
        };
    }
    return from;
}

fn rotate(mut from: Position, rot: Rotation) -> Position {
    match from.direction {
        Direction::Right => {
            match rot {
                Rotation::Clockwise => from.direction = Direction::Down,
                Rotation::CounterClockwise => from.direction = Direction::Up,
            }
        },
        Direction::Down => {
            match rot {
                Rotation::Clockwise => from.direction = Direction::Left,
                Rotation::CounterClockwise => from.direction = Direction::Right,
            }
        },
        Direction::Left => {
            match rot {
                Rotation::Clockwise => from.direction = Direction::Up,
                Rotation::CounterClockwise => from.direction = Direction::Down,
            }
        },
        Direction::Up => {
            match rot {
                Rotation::Clockwise => from.direction = Direction::Right,
                Rotation::CounterClockwise => from.direction = Direction::Left,
            }
        },
    };
    from
}

fn main() {
    let args : Vec<_> = env::args().collect();
    let input_path = &args[1];
    
    // let input_path = "src/example.txt";

    let raw_input = fs::read(input_path).unwrap();
    let raw_string = String::from_utf8_lossy(&raw_input);
    let input : Vec<_> = raw_string.lines().collect();

    // Compute maximum x and y
    let mut max_x: usize = 0;
    let mut max_y = 0;

    // Build the map
    let data = input.iter()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            max_x = max_x.max(line.len());
            max_y += 1;
            line.chars().collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut map = vec![vec![' '; max_x]; max_y];
    data.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, c)| {
            map[y][x] = *c;
        });
    });

    // Starting position
    let position = {
        let mut position = Position { x: 0, y: 0, direction: Direction::Right };
        let mut found = false;
        for y in 0..max_y {
            for x in 0..max_x {
                if map[y][x] != ' ' {
                    position.x = x;
                    position.y = y;
                    found = true;
                    break;
                }
            }
            if found { break; }
        }
        position
    };

    // Build the cube
    let mut cube = [[[' '; SIDE_LENGTH]; SIDE_LENGTH]; 6];
    // Front face of the cube
    for j in 0..SIDE_LENGTH {
        for i in 0..SIDE_LENGTH {
            let x = position.x + i;
            let y = position.y + j;
            cube[Face::Front as usize][j][i] = map[y][x];
        }
    }
    // Immediately right is the right face
    if position.x+SIDE_LENGTH < max_x-1 && map[position.y][position.x+SIDE_LENGTH] != ' ' {
        let mut position = position;
        position.x += SIDE_LENGTH;
        for j in 0..SIDE_LENGTH {
            for i in 0..SIDE_LENGTH {
                let x = position.x + i;
                let y = position.y + j;
                cube[Face::Right as usize][j][i] = map[y][x];
            }
        }
    }
    // Bottom of front face would be the bottom face
    if position.y+SIDE_LENGTH < max_y-1 && map[position.y+SIDE_LENGTH][position.x] != ' ' {
        let mut position = position;
        position.y += SIDE_LENGTH;
        for j in 0..SIDE_LENGTH {
            for i in 0..SIDE_LENGTH {
                let x = position.x + i;
                let y = position.y + j;
                cube[Face::Bottom as usize][j][i] = map[y][x];
            }
        }
        // Left of the bottom face would be the left face
        if position.x >= SIDE_LENGTH && map[position.y][position.x-SIDE_LENGTH] != ' ' {
            let mut position = position;
            position.x -= SIDE_LENGTH;
            for j in 0..SIDE_LENGTH {
                for i in 0..SIDE_LENGTH {
                    let x = position.x + i;
                    let y = position.y + j;
                    cube[Face::Left as usize][i][SIDE_LENGTH-j-1] = map[y][x];
                }
            }
            // Left of the left face would be the top face
            if position.x >= SIDE_LENGTH && map[position.y][position.x-SIDE_LENGTH] != ' ' {
                let mut position = position;
                position.x -= SIDE_LENGTH;
                for j in 0..SIDE_LENGTH {
                    for i in 0..SIDE_LENGTH {
                        let x = position.x + i;
                        let y = position.y + j;
                        cube[Face::Top as usize][SIDE_LENGTH-j-1][SIDE_LENGTH-i-1] = map[y][x];
                    }
                }
            }
        }
        // Bottom of bottom face would be the back face
        if position.y+SIDE_LENGTH < max_y-1 && map[position.y+SIDE_LENGTH][position.x] != ' ' {
            let mut position = position;
            position.y += SIDE_LENGTH;
            for j in 0..SIDE_LENGTH {
                for i in 0..SIDE_LENGTH {
                    let x = position.x + i;
                    let y = position.y + j;
                    cube[Face::Back as usize][SIDE_LENGTH-j-1][i] = map[y][x];
                }
            }
            // Right of the back face would be the right face
            if position.x+SIDE_LENGTH < max_x-1 && map[position.y][position.x+SIDE_LENGTH] != ' ' {
                let mut position = position;
                position.x += SIDE_LENGTH;
                for j in 0..SIDE_LENGTH {
                    for i in 0..SIDE_LENGTH {
                        let x = position.x + i;
                        let y = position.y + j;
                        cube[Face::Right as usize][SIDE_LENGTH-j-1][SIDE_LENGTH-i-1] = map[y][x];
                    }
                }
            }
        }
    }

    println!("FRONT FACE:");
    for j in 0..SIDE_LENGTH {
        for i in 0..SIDE_LENGTH {
            print!("{}", cube[Face::Front as usize][j][i]);
        }
        println!("");
    }
    
    println!("LEFT FACE:");
    for j in 0..SIDE_LENGTH {
        for i in 0..SIDE_LENGTH {
            print!("{}", cube[Face::Left as usize][j][i]);
        }
        println!("");
    }

    println!("RIGHT FACE:");
    for j in 0..SIDE_LENGTH {
        for i in 0..SIDE_LENGTH {
            print!("{}", cube[Face::Right as usize][j][i]);
        }
        println!("");
    }

    println!("BOTTOM FACE:");
    for j in 0..SIDE_LENGTH {
        for i in 0..SIDE_LENGTH {
            print!("{}", cube[Face::Bottom as usize][j][i]);
        }
        println!("");
    }

    println!("TOP FACE:");
    for j in 0..SIDE_LENGTH {
        for i in 0..SIDE_LENGTH {
            print!("{}", cube[Face::Top as usize][j][i]);
        }
        println!("");
    }

    println!("BACK FACE:");
    for j in 0..SIDE_LENGTH {
        for i in 0..SIDE_LENGTH {
            print!("{}", cube[Face::Back as usize][j][i]);
        }
        println!("");
    }

    // Read the instructions from the monkeys
    let mut repeat: usize = 0;
    let mut pos = position;
    for char in input.last().unwrap().chars() {
        if char.is_digit(10) {
            repeat *= 10;
            repeat += char.to_digit(10).unwrap() as usize;
        } else {
            if repeat != 0 {
                pos = move_n(repeat, pos, &map, max_x, max_y);
                repeat = 0;
            }
            if char == 'R' {
                pos = rotate(pos, Rotation::Clockwise);
            }
            else if char == 'L' {
                pos = rotate(pos, Rotation::CounterClockwise);
            }
            else {
                panic!("Unexpected input char: '{}'", char);
            }
        }
    }
    if repeat != 0 {
        pos = move_n(repeat, pos, &map, max_x, max_y);
    }

    let score = {
        1000 * (pos.y+1)
        + 4 * (pos.x+1)
        + match pos.direction {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
    };

    println!("Final position: {:?}", pos);
    println!("Score: {}", score);
}
