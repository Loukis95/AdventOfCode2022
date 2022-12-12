use std::{env, fs, cmp, cmp::Reverse, collections::BinaryHeap};

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

fn cost(x:usize, y:usize, world:&Vec<usize>, m:usize, n:usize) -> usize {
    world[y*m+x]
}

fn hcost(x:usize, y:usize, world:&Vec<usize>, m:usize, n:usize) -> usize {
    cost(x, y, world, m, n)
}

fn main() {
    let args : Vec<_> = env::args().collect();
    let input_path = &args[1];
    let raw_input = fs::read(input_path).unwrap();
    let raw_string = String::from_utf8_lossy(&raw_input);
    let input : Vec<_> = raw_string.lines().collect();

    let n = input.len();
    let m = input[0].len();

    let world : Vec<_> = input.iter()
        .flat_map(|line| {
            line.chars().map(|c| usize::try_from(c.to_digit(10).unwrap()).unwrap()).collect::<Vec<_>>()
        })
        .collect();

    let start = Node::new(0, 0, 0, 0);
    
    let mut to_visit = BinaryHeap::<Reverse<Node>>::new();
    let mut visited = Vec::<(usize,usize,usize)>::new();
    let mut found = false;
    let mut path_cost: usize = 0;
    
    to_visit.push(Reverse(start));
    while !found {
        let Reverse(current) = to_visit.pop().unwrap();
        visited.push((current.x, current.y, current.hcost));
        if current.x == m-1 && current.y == n-1 {
            found = true;
            path_cost = current.cost;
        }
        else
        {
            if current.x > 0 {
                let cost = cost(current.x-1, current.y, &world, m, n);
                let hcost = hcost(current.x-1, current.y, &world, m, n);
                let new_node = Node::new(current.x-1, current.y, current.cost+cost, current.hcost+hcost);
                if None == to_visit.iter().find(|Reverse(node)| new_node.x == node.x && new_node.y == node.y && new_node.hcost >= node.hcost)
                && None == visited.iter().find(|node| new_node.x == node.0 && new_node.y == node.1 && new_node.hcost >= node.2)
                {
                    to_visit.push(Reverse(new_node));
                }
            }
            if current.x < m-1 {
                let cost = cost(current.x+1, current.y, &world, m, n);
                let hcost = hcost(current.x+1, current.y, &world, m, n);
                let new_node = Node::new(current.x+1, current.y, current.cost+cost, current.hcost+hcost);
                if None == to_visit.iter().find(|Reverse(node)| new_node.x == node.x && new_node.y == node.y && new_node.hcost >= node.hcost)
                && None == visited.iter().find(|node| new_node.x == node.0 && new_node.y == node.1 && new_node.hcost >= node.2)
                {
                    to_visit.push(Reverse(new_node));
                }
            }
            if current.y > 0 {
                let cost = cost(current.x, current.y-1, &world, m, n);
                let hcost = hcost(current.x, current.y-1, &world, m, n);
                let new_node = Node::new(current.x, current.y-1, current.cost+cost, current.hcost+hcost);
                if None == to_visit.iter().find(|Reverse(node)| new_node.x == node.x && new_node.y == node.y && new_node.hcost >= node.hcost)
                && None == visited.iter().find(|node| new_node.x == node.0 && new_node.y == node.1 && new_node.hcost >= node.2)
                {
                    to_visit.push(Reverse(new_node));
                }
            }
            if current.y < n-1 {
                let cost = cost(current.x, current.y+1, &world, m, n);
                let hcost = hcost(current.x, current.y+1, &world, m, n);
                let new_node = Node::new(current.x, current.y+1, current.cost+cost, current.hcost+hcost);
                if None == to_visit.iter().find(|Reverse(node)| new_node.x == node.x && new_node.y == node.y && new_node.hcost >= node.hcost)
                && None == visited.iter().find(|node| new_node.x == node.0 && new_node.y == node.1 && new_node.hcost >= node.2)
                {
                    to_visit.push(Reverse(new_node));
                }
            }
        }
    }

    println!("answer: {}", path_cost);
}