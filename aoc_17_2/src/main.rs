use std::{env, fs};

pub trait Move {
    fn r#move_n(&mut self, direction: Direction, n: isize);

    // Auto implemented
    fn r#move(&mut self, direction: Direction) {
        self.move_n(direction, 1);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
    DownLeft,
    DownRight,
    UpLeft,
    UpRight,
}

impl Direction {
    pub const fn opposite(dir: Direction) -> Direction {
        match dir {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::DownLeft => Direction::UpRight,
            Direction::DownRight => Direction::UpLeft,
            Direction::UpLeft => Direction::DownRight,
            Direction::UpRight => Direction::DownLeft,
        }
    }
}

const NB_ROCKS: usize = 1000000000000; // 1000 billions of rocks
const RIGHT_SHIFT: usize = 2;
const UP_SHIFT: usize = 3;
const CAVE_WIDTH: usize = 7;

#[derive(Debug, Clone)]
pub struct Rock {
    y: usize,
    data: Vec<u8>,
}

impl Rock {
    pub const fn new(data: Vec<u8>) -> Self {
        Self { y: 0, data }
    }
}

impl Move for Rock {
    fn r#move(&mut self, direction: Direction) {
        match direction {
            Direction::Left => {
                let accumulated = self.data.iter().fold(0, |acc, value| acc|value);
                if accumulated & (0x1 << CAVE_WIDTH-1) == 0 {
                    self.data.iter_mut().for_each(|v| { *v = *v << 1 });
                }
            },
            Direction::Right => {
                let accumulated = self.data.iter().fold(0, |acc, value| acc|value);
                if accumulated & 0x1 == 0 {
                    self.data.iter_mut().for_each(|v| { *v = *v >> 1 });
                }
            },
            Direction::Up => {
                self.y += 1;
            },
            Direction::Down => {
                self.y -= 1;
            },
            Direction::DownLeft => todo!(),
            Direction::DownRight => todo!(),
            Direction::UpLeft => todo!(),
            Direction::UpRight => todo!(),
        }
    }

    fn r#move_n(&mut self, direction: Direction, n: isize) {
        for _ in 0..n {
            self.r#move(direction);
        }
    }
}


fn main() {
    let args : Vec<_> = env::args().collect();
    let input_path = &args[1];
    let raw_input = fs::read(input_path).unwrap();
    let raw_string = String::from_utf8_lossy(&raw_input);
    let input : Vec<_> = raw_string.lines().collect();

    // Gas jets
    let mut gas_jets = input.iter().flat_map(|line| {
        line.chars().map(|c| {
            match c {
                '<' => Direction::Left,
                '>' => Direction::Right,
                _ => panic!("Corrupted input"),
            }
        }).collect::<Vec<_>>()
    }).cycle();

    // Rocks
    let rocks = vec![
        Rock::new(vec![0x78]),
        Rock::new(vec![0x20, 0x70, 0x20]),
        Rock::new(vec![0x10, 0x10, 0x70]),
        Rock::new(vec![0x40, 0x40, 0x40, 0x40]),
        Rock::new(vec![0x60, 0x60]),
    ];
    let mut rocks_generator = rocks.iter().cycle();

    // Lets run the simulation
    let mut archive = Vec::<usize>::new();
    let mut progress: usize = 0;
    let mut max_height: usize = 0;
    let mut fallen_rocks = Vec::<u8>::new();

    let mut cycle_y: usize = 0;         // Height where the first cycle starts
    let mut start_y: usize = 0;         // Height before the beginning of the cycle
    let mut cycle_height: usize = 0;    // Height of the cycle

    for rock in &mut rocks_generator {
        // Spawn a new rock
        let mut rock = rock.clone();
        // println!("Spawned rock: {:?}", rock);
        rock.move_n(Direction::Right, RIGHT_SHIFT as isize);
        rock.move_n(Direction::Up, (UP_SHIFT+max_height+1) as isize);
        // println!("Falling rock: {:?}", rock);
        // Always keep some empty space on top of other pieces
        for _ in fallen_rocks.len()-max_height..UP_SHIFT+5 {
            fallen_rocks.push(0);
        }

        // Simulate the fall
        loop {
            // println!("Falling rock: {:?}", rock);
            // Move in the jet direction
            let jet_direction = gas_jets.next().unwrap();
            rock.r#move(jet_direction);
            // Check if new position collides with other rocks
            if rock.data.iter().rev().enumerate().any(|(n, value)| {
                let v = fallen_rocks[rock.y+n];
                v & value != 0
            })
            {
                // Undo the move if a collision happen
                rock.r#move(Direction::opposite(jet_direction));
            }
            // println!("Falling rock: {:?}", rock);
            // Move down
            rock.r#move(Direction::Down);
            // Check if new position collides with anything (cave or other rocks)
            if rock.data.iter().rev().enumerate().any(|(n, value)| {
                let v = fallen_rocks[rock.y+n];
                v & value != 0 || rock.y <= 0
            })
            {
                // Undo the move
                rock.r#move(Direction::Up);
                // Settle the rock here
                for (n, value) in rock.data.iter().rev().enumerate() {
                    fallen_rocks[rock.y+n] |= value;
                }
                // Compute the new max_height
                let (n, _value) = fallen_rocks.iter().enumerate().rev().find(|(_n, value)| **value != 0 ).unwrap();
                max_height = n;
                progress += 1;
                // Break the loop
                break;
            }
        }
        archive.push(max_height);
        println!("Fallen rocks: {}, max_height: {}", progress, max_height);

        // Search for a cycle in the data
        let mut iterator = fallen_rocks.iter().enumerate().rev().skip_while(|(n, _d)| n > &max_height);
        let mut data_iterator = iterator.clone().skip(1);
        let mut found = false;
        if let Some((current_y, current)) = iterator.next() {
            while let Some((data_y, data)) = data_iterator.next() {
                if data == current {
                    let mut zip_iter = iterator.clone().zip(data_iterator.clone()).take(current_y-data_y);
                    found = true;
                    while let Some(((_current_y, current),(_data_y, data))) = zip_iter.next() {
                        if current != data {
                            found = false;
                            break
                        }
                    }
                    // Found a cycle
                    if found && current_y-data_y > 10 {
                        cycle_height = current_y-data_y;
                        cycle_y = data_y;
                        start_y = data_y-cycle_height;

                        println!("Found a cycle from height {} to {}", current_y, data_y);
                        println!("Starting height before first cycle: {}", start_y);
                        for i in 0..cycle_height {
                            println!("{:03}: {:#04x}  -  {:03}: {:#04x}", current_y-i, fallen_rocks[current_y-i], data_y-i, fallen_rocks[data_y-i]);
                        }
                        for i in 0..start_y {
                            println!("           -  {:03}: {:#04x}", start_y-i, fallen_rocks[start_y-i]);
                        }
                        break;
                    }
                }
            }
        }
        if found {
            break;
        }
    }

    println!("");
    let nb_rocks_before_first_cycle = archive.iter().enumerate().find(|(_n, val)| **val == start_y).unwrap().0+1;
    let nb_rocks_before_second_cycle = archive.iter().enumerate().find(|(_n, val)| **val == cycle_y).unwrap().0+1;
    let nb_rocks_in_cycle = nb_rocks_before_second_cycle - nb_rocks_before_first_cycle;
    println!("nb_rocks_before_first_cycle: {}", nb_rocks_before_first_cycle);
    println!("nb_rocks_before_second_cycle: {}", nb_rocks_before_second_cycle);
    println!("nb_rocks_in_cycle: {}", nb_rocks_in_cycle);

    let sim_length = NB_ROCKS - nb_rocks_before_first_cycle;
    let nb_cycles = sim_length / nb_rocks_in_cycle;
    let remaining_rocks = sim_length % nb_rocks_in_cycle;

    println!("nb_cycles: {}", nb_cycles);
    println!("remaining_rocks: {}", remaining_rocks);
    println!("");

    println!("starting_height: {}", archive[nb_rocks_before_first_cycle-1]);
    println!("nb_cycles*cycle_height: {}", nb_cycles*cycle_height);

    let max_height = nb_cycles*cycle_height+archive[nb_rocks_before_first_cycle+remaining_rocks-1];

    println!("");
    println!("Max height: {}", max_height);
}
