use std::{env, fs};
use std::collections::VecDeque;

pub trait Contains<Rhs=Self> {
    fn contains(&self, rhs: Rhs) -> bool;
}

pub trait Intersects<Rhs=Self> {
    fn intersects(&self, rhs: Rhs) -> bool;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Segment {
    pub begin: Point,
    pub end: Point,
}

impl Point {
    pub const MAX: Point = Point{x:isize::MAX, y:isize::MAX};
    pub const MIN: Point = Point{x:isize::MIN, y:isize::MIN};

    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub const fn left(&self) -> Self {
        Self { x:self.x-1, y:self.y }
    }
    pub const fn right(&self) -> Self {
        Self { x:self.x+1, y:self.y }
    }
    pub const fn up(&self) -> Self {
        Self { x:self.x, y:self.y-1 }
    }
    pub const fn down(&self) -> Self {
        Self { x:self.x, y:self.y+1 }
    }
    pub const fn down_left(&self) -> Self {
        Self { x:self.x-1, y:self.y+1 }
    }
    pub const fn down_right(&self) -> Self {
        Self { x:self.x+1, y:self.y+1 }
    }
    pub const fn up_left(&self) -> Self {
        Self { x:self.x-1, y:self.y-1 }
    }
    pub const fn up_right(&self) -> Self {
        Self { x:self.x+1, y:self.y-1 }
    }

    pub const fn is_left_of(&self, other: &Self) -> bool {
        self.y == other.y && other.x > self.x
    }
    pub const fn is_right_of(&self, other: &Self) -> bool {
        self.y == other.y && other.x < self.x
    }
    pub const fn is_up_of(&self, other: &Self) -> bool {
        self.x == other.x && other.y > self.y
    }
    pub const fn is_down_of(&self, other: &Self) -> bool {
        self.x == other.x && other.y < self.y
    }

    pub const fn abs_diff(&self, other: &Self) -> Self {
        Self { x: self.x.abs_diff(other.x) as isize, y: self.y.abs_diff(other.y) as isize }
    }

    pub const fn manhattan_distance(lhs: &Self, rhs:&Self) -> usize {
        lhs.x.abs_diff(rhs.x) + lhs.y.abs_diff(rhs.y)
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
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

impl Contains<&Point> for Point {
    fn contains(&self, rhs: &Point) -> bool {
        self == rhs
    }
}

impl Contains<&Segment> for Point {
    fn contains(&self, _rhs: &Segment) -> bool {
        false
    }
}

impl Intersects<&Point> for Point {
    fn intersects(&self, rhs: &Point) -> bool {
        self == rhs
    }
}

impl Intersects<&Segment> for Point {
    fn intersects(&self, rhs: &Segment) -> bool {
        rhs.intersects(self)
    }
}

impl Segment {
    pub const fn new(begin: Point, end: Point) -> Self {
        Self { begin, end }
    }

    pub const fn horizontal(&self) -> bool {
        self.begin.y == self.end.y
    }
    pub const fn vertical(&self) -> bool {
        self.begin.x == self.end.x
    }
    
    pub const fn orientation(&self) -> Option<Orientation> {
        if self.horizontal() { Some(Orientation::Horizontal) }
        else if self.vertical() { Some(Orientation::Vertical) }
        else { None }
    }

    pub fn merge_with(&self, rhs: &Self) -> Option<Self> {
        if self.horizontal() && rhs.horizontal() && self.begin.y == rhs.begin.y {
            if (isize::max(self.begin.x, self.end.x) >= isize::min(rhs.begin.x, rhs.end.x)
                && isize::max(self.begin.x, self.end.x) <= isize::max(rhs.begin.x, rhs.end.x))
                || (isize::min(self.begin.x, self.end.x) <= isize::max(rhs.begin.x, rhs.end.x)
                && isize::min(self.begin.x, self.end.x) >= isize::min(rhs.begin.x, rhs.end.x))
            {
                let begin = Point::new(
                    isize::min(self.begin.x, self.end.x).min(rhs.begin.x).min(rhs.end.x),
                    self.begin.y
                );
                let end = Point::new(
                    isize::max(self.begin.x, self.end.x).max(rhs.begin.x).max(rhs.end.x),
                    self.begin.y
                );
                return Some(Segment::new(begin, end));
            }
            else { return None; }
        }
        else if self.vertical() && rhs.vertical() && self.begin.x == rhs.begin.x {
            if (isize::max(self.begin.y, self.end.y) >= isize::min(rhs.begin.y, rhs.end.y)
                && isize::max(self.begin.y, self.end.y) <= isize::max(rhs.begin.y, rhs.end.y))
                || (isize::min(self.begin.y, self.end.y) <= isize::max(rhs.begin.y, rhs.end.y)
                && isize::min(self.begin.y, self.end.y) >= isize::min(rhs.begin.y, rhs.end.y))
            {
                let begin = Point::new(
                    self.begin.x,
                    isize::min(self.begin.y, self.end.y).min(rhs.begin.y).min(rhs.end.y)
                );
                let end = Point::new(
                    self.begin.x,
                    isize::max(self.begin.y, self.end.y).max(rhs.begin.y).max(rhs.end.y)
                );
                return Some(Segment::new(begin, end));
            }
            else { return None; }
        } else {
            return None;
        }
    }
}

impl Contains<&Point> for Segment {
    fn contains(&self, rhs: &Point) -> bool {
        if self.horizontal() && rhs.x >= isize::min(self.begin.x, self.end.x) && rhs.x <= isize::max(self.begin.x, self.end.x) { return true }
        else if self.vertical() && rhs.y >= isize::min(self.begin.y, self.end.y) && rhs.y <= isize::max(self.begin.y, self.end.y) { return true }
        else { return false }
    }
}

impl Contains<&Segment> for Segment {
    fn contains(&self, rhs: &Segment) -> bool {
        self.contains(&rhs.begin) && self.contains(&rhs.end)
    }
}

impl Intersects<&Point> for Segment {
    fn intersects(&self, rhs: &Point) -> bool {
        self.contains(rhs)
    }
}

impl Intersects<&Segment> for Segment {
    fn intersects(&self, _rhs: &Segment) -> bool {
        todo!()
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
                return Ok(Point::new(x.parse()?,y.parse()?));
            }
        }
        Err(ParsePointError{kind:PointErrorKind::WrongSeparator})
    }
}

#[cfg(test)]
mod tests {
    use crate::{Segment, Point};

    #[test]
    fn segment_merge() {
        let s1 = Segment::new(Point::new(0,0), Point::new(2,0));
        let s2 = Segment::new(Point::new(1,0), Point::new(3,0));
        let merged1 = s1.merge_with(&s2);
        let merged2 = s2.merge_with(&s1);
        assert_eq!(merged1, Some(Segment::new(Point::new(0,0), Point::new(3,0))));
        assert_eq!(merged2, merged1);
        
        let s1 = Segment::new(Point::new(2,0), Point::new(0,0));
        let s2 = Segment::new(Point::new(1,0), Point::new(3,0));
        let merged1 = s1.merge_with(&s2);
        let merged2 = s2.merge_with(&s1);
        assert_eq!(merged1, Some(Segment::new(Point::new(0,0), Point::new(3,0))));
        assert_eq!(merged2, merged1);
        
        let s1 = Segment::new(Point::new(0,0), Point::new(2,0));
        let s2 = Segment::new(Point::new(3,0), Point::new(1,0));
        let merged1 = s1.merge_with(&s2);
        let merged2 = s2.merge_with(&s1);
        assert_eq!(merged1, Some(Segment::new(Point::new(0,0), Point::new(3,0))));
        assert_eq!(merged2, merged1);
        
        let s1 = Segment::new(Point::new(2,0), Point::new(0,0));
        let s2 = Segment::new(Point::new(3,0), Point::new(1,0));
        let merged1 = s1.merge_with(&s2);
        let merged2 = s2.merge_with(&s1);
        assert_eq!(merged1, Some(Segment::new(Point::new(0,0), Point::new(3,0))));
        assert_eq!(merged2, merged1);
        
        let s1 = Segment::new(Point::new(0,0), Point::new(0,2));
        let s2 = Segment::new(Point::new(0,1), Point::new(0,3));
        let merged1 = s1.merge_with(&s2);
        let merged2 = s2.merge_with(&s1);
        assert_eq!(merged1, Some(Segment::new(Point::new(0,0), Point::new(0,3))));
        assert_eq!(merged2, merged1);
        
        let s1 = Segment::new(Point::new(0,2), Point::new(0,0));
        let s2 = Segment::new(Point::new(0,1), Point::new(0,3));
        let merged1 = s1.merge_with(&s2);
        let merged2 = s2.merge_with(&s1);
        assert_eq!(merged1, Some(Segment::new(Point::new(0,0), Point::new(0,3))));
        assert_eq!(merged2, merged1);
        
        let s1 = Segment::new(Point::new(0,0), Point::new(0,2));
        let s2 = Segment::new(Point::new(0,3), Point::new(0,1));
        let merged1 = s1.merge_with(&s2);
        let merged2 = s2.merge_with(&s1);
        assert_eq!(merged1, Some(Segment::new(Point::new(0,0), Point::new(0,3))));
        assert_eq!(merged2, merged1);
        
        let s1 = Segment::new(Point::new(0,2), Point::new(0,0));
        let s2 = Segment::new(Point::new(0,3), Point::new(0,1));
        let merged1 = s1.merge_with(&s2);
        let merged2 = s2.merge_with(&s1);
        assert_eq!(merged1, Some(Segment::new(Point::new(0,0), Point::new(0,3))));
        assert_eq!(merged2, merged1);
        
        let s1 = Segment::new(Point::new(0,0), Point::new(0,1));
        let s2 = Segment::new(Point::new(0,1), Point::new(0,2));
        let merged1 = s1.merge_with(&s2);
        let merged2 = s2.merge_with(&s1);
        assert_eq!(merged1, Some(Segment::new(Point::new(0,0), Point::new(0,2))));
        assert_eq!(merged2, merged1);
        
        let s1 = Segment::new(Point::new(0,1), Point::new(0,0));
        let s2 = Segment::new(Point::new(0,1), Point::new(0,2));
        let merged1 = s1.merge_with(&s2);
        let merged2 = s2.merge_with(&s1);
        assert_eq!(merged1, Some(Segment::new(Point::new(0,0), Point::new(0,2))));
        assert_eq!(merged2, merged1);
        
        let s1 = Segment::new(Point::new(0,0), Point::new(0,1));
        let s2 = Segment::new(Point::new(0,2), Point::new(0,1));
        let merged1 = s1.merge_with(&s2);
        let merged2 = s2.merge_with(&s1);
        assert_eq!(merged1, Some(Segment::new(Point::new(0,0), Point::new(0,2))));
        assert_eq!(merged2, merged1);
        
        let s1 = Segment::new(Point::new(0,1), Point::new(0,0));
        let s2 = Segment::new(Point::new(0,2), Point::new(0,1));
        let merged1 = s1.merge_with(&s2);
        let merged2 = s2.merge_with(&s1);
        assert_eq!(merged1, Some(Segment::new(Point::new(0,0), Point::new(0,2))));
        assert_eq!(merged2, merged1);
    }
}

const TARGET_Y: isize = 10;

fn main() {
    let args : Vec<_> = env::args().collect();
    let input_path = &args[1];
    let raw_input = fs::read(input_path).unwrap();
    let raw_string = String::from_utf8_lossy(&raw_input);
    let input : Vec<_> = raw_string.lines().collect();

    let mut min = Point::MAX;
    let mut max = Point::MIN;

    let sensors_beacons : Vec<(Point,Point)> = input.iter().map(|line| {
        let mut it = line.split(':');
        if let Some(p) = it.next() {
            let p = p.trim_start_matches("Sensor at");
            let sensor: Point = p.parse().unwrap();
            if sensor.x < min.x { min.x = sensor.x }
            if sensor.y < min.y { min.y = sensor.y }
            if sensor.x > max.x { max.x = sensor.x }
            if sensor.y > max.y { max.y = sensor.y }
            if let Some(b) = it.next() {
                let b = b.trim();
                let b = b.trim_start_matches("closest beacon is at");
                let beacon: Point = b.parse().unwrap();
                if beacon.x < min.x { min.x = beacon.x }
                if beacon.y < min.y { min.y = beacon.y }
                if beacon.x > max.x { max.x = beacon.x }
                if beacon.y > max.y { max.y = beacon.y }
                return (sensor, beacon);
            } else {
                panic!("Incorrect input");
            }   
        } else {
            panic!("Incorrect input");
        }
    }).collect();

    println!("Segments at y={}:", TARGET_Y);
    let mut scan_at_target_y: VecDeque<Segment> = sensors_beacons.iter().filter_map(|(sensor, beacon)| {
        let beacon_distance = Point::manhattan_distance(sensor, beacon);
        let target_distance = sensor.y.abs_diff(TARGET_Y);
        if beacon_distance >= target_distance {
            // let diff_y = sensor.y-TARGET_Y;
            let diff_x = beacon_distance as isize - target_distance as isize ;
            let begin = Point::new(sensor.x-diff_x, TARGET_Y);
            let end = Point::new(sensor.x+diff_x, TARGET_Y);
            let segment = Segment::new(begin, end);
            println!("Sensor: {:?} - Beacon: {:?} => {:?}", sensor, beacon, segment);
            return Some(segment);
        }
        None
    }).collect();

    let mut counter: usize = 0;
    loop {
        if let Some(segment) = scan_at_target_y.pop_front() {
            let mut found = false;
            for other in scan_at_target_y.iter_mut() {
                if let Some(merged) = other.merge_with(&segment) {
                    *other = merged;
                    counter = 0;
                    found = true;
                    println!("merged: {:?}", other);
                    break;
                }
            }
            if found { continue; }
            counter += 1;
            scan_at_target_y.push_back(segment);
            println !("counter: {} - len: {}", counter, scan_at_target_y.len());
            if counter >= scan_at_target_y.len() {
                break;
            }
        } else {
            break;
        }
    }

    println!("Disjoint segments:");
    let result: usize = scan_at_target_y.iter()
        .map(|segment| {
            println!("{:?}", segment);
            Point::manhattan_distance(&segment.begin, &segment.end)
        })
        .sum();

    println!("Result: {}", result);
}
