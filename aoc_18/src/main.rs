use std::{env, fs};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Point {
    pub const MAX: Point = Point{x:isize::MAX, y:isize::MAX, z:isize::MAX};
    pub const MIN: Point = Point{x:isize::MIN, y:isize::MIN, z:isize::MIN};

    pub const fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    pub const fn abs_diff(&self, other: &Self) -> Self {
        Self {
            x: self.x.abs_diff(other.x) as isize,
            y: self.y.abs_diff(other.y) as isize,
            z: self.z.abs_diff(other.z) as isize,
        }
    }

    pub const fn manhattan_distance(lhs: &Self, rhs:&Self) -> usize {
        lhs.x.abs_diff(rhs.x) + lhs.y.abs_diff(rhs.y) + lhs.z.abs_diff(rhs.z)
    }
}

impl std::ops::Sub<&Point> for Point {
    type Output = Self;
    fn sub(mut self, rhs: &Point) -> Self::Output {
        self -= rhs;
        self
    }
}
impl std::ops::SubAssign<&Point> for Point {
    fn sub_assign(&mut self, rhs: &Point) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}
impl std::ops::Add<&Point> for Point {
    type Output = Self;
    fn add(mut self, rhs: &Point) -> Self::Output {
        self += rhs;
        self
    }
}
impl std::ops::AddAssign<&Point> for Point {
    fn add_assign(&mut self, rhs: &Point) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
impl std::ops::Mul<isize> for Point {
    type Output = Self;
    fn mul(mut self, rhs: isize) -> Self::Output {
        self *= rhs;
        self
    }
}
impl std::ops::MulAssign<isize> for Point {
    fn mul_assign(&mut self, rhs: isize) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}
impl std::ops::Div<isize> for Point {
    type Output = Self;
    fn div(mut self, rhs: isize) -> Self::Output {
        self /= rhs;
        self
    }
}
impl std::ops::DivAssign<isize> for Point {
    fn div_assign(&mut self, rhs: isize) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PointErrorKind {
    ParseIntError(std::num::ParseIntError),
    WrongSeparator,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsePointError {
    pub kind: PointErrorKind,
}

impl From<std::num::ParseIntError> for ParsePointError {
    fn from(error: std::num::ParseIntError) -> Self {
        Self { kind:PointErrorKind::ParseIntError(error) }
    }
}

impl std::str::FromStr for Point {
    type Err = ParsePointError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(',');
        if let Some(x) = it.next() {
            let x = x.trim();
            let x = x.trim_start_matches("x=");
            if let Some(y) = it.next() {
                let y = y.trim();
                let y = y.trim_start_matches("y=");
                if let Some(z) = it.next() {
                    let z = z.trim();
                    let z = z.trim_start_matches("z=");
                    return Ok(Point::new(x.parse()?,y.parse()?,z.parse()?));
                }
            }
        }
        Err(ParsePointError{kind:PointErrorKind::WrongSeparator})
    }
}


fn main() {
    let args : Vec<_> = env::args().collect();
    let input_path = &args[1];
    let raw_input = fs::read(input_path).unwrap();
    let raw_string = String::from_utf8_lossy(&raw_input);
    let input : Vec<_> = raw_string.lines().collect();

    let points : Vec<Point> = input.iter()
        .map(|line| line.parse().unwrap())
        .collect();
    
    let total_exposed_faces: usize = points.iter().fold(0, |acc, p| {
        let exposed_faces: usize = points.iter().fold(6, |acc, other| {
            if other == p { return acc }
            else {
                let dist = Point::manhattan_distance(p, other);
                if dist == 1 {
                    acc - 1 
                } else {
                    acc
                }
            }
        });
        acc + exposed_faces
    });

    println!("Total exposed faces: {}", total_exposed_faces);
}
