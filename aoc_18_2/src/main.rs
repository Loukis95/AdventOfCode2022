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
    pub const ORIGIN: Point = Point{x:0, y:0, z:0};

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

    let mut min = Point::MAX;
    let mut max = Point::MIN;

    // Points
    let points : Vec<Point> = input.iter()
        .map(|line| {
            let p: Point = line.parse().unwrap();
            if p.x < min.x { min.x = p.x }
            if p.y < min.y { min.y = p.y }
            if p.z < min.z { min.z = p.z }
            if p.x >= max.x { max.x = p.x+1 }
            if p.y >= max.y { max.y = p.y+1 }
            if p.z >= max.z { max.z = p.z+1 }
            p
        })
        .collect();

    println!("Min: {}", min);
    println!("Max: {}", max);
    
    // Total of exposed faces
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

    // 3D grid
    let mut list_of_neighbours = Vec::<Point>::new();
    let mut grid: Vec<Vec<Vec<u8>>> = vec![vec![vec![0; max.x as usize]; max.y as usize]; max.z as usize];
    for p in points.iter() {
        // Register points in the grid
        grid[p.z as usize][p.y as usize][p.x as usize] = 1;
        // Lambda test
        let mut fn_test_and_push = |neighbour: Point| {
            if neighbour.x >= 0 && neighbour.y >= 0 && neighbour.z >= 0
            && neighbour.x < max.x && neighbour.y < max.y && neighbour.z < max.z
            {
                list_of_neighbours.push(neighbour)
            }
        };
        // 1st neighbour
        let neighbour = Point::new(p.x-1, p.y, p.z);
        fn_test_and_push(neighbour);
        // 2nd neighbour
        let neighbour = Point::new(p.x+1, p.y, p.z);
        fn_test_and_push(neighbour);
        // 3rd neighbour
        let neighbour = Point::new(p.x, p.y-1, p.z);
        fn_test_and_push(neighbour);
        // 4th neighbour
        let neighbour = Point::new(p.x, p.y+1, p.z);
        fn_test_and_push(neighbour);
        // 5th neighbour
        let neighbour = Point::new(p.x, p.y, p.z-1);
        fn_test_and_push(neighbour);
        // 6th neighbour
        let neighbour = Point::new(p.x, p.y, p.z+1);
        fn_test_and_push(neighbour);
    }
    
    let mut neighbour_iterator = list_of_neighbours.iter();
    while let Some(p) = neighbour_iterator.next() {
        // Lambda check if outside the area
        let is_outside = |point: Point| -> bool {
            if point.x < min.x || point.y < min.y || point.z < min.z
            || point.x >= max.x || point.y >= max.y || point.z >= max.z
            || grid[point.z as usize][point.y as usize][point.x as usize] == 2
            {
                true
            }
            else
            {
                false
            }
        };
    
        // Lambda check if is blocked by neighbour
        let is_blocked = |point: Point| -> bool {
            let value = grid[point.z as usize][point.y as usize][point.x as usize];
            value == 1
        };
        // Skip obvious blocking values
        if grid[p.z as usize][p.y as usize][p.x as usize] == 1 { continue; }
        // Try to reach max.x from this point
        let mut test = *p;
        let mut diff = Point::new(1,0,0);
        let mut counter = 0;
        loop {
            test = diff + &test;
            if is_outside(test) {
                grid[p.z as usize][p.y as usize][p.x as usize] = 2;
                // println!("Removed ({}) because ({}) is outside", p, test);
                break;
            }
            if is_blocked(test) {
                test = *p;
                if counter == 0 {
                    // Try to reach min.x from this point
                    diff = Point::new(-1,0,0);
                }
                if counter == 1 {
                    // Try to reach max.y from this point
                    diff = Point::new(0,1,0);
                }
                if counter == 2 {
                    // Try to reach min.y from this point
                    diff = Point::new(0,-1,0);
                }
                if counter == 3 {
                    // Try to reach max.z from this point
                    diff = Point::new(0,0,1);
                }
                if counter == 4 {
                    // Try to reach min.z from this point
                    diff = Point::new(0,0,-1);
                }
                if counter == 5 {
                    grid[p.z as usize][p.y as usize][p.x as usize] = 3;
                    break;
                }
                counter += 1;
            }
        }
    }

    // Make a list of maybe iterior points
    let mut maybe_interior_points = Vec::<Point>::new();
    for x in min.x..max.x {
        for y in min.y..max.y {
            for z in min.z..max.z {
                if grid[z as usize][y as usize][x as usize] == 3 {
                    maybe_interior_points.push(Point::new(x,y,z));
                    grid[z as usize][y as usize][x as usize] = 0;
                }
            }
        }
    }

    println!("Maybe Interior points: {}", maybe_interior_points.len());
    // println!("{:?}", maybe_interior_points);

    // Group potential interior points in clusters
    let mut interior_points_clusters = Vec::<Vec<Point>>::new();
    let mut iterator = maybe_interior_points.iter();
    while let Some(this) = iterator.next() {
        if !interior_points_clusters.iter().any(|cl| cl.contains(this)) {
            let mut cluster = vec![*this];
            for other in maybe_interior_points.iter() {
                if !cluster.contains(&other) {
                    for p in cluster.iter() {
                        if Point::manhattan_distance(other, p) == 1 {
                            cluster.push(*other);
                            break;
                        }
                    }
                }
            }
            interior_points_clusters.push(cluster);
        }
    }
    
    println!("Interior points clusters: {}", interior_points_clusters.len());
    // println!("{:?}", interior_points_clusters);



    // Find cluster neighbours
    let mut list_of_cluster_neighbours = Vec::<Vec<Point>>::new();
    for cluster in interior_points_clusters.iter() {
        let mut cluster_neighbours = Vec::<Point>::new();
        for p in cluster.iter() {
            // Register points in the grid
            grid[p.z as usize][p.y as usize][p.x as usize] = 1;
            // Lambda test
            let mut fn_test_and_push = |neighbour: Point| {
                if neighbour.x >= 0 && neighbour.y >= 0 && neighbour.z >= 0
                && neighbour.x < max.x && neighbour.y < max.y && neighbour.z < max.z
                {
                    cluster_neighbours.push(neighbour)
                }
            };
            // 1st neighbour
            let neighbour = Point::new(p.x-1, p.y, p.z);
            fn_test_and_push(neighbour);
            // 2nd neighbour
            let neighbour = Point::new(p.x+1, p.y, p.z);
            fn_test_and_push(neighbour);
            // 3rd neighbour
            let neighbour = Point::new(p.x, p.y-1, p.z);
            fn_test_and_push(neighbour);
            // 4th neighbour
            let neighbour = Point::new(p.x, p.y+1, p.z);
            fn_test_and_push(neighbour);
            // 5th neighbour
            let neighbour = Point::new(p.x, p.y, p.z-1);
            fn_test_and_push(neighbour);
            // 6th neighbour
            let neighbour = Point::new(p.x, p.y, p.z+1);
            fn_test_and_push(neighbour);
        }
        list_of_cluster_neighbours.push(cluster_neighbours);
    }
    println!("Lists of cluster neighbours: {}", list_of_cluster_neighbours.len());



    // Inspect cluster neighbours to find which clusters are connected to the air
    let mut interior_points = Vec::<Point>::new();
    for (n, cluster_neighbour) in list_of_cluster_neighbours.into_iter().enumerate() {
        if cluster_neighbour.iter().all(|neighbour| {
            grid[neighbour.z as usize][neighbour.y as usize][neighbour.x as usize] == 1
        })
        {
            // This cluster is interior
            for interior_point in interior_points_clusters[n].iter() {
                interior_points.push(*interior_point);
            }
        }
    }



    println!("Interior points: {}", interior_points.len());
    // println!("{:?}", interior_points);




    // Total of interior faces
    let total_interior_faces: usize = interior_points.iter().fold(0, |acc, p| {
        let interior_faces: usize = interior_points.iter().fold(6, |acc, other| {
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
        acc + interior_faces
    });

    println!("Total exposed faces: {}", total_exposed_faces);
    println!("Total interior faces: {}", total_interior_faces);
    println!("Total external faces: {}", total_exposed_faces-total_interior_faces);
}
