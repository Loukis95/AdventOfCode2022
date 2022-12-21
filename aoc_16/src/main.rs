use std::{env, fs, collections::{HashMap, BinaryHeap, VecDeque}, cmp::Reverse};

const DEPTH_LIMIT: usize = 30;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Valve {
    name: String,
    flow_rate: usize,
    tunnels: Vec<String>
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Action {
    WaitAt(String),
    MoveToValve(String),
    OpenValve(String),
}

#[derive(Debug, Clone)]
struct Node {
    name: String,
    open_valves: Vec<String>,
    action: Action,
    stack: Vec<Action>,
    flow_tick: usize,
    cost: usize,
    heuristic: usize,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.heuristic.eq(&other.heuristic)
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.heuristic.partial_cmp(&other.heuristic)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.heuristic.cmp(&other.heuristic)
    }
}

fn cost(_this: &Node, from: &Node, _graph: &HashMap<String, Valve>) -> usize {
    from.cost + from.flow_tick
}

fn heuristic(this: &Node, from: &Node, graph: &HashMap<String, Valve>) -> usize {
    match &this.action {
        Action::OpenValve(s) => {
            let flow_rate = graph.get(s).unwrap().flow_rate;
            let depth = this.stack.len();
            from.heuristic + (DEPTH_LIMIT - depth) * flow_rate
        }
        _ => from.heuristic
    }
}

fn closed_valves(open_valves: &[String], graph: &HashMap<String,Valve>) -> Vec<String> {
    let mut closed_valves = Vec::<String>::new();
    for valve in graph.keys() {
        if !open_valves.contains(valve) && graph.get(valve).unwrap().flow_rate != 0 {
            closed_valves.push(valve.to_string());
        }
    }
    closed_valves
}

#[derive(Debug, Clone)]
struct ShortNode {
    name: String,
    path: Vec<String>,
}

impl PartialEq for ShortNode {
    fn eq(&self, other: &Self) -> bool {
        self.path.len().eq(&other.path.len())
    }
}

impl Eq for ShortNode {}

impl PartialOrd for ShortNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.path.len().partial_cmp(&other.path.len())
    }
}

impl Ord for ShortNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.path.len().cmp(&other.path.len())
    }
}

fn shortest_path_to_valve(target: &str, from: &str, graph: &HashMap<String,Valve>) -> Vec<String> {
    let mut to_visit = BinaryHeap::<Reverse<ShortNode>>::new();
    let mut visited = Vec::<(String, usize)>::new();
    let found: Option<ShortNode>;
    let start = ShortNode{name: from.to_string(), path: Vec::<String>::new()};
    to_visit.push(Reverse(start));
    loop {
        let current = to_visit.pop().unwrap().0;
        visited.push((current.name.clone(), current.path.len()));
        if &current.name == target {
            found = Some(current);
            break;
        }
        let nearby_valves = &graph.get(&current.name).unwrap().tunnels;
        for nearby_valve in nearby_valves {
            let mut path = current.path.clone();
            path.push(current.name.clone());
            let path_length = path.len();
            let next = ShortNode{name: nearby_valve.clone(), path};
            if visited.iter().all(|(valve, cost)| valve != nearby_valve || cost > &path_length) 
            && to_visit.iter().all(|Reverse(node)| &node.name != nearby_valve || node.path.len() > path_length)
            {
                to_visit.push(Reverse(next));
            }
        }
    }
    let found = found.unwrap();
    let mut path = found.path;
    path.push(found.name.clone());
    path
}

fn main() {
    let args : Vec<_> = env::args().collect();
    let input_path = &args[1];
    let raw_input = fs::read(input_path).unwrap();
    let raw_string = String::from_utf8_lossy(&raw_input);
    let input : Vec<_> = raw_string.lines().collect();

    let mut graph = HashMap::<String, Valve>::new();

    input.iter().for_each(|line| {
        let mut it = line.split(";");
        let part1 = it.next().unwrap();
        let part2 = it.next().unwrap();
        let mut it = part1.split_whitespace().skip(1);
        let name = it.next().unwrap();
        let mut it = it.skip(2);
        let flow_rate = it.next().unwrap();
        let flow_rate = flow_rate.split("=").nth(1).unwrap();
        let flow_rate: usize = flow_rate.parse().unwrap();
        let mut valve = Valve{name: name.to_string(), flow_rate, tunnels: Vec::<String>::new()};
        let mut it = part2.split_whitespace().skip(4);
        while let Some(tunnel) = it.next() {
            let tunnel = tunnel.trim_start_matches("tunnels leads to valves");
            let tunnel = tunnel.trim_start_matches("tunnels leads to valve");
            let tunnel = tunnel.trim_end_matches(",");
            valve.tunnels.push(tunnel.to_string());
        }
        println!("{:?}", valve);
        graph.insert(name.to_string(), valve);
    });

    // Create the starting node
    let start = Node {
        name: "AA".to_string(),
        open_valves: Vec::<String>::new(),
        action: Action::WaitAt("AA".to_string()),
        stack: Vec::<Action>::new(),
        flow_tick: 0,
        cost: 0,
        heuristic: 0
    };

    let mut to_visit = BinaryHeap::<Node>::new();
    let mut visited = Vec::<(String, usize, usize)>::new(); // < Node name, (step, heuristic)>
    let mut found: Option<Node> = None;
    to_visit.push(start);
    while !to_visit.is_empty() {
        let current = to_visit.pop().unwrap();
        visited.push((current.name.clone(), current.stack.len(), current.heuristic));
        {
            println!("Exploring node with depth: {}, cost: {}, h: {}, search depth: {}", current.stack.len(), current.cost, current.heuristic, to_visit.len());
            // for (n, step) in current.stack.iter().enumerate() {
            //     println!("======== Step {} ========", n);
            //     match &step {
            //         Action::WaitAt(s) => println!("Chill at node {}", s),
            //         Action::OpenValve(s) => println!("Open valve {}", s),
            //         Action::MoveToValve(s) => println!("Move to valve {}", s),
            //     }
            // }
            // println!("");
        }
        // Stop condition
        if current.stack.len() >= DEPTH_LIMIT {
            let mut stack = current.stack.clone();
            stack.push(current.action.clone());
            let mut end = current.clone();
            end.stack = stack;
            if let Some(tmp) = &found {
                if tmp.cost < end.cost {
                    found = Some(end);
                }
            } else {
                found = Some(end);
            }
            continue;
        }
        // find closed valves for current state
        let closed = closed_valves(&current.open_valves, &graph);
        // println!("Closed valves: {:?}", closed);
        for closed_valve in closed.iter() {
            // find shortest path to closed valve
            // print!("Shortest path to {} from {}", closed_valve, &current.name);
            let shortest_path = shortest_path_to_valve(closed_valve, &current.name, &graph);
            // println!(" => {:?}", shortest_path);
            let mut stack = current.stack.clone();
            stack.push(current.action.clone());
            let mut previous = current.clone();
            // Simulate the steps to reach this valve
            for step in shortest_path.iter().skip(1) {
                let mut next = Node { 
                    name: step.clone(),
                    open_valves: previous.open_valves.clone(),
                    action: Action::MoveToValve(step.clone()),
                    stack: stack.clone(), 
                    flow_tick: previous.flow_tick,
                    cost: 0,
                    heuristic: 0,
                };
                next.cost = cost(&next, &previous, &graph);
                next.heuristic = heuristic(&next, &previous, &graph);

                if next.stack.len() >= DEPTH_LIMIT { break; }

                stack.push(next.action.clone());
                // println!("Push to stack: {:?}", next.action);
                previous = next;
            }
            // Open the closed valve
            let mut open_valves = previous.open_valves.clone();
            open_valves.push(previous.name.clone());
            let flow_tick = previous.flow_tick + graph.get(&previous.name).unwrap().flow_rate;
            let mut next = Node { 
                name: previous.name.clone(),
                open_valves,
                action: Action::OpenValve(previous.name.clone()),
                stack: stack,
                flow_tick,
                cost: 0,
                heuristic: 0,
            };
            next.cost = cost(&next, &previous, &graph);
            next.heuristic = heuristic(&next, &previous, &graph);
            // println!("Push to queue: {:?}", next.action);
            if visited.iter().all(|(name, step, h)| name != &next.name || step != &next.stack.len() || h < &next.heuristic) 
            && to_visit.iter().all(|other| &other.name != &next.name || other.stack.len() != next.stack.len() || &other.heuristic < &next.heuristic)
            {
                to_visit.push(next);
            }
        }
        // Or we can just chill
        if closed.len() == 0 {
            let mut stack = current.stack.clone();
            stack.push(current.action.clone());
            let mut next = Node { 
                name: current.name.clone(),
                open_valves: current.open_valves.clone(),
                action: Action::WaitAt(current.name.clone()),
                stack: stack, 
                flow_tick: current.flow_tick,
                cost: 0,
                heuristic: 0
            };
            let cost = cost(&next, &current, &graph);
            let heuristic = heuristic(&next, &current, &graph);
            next.cost = cost;
            next.heuristic = heuristic;
            // Add nodes to the list
            to_visit.push(next);
        }
    }




    let node = found.unwrap();
    for (n, step) in node.stack.iter().enumerate() {
        println!("======== Step {} ========", n);
        match &step {
            Action::WaitAt(s) => println!("Chill at node {}", s),
            Action::OpenValve(s) => println!("Open valve {}", s),
            Action::MoveToValve(s) => println!("Move to valve {}", s),
        }
    }
    println!("Released pressure: {}", node.cost);
}
