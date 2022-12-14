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
            if let Some(y) = it.next() {
                return Ok(Point::new(x.parse()?,y.parse()?));
            }
        }
        Err(ParsePointError{kind:PointErrorKind::WrongSeparator})
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
    fn contains(&self, rhs: &Segment) -> bool {
        false
    }
}

impl Contains<&Rectangle> for Point {
    fn contains(&self, rhs: &Rectangle) -> bool {
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

impl Intersects<&Rectangle> for Point {
    fn intersects(&self, rhs: &Rectangle) -> bool {
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

impl Contains<&Rectangle> for Segment {
    fn contains(&self, rhs: &Rectangle) -> bool {
        false
    }
}

impl Intersects<&Point> for Segment {
    fn intersects(&self, rhs: &Point) -> bool {
        self.contains(rhs)
    }
}

impl Intersects<&Segment> for Segment {
    fn intersects(&self, rhs: &Segment) -> bool {
        todo!()
    }
}

impl Intersects<&Rectangle> for Segment {
    fn intersects(&self, rhs: &Rectangle) -> bool {
        todo!()
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
}

impl Contains<&Point> for Rectangle {
    fn contains(&self, rhs: &Point) -> bool {
        rhs.x <= isize::max(self.end.x, self.begin.x) &&
        rhs.x >= isize::min(self.end.x, self.begin.x) &&
        rhs.y <= isize::max(self.end.x, self.begin.x) &&
        rhs.y >= isize::min(self.end.x, self.begin.x)
    }
}
impl Contains<&Segment> for Rectangle {
    fn contains(&self, rhs: &Segment) -> bool {
        self.contains(&rhs.begin) && self.contains(&rhs.end)
    }
}
impl Contains<&Rectangle> for Rectangle {
    fn contains(&self, rhs: &Rectangle) -> bool {
        self.contains(&rhs.begin) && self.contains(&rhs.end)
    }
}
impl Intersects<&Point> for Rectangle {
    fn intersects(&self, rhs: &Point) -> bool {
        self.contains(rhs)
    }
}
impl Intersects<&Segment> for Rectangle {
    fn intersects(&self, rhs: &Segment) -> bool {
        todo!()
    }
}
impl Intersects<&Rectangle> for Rectangle {
    fn intersects(&self, rhs: &Rectangle) -> bool {
        todo!()
    }
}




pub enum QuadTreeCell<T> {
    Cell(Vec<T>),
    DividedCell(Vec<QuadTreeCell<T>>),
}

pub struct QuadTree<T> {
    pub cell: QuadTreeCell<T>
}
