use std::{env, fs, cmp, cmp::Reverse, collections::BinaryHeap};

struct Position {
    x: usize,
    y: usize,
}

struct Node {
    x:usize,
    y:usize,
    cost:usize,
    hcost:usize,
}

impl Node {
    fn new(x:usize, y:usize, cost:usize, hcost:usize) -> Self {
        Self {
            x, y, cost, hcost,
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.hcost.eq(&other.hcost)
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.hcost.partial_cmp(&other.hcost)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.hcost.cmp(&other.hcost)
    }
}

fn cost(x:usize, y:usize, world:&[Vec<usize>], from_x:usize, from_y:usize) -> usize {
    let value = world[from_y][from_x];
    let new_value = world[y][x];
    if new_value > value+1 {
        return usize::MAX;
    } else {
        return 1;
    }
}

fn hcost(x:usize, y:usize, world:&[Vec<usize>], from_x:usize, from_y:usize) -> usize {
    cost(x, y, world, from_x, from_y)
}

fn main() {
    let args : Vec<_> = env::args().collect();
    let input_path = &args[1];
    let raw_input = fs::read(input_path).unwrap();
    let raw_string = String::from_utf8_lossy(&raw_input);
    let input : Vec<_> = raw_string.lines().collect();

    let n = input.len();
    let m = input[0].len();

    let mut start_position = Position{x:0, y:0};
    let mut end_position = Position{x:0, y:0};

    let world : Vec<Vec<_>> = input.iter()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
            .enumerate()
            .map(|(x, mut c)| {
                if c == 'S' {
                    start_position = Position{x, y};
                    c = 'a';
                }
                else if c == 'E' {
                    end_position = Position{x, y};
                    c = 'z';
                }
                let ret: usize = c as u32 as usize - 'a' as u32 as usize;
                ret
            }).collect::<Vec<_>>()
        })
        .collect();

    let start = Node::new(start_position.x, start_position.y, 0, 0);
    
    let mut to_visit = BinaryHeap::<Reverse<Node>>::new();
    let mut visited = Vec::<(usize,usize,usize)>::new();
    let mut found = false;
    let mut path_cost: usize = 0;
    
    to_visit.push(Reverse(start));
    while !found {
        let Reverse(current) = to_visit.pop().unwrap();
        visited.push((current.x, current.y, current.cost));
        if current.x == end_position.x && current.y == end_position.y {
            found = true;
            path_cost = current.cost;
        }
        else
        {
            if current.x > 0 {
                let cost = cost(current.x-1, current.y, &world, current.x, current.y);
                let hcost = hcost(current.x-1, current.y, &world, current.x, current.y);
                let new_node = Node::new(current.x-1, current.y, usize::saturating_add(current.cost,cost), usize::saturating_add(current.cost,hcost));
                if None == to_visit.iter().find(|Reverse(node)| new_node.x == node.x && new_node.y == node.y && new_node.hcost >= node.hcost)
                && None == visited.iter().find(|node| new_node.x == node.0 && new_node.y == node.1 && new_node.cost >= node.2)
                {
                    to_visit.push(Reverse(new_node));
                }
            }
            if current.x < m-1 {
                let cost = cost(current.x+1, current.y, &world, current.x, current.y);
                let hcost = hcost(current.x+1, current.y, &world, current.x, current.y);
                let new_node = Node::new(current.x+1, current.y, usize::saturating_add(current.cost,cost), usize::saturating_add(current.cost,hcost));
                if None == to_visit.iter().find(|Reverse(node)| new_node.x == node.x && new_node.y == node.y && new_node.hcost >= node.hcost)
                && None == visited.iter().find(|node| new_node.x == node.0 && new_node.y == node.1 && new_node.cost >= node.2)
                {
                    to_visit.push(Reverse(new_node));
                }
            }
            if current.y > 0 {
                let cost = cost(current.x, current.y-1, &world, current.x, current.y);
                let hcost = hcost(current.x, current.y-1, &world, current.x, current.y);
                let new_node = Node::new(current.x, current.y-1, usize::saturating_add(current.cost,cost), usize::saturating_add(current.cost,hcost));
                if None == to_visit.iter().find(|Reverse(node)| new_node.x == node.x && new_node.y == node.y && new_node.hcost >= node.hcost)
                && None == visited.iter().find(|node| new_node.x == node.0 && new_node.y == node.1 && new_node.cost >= node.2)
                {
                    to_visit.push(Reverse(new_node));
                }
            }
            if current.y < n-1 {
                let cost = cost(current.x, current.y+1, &world, current.x, current.y);
                let hcost = hcost(current.x, current.y+1, &world, current.x, current.y);
                let new_node = Node::new(current.x, current.y+1, usize::saturating_add(current.cost,cost), usize::saturating_add(current.cost,hcost));
                if None == to_visit.iter().find(|Reverse(node)| new_node.x == node.x && new_node.y == node.y && new_node.hcost >= node.hcost)
                && None == visited.iter().find(|node| new_node.x == node.0 && new_node.y == node.1 && new_node.cost >= node.2)
                {
                    to_visit.push(Reverse(new_node));
                }
            }
        }
    }

    println!("answer: {}", path_cost);
}