use std::{env, fs};

pub trait Contains<Rhs=Self> {
    fn contains(&self, rhs: Rhs) -> bool;
}

pub trait Intersects<Rhs=Self> {
    fn intersects(&self, rhs: Rhs) -> bool;
}

pub trait Move {
    fn r#move_n(&mut self, direction: Direction, n: isize);

    // Auto implemented
    fn r#move(&mut self, direction: Direction) {
        self.move_n(direction, 1);
    }
}

pub trait BoundingBox {
    fn bounding_box(&self) -> Rectangle;
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
pub enum PointDirection {
    Collinear,
    AntiClockwise,
    Clockwise,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Segment {
    pub begin: Point,
    pub end: Point,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rectangle {
    pub begin: Point,
    pub end: Point,
}

impl Point {
    pub const MAX: Point = Point{x:isize::MAX, y:isize::MAX};
    pub const MIN: Point = Point{x:isize::MIN, y:isize::MIN};
    pub const ORIGIN: Point = Point{x:0, y:0};

    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub const fn left(&self) -> Self {
        self.left_n(1)
    }
    pub const fn right(&self) -> Self {
        self.right_n(1)
    }
    pub const fn up(&self) -> Self {
        self.up_n(1)
    }
    pub const fn down(&self) -> Self {
        self.down_n(1)
    }
    pub const fn down_left(&self) -> Self {
        self.down_left_n(1)
    }
    pub const fn down_right(&self) -> Self {
        self.down_right_n(1)
    }
    pub const fn up_left(&self) -> Self {
        self.up_left_n(1)
    }
    pub const fn up_right(&self) -> Self {
        self.up_right_n(1)
    }

    pub const fn left_n(&self, n: isize) -> Self {
        Self { x:self.x-n, y:self.y }
    }
    pub const fn right_n(&self, n: isize) -> Self {
        Self { x:self.x+n, y:self.y }
    }
    pub const fn up_n(&self, n: isize) -> Self {
        Self { x:self.x, y:self.y-n }
    }
    pub const fn down_n(&self, n: isize) -> Self {
        Self { x:self.x, y:self.y+n }
    }
    pub const fn down_left_n(&self, n: isize) -> Self {
        Self { x:self.x-n, y:self.y+n }
    }
    pub const fn down_right_n(&self, n: isize) -> Self {
        Self { x:self.x+n, y:self.y+n }
    }
    pub const fn up_left_n(&self, n: isize) -> Self {
        Self { x:self.x-n, y:self.y-n }
    }
    pub const fn up_right_n(&self, n: isize) -> Self {
        Self { x:self.x+n, y:self.y-n }
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

    pub const fn point_direction(p1: &Point, p2: &Point, p3: &Point) -> PointDirection {
        let val = (p2.y-p1.y)*(p3.x-p2.x)-(p2.x-p1.x)*(p3.y-p2.y);
        if val == 0 {
            return PointDirection::Collinear;
        } else if val < 0 {
            return PointDirection::AntiClockwise;
        } else {
            return PointDirection::Clockwise;
        }
    }
}

impl std::ops::Sub<Point> for Point {
    type Output = Self;
    fn sub(mut self, rhs: Point) -> Self::Output {
        self -= rhs;
        self
    }
}
impl std::ops::SubAssign<Point> for Point {
    fn sub_assign(&mut self, rhs: Point) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
impl std::ops::Add<Point> for Point {
    type Output = Self;
    fn add(mut self, rhs: Point) -> Self::Output {
        self += rhs;
        self
    }
}
impl std::ops::AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
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

impl Move for Point {
    fn r#move_n(&mut self, direction: Direction, n: isize) {
        *self = match direction {
            Direction::Left => self.left_n(n),
            Direction::Right => self.right_n(n),
            Direction::Up => self.up_n(n),
            Direction::Down => self.down_n(n),
            Direction::DownLeft => self.down_left_n(n),
            Direction::DownRight => self.down_right_n(n),
            Direction::UpLeft => self.up_left_n(n),
            Direction::UpRight => self.up_right_n(n),
        }
    }
}

impl BoundingBox for Point {
    fn bounding_box(&self) -> Rectangle {
        Rectangle::new(self.clone(), self.clone())
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

impl Move for Segment {
    fn r#move_n(&mut self, direction: Direction, n: isize) {
        self.begin.r#move_n(direction, n);
        self.end.r#move_n(direction, n);
    }
}

impl BoundingBox for Segment {
    fn bounding_box(&self) -> Rectangle {
        Rectangle::new(self.begin.clone(), self.end.clone())
    }
}

impl Contains<&Point> for Segment {
    fn contains(&self, rhs: &Point) -> bool {
        if self.horizontal() && rhs.x >= isize::min(self.begin.x, self.end.x) && rhs.x <= isize::max(self.begin.x, self.end.x) { return true }
        else if self.vertical() && rhs.y >= isize::min(self.begin.y, self.end.y) && rhs.y <= isize::max(self.begin.y, self.end.y) { return true }
        else if !self.vertical() && ! self.horizontal() { 
            println!("{:?}", self);
            todo!()
        }
        else {
            return false;
        }
    }
}

impl Contains<&Segment> for Segment {
    fn contains(&self, rhs: &Segment) -> bool {
        self.contains(&rhs.begin) && self.contains(&rhs.end)
    }
}

impl Contains<&Rectangle> for Segment {
    fn contains(&self, rhs: &Rectangle) -> bool {
        self.contains(&rhs.begin) && self.contains(&rhs.end)
    }
}

impl Intersects<&Point> for Segment {
    fn intersects(&self, rhs: &Point) -> bool {
        self.contains(rhs)
    }
}

impl Intersects<&Segment> for Segment {
    fn intersects(&self, rhs: &Segment) -> bool {
        let dir1 = Point::point_direction(&self.begin, &self.end, &rhs.begin);
        let dir2 = Point::point_direction(&self.begin, &self.end, &rhs.end);
        let dir3 = Point::point_direction(&rhs.begin, &rhs.end, &self.begin);
        let dir4 = Point::point_direction(&rhs.begin, &rhs.end, &self.end);

        if dir1 != dir2 && dir3 != dir4 {
            return true
        }
        if dir1 == PointDirection::Collinear && self.contains(&rhs.begin) {
            return true
        }
        if dir2 == PointDirection::Collinear && self.contains(&rhs.end) {
            return true
        }
        if dir3 == PointDirection::Collinear && rhs.contains(&self.begin) {
            return true
        }
        if dir4 == PointDirection::Collinear && rhs.contains(&self.end) {
            return true
        }
        return false
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

impl Rectangle {
    pub const fn new(begin: Point, end: Point) -> Self {
        Self { begin, end }
    }

    pub const fn is_square(&self) -> bool {
        let point = self.begin.abs_diff(&self.end);
        point.x == point.y
    }

    pub fn height(&self) -> isize {
        self.upper_left_corner().abs_diff(&self.lower_left_corner()).y
    }

    pub fn width(&self) -> isize {
        self.upper_left_corner().abs_diff(&self.upper_right_corner()).x
    }

    pub fn upper_left_corner(&self) -> Point {
        Point::new(isize::min(self.end.x, self.begin.x), isize::min(self.end.y, self.begin.y))
    }

    pub fn upper_right_corner(&self) -> Point {
        Point::new(isize::max(self.end.x, self.begin.x), isize::min(self.end.y, self.begin.y))
    }

    pub fn lower_left_corner(&self) -> Point {
        Point::new(isize::min(self.end.x, self.begin.x), isize::max(self.end.y, self.begin.y))
    }

    pub fn lower_right_corner(&self) -> Point {
        Point::new(isize::max(self.end.x, self.begin.x), isize::max(self.end.y, self.begin.y))
    }

    pub fn enclosing_area(lhs: &dyn BoundingBox, rhs: &dyn BoundingBox) -> Rectangle {
        let lhs = lhs.bounding_box();
        let rhs = rhs.bounding_box();
        let upper_left_corner = Point::new(
            isize::min(lhs.upper_left_corner().x, rhs.upper_left_corner().x),
            isize::min(lhs.upper_left_corner().y, rhs.upper_left_corner().y),
        );
        let lower_right_corner = Point::new(
            isize::max(lhs.lower_right_corner().x, rhs.lower_right_corner().x),
            isize::max(lhs.lower_right_corner().y, rhs.lower_right_corner().y),
        );
        Rectangle::new(upper_left_corner, lower_right_corner)
    }
}

impl Move for Rectangle {
    fn r#move_n(&mut self, direction: Direction, n: isize) {
        self.begin.r#move_n(direction, n);
        self.end.r#move_n(direction, n);
    }
}

impl BoundingBox for Rectangle {
    fn bounding_box(&self) -> Rectangle {
        self.clone()
    }
}

impl Contains<Point> for Rectangle {
    fn contains(&self, rhs: Point) -> bool {
        rhs.x <= isize::max(self.end.x, self.begin.x) &&
        rhs.x >= isize::min(self.end.x, self.begin.x) &&
        rhs.y <= isize::max(self.end.y, self.begin.y) &&
        rhs.y >= isize::min(self.end.y, self.begin.y)
    }
}
impl Contains<&Segment> for Rectangle {
    fn contains(&self, rhs: &Segment) -> bool {
        self.contains(rhs.begin) && self.contains(rhs.end)
    }
}
impl Contains<&Rectangle> for Rectangle {
    fn contains(&self, rhs: &Rectangle) -> bool {
        self.contains(rhs.begin) && self.contains(rhs.end)
    }
}
impl Intersects<Point> for Rectangle {
    fn intersects(&self, rhs: Point) -> bool {
        self.contains(rhs)
    }
}
impl Intersects<&Segment> for Rectangle {
    fn intersects(&self, _rhs: &Segment) -> bool {
        todo!()
    }
}
impl Intersects<&Rectangle> for Rectangle {
    fn intersects(&self, _rhs: &Rectangle) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Segment, Point, Intersects};
    
    #[test]
    fn segment_intersects() {
        let s1 = Segment::new(Point::new(-1,0), Point::new(1,0));
        let s2 = Segment::new(Point::new(0,-1), Point::new(0,1));
        assert_eq!(s1.intersects(&s2), true);
        assert_eq!(s2.intersects(&s1), true);
    
        let s1 = Segment::new(Point::new(-1,0), Point::new(1,0));
        let s2 = Segment::new(Point::new(-1,0), Point::new(-1,1));
        assert_eq!(s1.intersects(&s2), true);
        assert_eq!(s2.intersects(&s1), true);
        
        let s1 = Segment::new(Point::new(-1,0), Point::new(1,0));
        let s2 = Segment::new(Point::new(-2,0), Point::new(-2,1));
        assert_eq!(s1.intersects(&s2), false);
        assert_eq!(s2.intersects(&s1), false);
    }

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
        
        let s1 = Segment::new(Point::new(0,0), Point::new(0,2));
        let s2 = Segment::new(Point::new(0,3), Point::new(0,6));
        let merged1 = s1.merge_with(&s2);
        let merged2 = s2.merge_with(&s1);
        assert_eq!(merged1, None);
        assert_eq!(merged2, merged1);
    }
}






pub enum QuadTreeCell<T> {
    Cell(Vec<T>),
    Div(Vec<QuadTree<T>>),
}

pub struct QuadTree<T> {
    cell: QuadTreeCell<T>,      // Cell of the quadtree
    pivot: Point,               // Point which divides the cells
    threshold: usize,           // Max number of items in a cell
    area: Rectangle,            // Area covered by all items, used to calculate pivot when splitting the area
    nb_items: usize,            // Number of items in the quadtree
}

impl<T> QuadTree<T>
where T: BoundingBox + Clone
{
    pub fn new() -> Self {
        Self {
            cell: QuadTreeCell::<T>::Cell(Vec::<T>::new()),
            pivot: Point::ORIGIN,
            threshold: 20,
            area: Rectangle::new(Point::ORIGIN, Point::ORIGIN),
            nb_items: 0,
        }
    }

    pub fn builder() -> QuadTreeBuilder<T> {
        QuadTreeBuilder::<T>::new()
    }

    pub fn len(&self) -> usize {
        self.nb_items
    }

    pub fn push(&mut self, item: T) {
        self.area = Rectangle::enclosing_area(&self.area, &item);
        self.nb_items += 1;
        // Insert in the quadtree
        match &mut self.cell {
            // Insert in the cell
            QuadTreeCell::Cell(cell) => {
                cell.push(item);
                self.optimize();
            },
            // Insert in the cells where the data belongs
            QuadTreeCell::Div(divisions) => {
                let bounding_box = item.bounding_box();
                let upper_left_corner = bounding_box.upper_left_corner();
                let upper_right_corner = bounding_box.upper_right_corner();
                let lower_left_corner = bounding_box.lower_left_corner();
                let lower_right_corner = bounding_box.lower_right_corner();
                if upper_left_corner.x <= self.pivot.x && upper_left_corner.y <= self.pivot.y {
                    divisions[0].push(item.clone());
                }
                if upper_right_corner.x > self.pivot.x && upper_right_corner.y <= self.pivot.y {
                    divisions[1].push(item.clone());
                }
                if lower_left_corner.x <= self.pivot.x && lower_left_corner.y > self.pivot.y {
                    divisions[2].push(item.clone());
                }
                if lower_right_corner.x > self.pivot.x && lower_right_corner.y > self.pivot.y {
                    divisions[3].push(item.clone());
                }
            },
        }
    }

    pub fn nearby_iter<'a>(&'a self, item: &dyn BoundingBox) -> NearbyIterator<'a, T> {
        NearbyIterator::<'a, T>::new(self.nearby_items(item))
    }

    fn nearby_items<'a>(&'a self, item: &dyn BoundingBox) -> Option<Box<dyn Iterator<Item=&T> + 'a>> {
        let bounding_box = item.bounding_box();
        match &self.cell {
            QuadTreeCell::Cell(data) => {
                return Some(Box::new(data.iter()));
            },
            QuadTreeCell::Div(divisions) => {
                let mut iterator : Option<Box<dyn Iterator<Item=&T> + 'a>> = None;
                let upper_left_corner = bounding_box.upper_left_corner();
                let upper_right_corner = bounding_box.upper_right_corner();
                let lower_left_corner = bounding_box.lower_left_corner();
                let lower_right_corner = bounding_box.lower_right_corner();
                if upper_left_corner.x <= self.pivot.x && upper_left_corner.y <= self.pivot.y {
                    iterator = divisions[0].nearby_items(item);
                }
                if upper_right_corner.x > self.pivot.x && upper_right_corner.y <= self.pivot.y {
                    let iterator1 = divisions[1].nearby_items(item).unwrap();
                    if let Some(it) = iterator {
                        iterator = Some(Box::new(it.chain(iterator1)));
                    } else {
                        iterator = Some(iterator1);
                    }
                }
                if lower_left_corner.x <= self.pivot.x && lower_left_corner.y > self.pivot.y {
                    let iterator2 = divisions[2].nearby_items(item).unwrap();
                    if let Some(it) = iterator {
                        iterator = Some(Box::new(it.chain(iterator2)));
                    } else {
                        iterator = Some(iterator2);
                    }
                }
                if lower_right_corner.x > self.pivot.x && lower_right_corner.y > self.pivot.y {
                    let iterator3 = divisions[3].nearby_items(item).unwrap();
                    if let Some(it) = iterator {
                        iterator = Some(Box::new(it.chain(iterator3)));
                    } else {
                        iterator = Some(iterator3);
                    }
                }
                return iterator;
            },
        }
    }

    fn optimize(&mut self) {
        let all_data: Vec<T>;
        let mut divisions = Vec::<QuadTree<T>>::new();
        if let QuadTreeCell::Cell(data) = &self.cell {
            if data.len() > self.threshold {
                all_data = data.clone();
                let upper_left_corner = self.area.upper_left_corner();
                let upper_right_corner = self.area.upper_right_corner();
                let lower_left_corner = self.area.lower_left_corner();
                let lower_right_corner = self.area.lower_right_corner();
                let pivot0 = (upper_left_corner - self.pivot) / 2;
                let pivot1 = (upper_right_corner - self.pivot) / 2;
                let pivot2 = (lower_left_corner - self.pivot) / 2;
                let pivot3 = (lower_right_corner - self.pivot) / 2;
                divisions.push(QuadTree::<T>::builder()
                    .with_threshold(self.threshold)
                    .with_pivot(pivot0)
                    .finish()
                );
                divisions.push(QuadTree::<T>::builder()
                    .with_threshold(self.threshold)
                    .with_pivot(pivot1)
                    .finish()
                );
                divisions.push(QuadTree::<T>::builder()
                    .with_threshold(self.threshold)
                    .with_pivot(pivot2)
                    .finish()
                );
                divisions.push(QuadTree::<T>::builder()
                    .with_threshold(self.threshold)
                    .with_pivot(pivot3)
                    .finish()
                );
            } else {
                return;
            }
        }
        else {
            return;
        }
        // Now push data to the new divided tree
        self.cell = QuadTreeCell::Div(divisions);
        for data in all_data.into_iter() {
            self.push(data);
        }
    }

}

pub struct QuadTreeBuilder<T> {
    tree: QuadTree<T>,
}

impl<T> QuadTreeBuilder<T>
where T: BoundingBox + Clone
{
    pub fn new() -> Self {
        Self { tree: QuadTree::<T>::new() }
    }

    pub fn with_threshold(mut self, threshold: usize) -> Self {
        self.tree.threshold = threshold;
        self
    }

    pub fn with_pivot(mut self, pivot: Point) -> Self {
        self.tree.pivot = pivot;
        self
    }

    pub fn finish(self) -> QuadTree<T> {
        self.tree
    }
}

pub struct NearbyIterator<'a, T> {
    iterator: Option<Box<dyn Iterator<Item=&'a T> + 'a>>,
}

impl<'a, T> NearbyIterator<'a, T> {
    pub fn new(iterator: Option<Box<dyn Iterator<Item=&'a T> + 'a>>) -> Self {
        Self {
            iterator,
        }
    }
}

impl<'a, T> Iterator for NearbyIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(it) = &mut self.iterator {
            return it.next();
        } else {
            return None;
        }
    }
}












pub enum DivTreeCell<T> {
    Cell(Vec<T>),
    Div(Vec<DivTree<T>>),
}

pub struct DivTree<T> {
    cell: DivTreeCell<T>,       // Cell of the quadtree
    pivot: Point,               // Point which divides the cells
    parent_pivot: Point,        // Pivot of the parent tree
    threshold: usize,           // Max number of items in a cell
    area: Rectangle,            // Area covered by all items, used to calculate pivot when splitting the area
    nb_items: usize,            // Number of items in the quadtree
    minimum_area_size: usize,   // Minimum area size (cannot subdivide to a smaller area)
}

impl<T> DivTree<T>
where T: BoundingBox + Clone
{
    pub fn new() -> Self {
        Self {
            cell: DivTreeCell::<T>::Cell(Vec::<T>::new()),
            pivot: Point::ORIGIN,
            parent_pivot: Point::ORIGIN,
            threshold: 20,
            minimum_area_size: 20,
            area: Rectangle::new(Point::ORIGIN, Point::ORIGIN),
            nb_items: 0,
        }
    }

    pub fn builder() -> DivTreeBuilder<T> {
        DivTreeBuilder::<T>::new()
    }

    pub fn len(&self) -> usize {
        self.nb_items
    }

    pub fn push(&mut self, item: T) {
        // adjust the area
        if self.pivot == self.parent_pivot {
            // print!("New area enclosing item: {:?}", item.bounding_box());
            self.area = Rectangle::enclosing_area(&self.area, &item);
            // println!(" => {:?}", self.area);
        } else {
            let bounding_box = item.bounding_box();
            println!("Extending current area: {:?}", self.area);
            println!("    item bounding box: {:?}", bounding_box);
            self.area = Rectangle::enclosing_area(&self.area, &bounding_box);
            println!("    enclosing area: {:?}", self.area);
            let diff = self.pivot - self.parent_pivot;
            println!("    pivot diff: {:?}", diff);
            if diff.y <= 0 {
                let min_y = self.parent_pivot.y;
                if self.area.begin.y > min_y    { self.area.begin.y = min_y; }
                if self.area.end.y > min_y      { self.area.end.y = min_y; }
            }
            else {
                let max_y = self.parent_pivot.y;
                if self.area.begin.y < max_y    { self.area.begin.y = max_y; }
                if self.area.end.y < max_y      { self.area.end.y = max_y; }
            }
            println!("    new area: {:?}", self.area);
        }
        self.nb_items += 1;
        // Insert in the quadtree
        match &mut self.cell {
            // Insert in the cell
            DivTreeCell::Cell(cell) => {
                cell.push(item);
                self.optimize();
            },
            // Insert in the cells where the data belongs
            DivTreeCell::Div(divisions) => {
                let bounding_box = item.bounding_box();
                let upper_left_corner = bounding_box.upper_left_corner();
                let lower_right_corner = bounding_box.lower_right_corner();
                if upper_left_corner.y <= self.pivot.y {
                    divisions[0].push(item.clone());
                }
                if lower_right_corner.y > self.pivot.y {
                    divisions[1].push(item.clone());
                }
            },
        }
    }

    pub fn nearby_iter<'a>(&'a self, item: &dyn BoundingBox) -> NearbyIterator<'a, T> {
        NearbyIterator::<'a, T>::new(self.nearby_items(item))
    }

    fn nearby_items<'a>(&'a self, item: &dyn BoundingBox) -> Option<Box<dyn Iterator<Item=&T> + 'a>> {
        let bounding_box = item.bounding_box();
        match &self.cell {
            DivTreeCell::Cell(data) => {
                return Some(Box::new(data.iter()));
            },
            DivTreeCell::Div(divisions) => {
                let mut iterator : Option<Box<dyn Iterator<Item=&T> + 'a>> = None;
                let upper_left_corner = bounding_box.upper_left_corner();
                let lower_right_corner = bounding_box.lower_right_corner();
                if upper_left_corner.y <= self.pivot.y {
                    iterator = divisions[0].nearby_items(item);
                }
                if lower_right_corner.y > self.pivot.y {
                    let iterator1 = divisions[1].nearby_items(item).unwrap();
                    if let Some(it) = iterator {
                        iterator = Some(Box::new(it.chain(iterator1)));
                    } else {
                        iterator = Some(iterator1);
                    }
                }
                return iterator;
            },
        }
    }

    fn optimize(&mut self) {
        let all_data: Vec<T>;
        let mut divisions = Vec::<DivTree<T>>::new();
        if let DivTreeCell::Cell(data) = &self.cell {
            if data.len() > self.threshold {
                println!("Optimizing DivTree with {} items", data.len());
                println!("Current area: {:?}", self.area);
                let lower_left_corner = self.area.lower_left_corner();
                let upper_right_corner = self.area.upper_right_corner();
                // Calculating a new pivot in the middle of the current area
                let next_pivot = lower_left_corner + (upper_right_corner - lower_left_corner) / 2;
                println!("Next pivot: {:?}", next_pivot);
                // Calculating temporary pivot for 1st child cell
                let pivot0 = next_pivot + (upper_right_corner - next_pivot) / 2;
                let area0 = Rectangle::enclosing_area(
                    &Rectangle::new(self.area.upper_left_corner(), self.area.upper_right_corner()),
                    &next_pivot
                );
                println!("1st child pivot: {:?}", pivot0);
                println!("1st child area: {:?}", area0);
                let pivot1 = next_pivot - (upper_right_corner - next_pivot) / 2;
                let area1 = Rectangle::enclosing_area(
                    &Rectangle::new(self.area.lower_left_corner(), self.area.lower_right_corner()),
                    &next_pivot
                );
                println!("2nd child pivot: {:?}", pivot1);
                println!("2nd child area: {:?}", area1);
                if area0.height() <= self.minimum_area_size as isize ||
                    area1.height() <= self.minimum_area_size as isize
                {
                    println!("Aborting optimization because child areas are too small");
                    return;
                }
                self.pivot = next_pivot;
                all_data = data.clone();
                divisions.push(DivTree::<T>::builder()
                    .with_threshold(self.threshold)
                    .with_parent_pivot(self.pivot)
                    .with_minimum_area_size(self.minimum_area_size)
                    .with_pivot(pivot0)
                    .with_area(area0)
                    .finish()
                );
                divisions.push(DivTree::<T>::builder()
                    .with_threshold(self.threshold)
                    .with_parent_pivot(self.pivot)
                    .with_minimum_area_size(self.minimum_area_size)
                    .with_pivot(pivot1)
                    .with_area(area1)
                    .finish()
                );
            } else {
                return;
            }
        }
        else {
            return;
        }
        // Now push data to the new divided tree
        self.nb_items = 0;
        self.cell = DivTreeCell::Div(divisions);
        for data in all_data.into_iter() {
            self.push(data);
        }
    }

}

pub struct DivTreeBuilder<T> {
    tree: DivTree<T>,
}

impl<T> DivTreeBuilder<T>
where T: BoundingBox + Clone
{
    pub fn new() -> Self {
        Self { tree: DivTree::<T>::new() }
    }

    pub fn with_threshold(mut self, threshold: usize) -> Self {
        self.tree.threshold = threshold;
        self
    }

    pub fn with_pivot(mut self, pivot: Point) -> Self {
        self.tree.pivot = pivot;
        self
    }

    pub fn with_parent_pivot(mut self, parent_pivot: Point) -> Self {
        self.tree.parent_pivot = parent_pivot;
        self
    }

    pub fn with_area(mut self, area: Rectangle) -> Self {
        self.tree.area = area;
        self
    }

    pub fn with_minimum_area_size(mut self, minimum_area_size: usize) -> Self {
        self.tree.minimum_area_size = minimum_area_size;
        self
    }

    pub fn finish(self) -> DivTree<T> {
        self.tree
    }
}







#[derive(Debug, Clone)]
pub struct Rock {
    segments: Vec<Segment>,
}

impl Rock {
    pub const fn new(segments: Vec<Segment>) -> Self {
        Self { segments }
    }

    pub fn points<'a>(&'a self) -> RockPointIterator<'a> {
        RockPointIterator::new(self)
    }
}

pub struct RockPointIterator<'a> {
    rock: &'a Rock,
    count: usize,
}

impl<'a> RockPointIterator<'a> {
    pub fn new(rock: &'a Rock) -> Self {
        Self { rock, count: 0 }
    }
}

impl<'a> Iterator for RockPointIterator<'a> {
    type Item = &'a Point;

    fn next(&mut self) -> Option<Self::Item> {
        let mut n: usize = 0;
        for segment in self.rock.segments.iter() {
            if n == self.count {
                self.count += 1;
                return Some(&segment.begin);
            }
            if n+1 == self.count {
                self.count += 1;
                return Some(&segment.end);
            }
            n += 2;
        }
        return None
    }
}

impl Move for Rock {
    fn r#move_n(&mut self, direction: Direction, n: isize) {
        for segment in self.segments.iter_mut() {
            segment.r#move_n(direction, n);
        }
    }
}

impl Intersects<&Rock> for Rock {
    fn intersects(&self, rhs: &Rock) -> bool {
        self.segments.iter().any(|s| {
            rhs.segments.iter().any(|o| s.intersects(o))
        })
    }
}

impl BoundingBox for Rock {
    fn bounding_box(&self) -> Rectangle {
        let first_segment = self.segments.first().unwrap();
        let mut bounding_box = first_segment.bounding_box();
        for segment in self.segments.iter().skip(1) {
            bounding_box = Rectangle::enclosing_area(&bounding_box, segment);
        }
        bounding_box
    }
}

const NB_ROCKS: usize = 25;
const RIGHT_SHIFT: isize = 2;
const UP_SHIFT: isize = 3;
const CAVE_WIDTH: isize = 7;

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
        Rock::new(vec![
            Segment::new(Point::new(0,0), Point::new(3,0))
        ]),
        Rock::new(vec![
            Segment::new(Point::new(0,-1), Point::new(2,-1)),
            Segment::new(Point::new(1,-2), Point::new(1,0))
        ]),
        Rock::new(vec![
            Segment::new(Point::new(0,0), Point::new(2,0)),
            Segment::new(Point::new(2,-2), Point::new(2,0))
        ]),
        Rock::new(vec![
            Segment::new(Point::new(0,-3), Point::new(0,0))
        ]),
        Rock::new(vec![
            Segment::new(Point::new(0,-1), Point::new(1,-1)),
            Segment::new(Point::new(0,0), Point::new(1,0)),
        ])
    ];
    let rocks_generator = rocks.iter().cycle();



    // Lets run the simulation
    let mut max_height: isize = 0;
    let mut fallen_rocks = DivTree::<Rock>::new();
    for rock in rocks_generator.take(NB_ROCKS) {
        // Spawn a new rock
        let mut rock = rock.clone();
        // println!("Spawned rock: {:?}", rock);
        rock.move_n(Direction::Right, RIGHT_SHIFT);
        rock.move_n(Direction::Up, UP_SHIFT+max_height+1);
        // println!("Falling rock: {:?}", rock);

        // Simulate the fall
        loop {
            // println!("Falling rock: {:?}", rock);
            // Move in the jet direction
            let jet_direction = gas_jets.next().unwrap();
            rock.r#move(jet_direction);
            // Check if new position collides with anything (cave or other rocks)
            if rock.points().any(|p| p.x < 0 || p.x >= CAVE_WIDTH)
                || fallen_rocks.nearby_iter(&rock).any(|fallen_rock| rock.intersects(fallen_rock))
            {
                // Undo the move if a collision happen
                rock.r#move(Direction::opposite(jet_direction));
            }
            // println!("Falling rock: {:?}", rock);
            // Move down
            rock.r#move(Direction::Down);
            // Check if new position collides with anything (cave or other rocks)
            if rock.points().any(|p| p.y >= 0)
                || fallen_rocks.nearby_iter(&rock).any(|fallen_rock| rock.intersects(fallen_rock))
            {
                // Undo the mode
                rock.r#move(Direction::Up);
                // Settle the rock here
                fallen_rocks.push(rock.clone());
                // Compute the new max_height
                max_height = isize::max(max_height, rock.points().reduce(|acc, p| if acc.y < p.y { acc } else { p }).unwrap().y*-1);
                // Break the loop
                break;
            }
        }
        let comparisons = fallen_rocks.nearby_iter(&rock).count();
        println!("Fallen rocks: {}, comparisons: {}, max_height: {}", fallen_rocks.len(), comparisons, max_height);
        println!("");
    }

    println!("Max height: {}", max_height);
}
