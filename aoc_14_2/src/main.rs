use std::{env, fs, str::FromStr, num::ParseIntError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    const MAX: Point = Point{x:usize::MAX, y:usize::MAX};
    const MIN: Point = Point{x:usize::MIN, y:usize::MIN};

    pub const fn new(x: usize, y: usize) -> Self {
        Self {
            x, y,
        }
    }

    pub const fn left(&self) -> Point {
        Point{x:self.x-1, y:self.y}
    }
    pub const fn right(&self) -> Point {
        Point{x:self.x+1, y:self.y}
    }
    pub const fn up(&self) -> Point {
        Point{x:self.x, y:self.y-1}
    }
    pub const fn down(&self) -> Point {
        Point{x:self.x, y:self.y+1}
    }
    pub const fn down_left(&self) -> Point {
        self.down().left()
    }
    pub const fn down_right(&self) -> Point {
        self.down().right()
    }
    pub const fn up_left(&self) -> Point {
        self.up().left()
    }
    pub const fn up_right(&self) -> Point {
        self.up().right()
    }

    pub const fn is_left(&self, other: &Point) -> bool {
        self.y == other.y && other.x > self.x
    }
    pub const fn is_right(&self, other: &Point) -> bool {
        self.y == other.y && other.x < self.x
    }
    pub const fn is_up(&self, other: &Point) -> bool {
        self.x == other.x && other.y > self.y
    }
    pub const fn is_down(&self, other: &Point) -> bool {
        self.x == other.x && other.y < self.y
    }

    pub fn collides_with_lines(&self, lines: &[(Point, Point)]) -> bool {
        for (begin, end) in lines {
            if self == begin || self == end { return true; }
            else if begin.is_left(end) && self.is_right(begin) && self.is_left(end) { return true; }
            else if begin.is_right(end) && self.is_left(begin) && self.is_right(end) { return true; }
            else if begin.is_up(end) && self.is_down(begin) && self.is_up(end) { return true; }
            else if begin.is_down(end) && self.is_up(begin) && self.is_down(end) { return true; }
        }
        return false;
    }

    pub fn collides_with_points(&self, points: &[Point]) -> bool {
        for point in points {
            if self == point { return true; }
        }
        return false;
    }
}

#[derive(Debug)]
enum ParsePointError {
    ParseIntError(ParseIntError),
    ParsePointError
}

impl From<ParseIntError> for ParsePointError{
    fn from(error: ParseIntError) -> Self {
        ParsePointError::ParseIntError(error)
    }
}

impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(',');
        if let Some(x) = it.next() {
            if let Some(y) = it.next() {
                return Ok(Point::new(x.parse()?,y.parse()?));
            }
        }
        Err(ParsePointError::ParsePointError)
    }
}


fn print_state(sand_source: &Point, rocks: &[(Point, Point)], sand_positions: &[Point], min: &Point, max: &Point) {
    let shift = min;
    let m = (max.y-shift.y+1) as usize;
    let n = (max.x-shift.x+1) as usize;
    let mut grid = vec![vec!['.';n];m];
    for (mut begin, end) in rocks {
        while begin != *end {
            grid[(begin.y-shift.y) as usize][(begin.x-shift.x) as usize] = '#';
            if begin.is_left(end) { begin = begin.right() }
            else if begin.is_right(end) { begin = begin.left() }
            else if begin.is_up(end) { begin = begin.down() }
            else if begin.is_down(end) { begin = begin.up() }
        }
        grid[(end.y-shift.y) as usize][(end.x-shift.x) as usize] = '#';
    }
    for pos in sand_positions {
        grid[(pos.y-shift.y) as usize][(pos.x-shift.x) as usize] = 'o';
    }
    grid[(sand_source.y-shift.y) as usize][(sand_source.x-shift.x) as usize] = '+';
    for j in 0..m {
        for i in 0..n {
            print!("{}", grid[j][i]);
        }
        println!("");
    }
    println!("");
}

fn main() {
    // let args : Vec<_> = env::args().collect();
    // let input_path = &args[1];
    let input_path = "src/input.txt";
    let raw_input = fs::read(input_path).unwrap();
    let raw_string = String::from_utf8_lossy(&raw_input);
    let input : Vec<_> = raw_string.lines().collect();

    let mut min = Point::MAX;
    let mut max = Point::MIN;

    let sand_source = Point::new(500, 0);
    if min.x > sand_source.x { min.x = sand_source.x }
    if min.y > sand_source.x { min.y = sand_source.y }
    if max.x < sand_source.x { max.x = sand_source.x }
    if max.y < sand_source.y { max.y = sand_source.y }

    let mut rocks = input.iter().flat_map(|line| {
        let point_iterator_1 = line.split(" -> ").map(|coordinates| {
            let point = coordinates.parse::<Point>().unwrap();
            if min.x > point.x { min.x = point.x }
            if min.y > point.x { min.y = point.y }
            if max.x < point.x { max.x = point.x }
            if max.y < point.y { max.y = point.y }
            return point
        });
        let point_iterator_2 = line.split(" -> ").map(|coordinates| {
            coordinates.parse::<Point>().unwrap()
        });
        
        point_iterator_1.zip(point_iterator_2.skip(1)).collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();

    max.y += 2;
    rocks.push((Point::new(usize::MIN,max.y), Point::new(usize::MAX,max.y)));

    let mut sand_positions = Vec::<Point>::new();

    let mut iteration: usize = 0;
    println!("==== Iteration #{} ====", iteration);
    print_state(&sand_source, &rocks, &sand_positions, &min, &max);

    loop {
        let mut sand = sand_source.clone();
        loop {
            // Stop when falling in the abyss
            if sand == sand_source { break }

            if !sand.down().collides_with_lines(&rocks) && !sand.down().collides_with_points(&sand_positions){
                sand = sand.down();
            }
            else if !sand.down_left().collides_with_lines(&rocks) && !sand.down_left().collides_with_points(&sand_positions){
                sand = sand.down_left();
            }
            else if !sand.down_right().collides_with_lines(&rocks) && !sand.down_right().collides_with_points(&sand_positions){
                sand = sand.down_right();
            }
            else {
                sand_positions.push(sand);
                iteration += 1;
                println!("==== Iteration #{} ====", iteration);
                print_state(&sand_source, &rocks, &sand_positions, &min, &max);
                break;
            }
        }
        // Stop when falling in the abyss
        if sand == sand_source { break }
    }

    println!("result: {}", iteration);
}
