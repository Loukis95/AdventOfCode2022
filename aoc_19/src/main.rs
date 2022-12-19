use std::{env, fs, collections::{HashMap, BinaryHeap, VecDeque}, cmp::Reverse, ops::{Sub, SubAssign, Add, AddAssign}};
use regex::Regex;

const TIME_LIMIT: usize = 24;

#[derive(Debug, Clone)]
struct Node {
    stack: Vec<State>,
    score: usize,
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

impl Node {
    fn score(next: &State, previous: &Node) -> usize {
        todo!()
    }

    fn heuristic(next: &State, previous: &Node) -> usize {
        todo!()
    }
}

#[derive(Debug, Clone)]
struct ShortNode {
    state: State,
    heuristic: usize,
}



#[derive(Debug, Clone, PartialEq, Eq)]
struct Blueprint {
    number: usize,
    ore_robot_cost: Resources,
    clay_robot_cost: Resources,
    obsidian_robot_cost: Resources,
    geode_robot_cost: Resources,
}




#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Resources {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

impl PartialOrd for Resources {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let cmp = [
            self.ore.cmp(&other.ore),
            self.clay.cmp(&other.clay),
            self.obsidian.cmp(&other.clay),
            self.geode.cmp(&other.clay),
        ];
        
        if cmp.iter().all(|ord| ord == &std::cmp::Ordering::Equal) {
            return Some(std::cmp::Ordering::Equal);
        }
        if cmp.iter().all(|ord| ord == &std::cmp::Ordering::Equal || ord == &std::cmp::Ordering::Less) {
            return Some(std::cmp::Ordering::Less);
        }
        if cmp.iter().all(|ord| ord == &std::cmp::Ordering::Equal || ord == &std::cmp::Ordering::Greater) {
            return Some(std::cmp::Ordering::Greater);
        }
        return None;
    }
}

impl Sub<Resources> for Resources {
    type Output = Resources;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl SubAssign<Resources> for Resources {
    fn sub_assign(&mut self, rhs: Self) {
        self.ore -= rhs.ore;
        self.clay -= rhs.clay;
        self.obsidian -= rhs.obsidian;
        self.geode -= rhs.geode;
    }
}

impl Add<Resources> for Resources {
    type Output = Resources;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign<Resources> for Resources {
    fn add_assign(&mut self, rhs: Self) {
        self.ore += rhs.ore;
        self.clay += rhs.clay;
        self.obsidian += rhs.obsidian;
        self.geode += rhs.geode;
    }
}




#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Action {
    Wait,
    BuildOreRobot,
    BuildClayRobot,
    BuildObsidianRobot,
    BuildGeodeRobot,
}




#[derive(Debug, Clone)]
struct State {
    minute: usize,
    action: Action,
    nb_ore_robots: usize,
    nb_clay_robots: usize,
    nb_obsidian_robots: usize,
    nb_geode_robots: usize,
    available_resources: Resources,
}




/**
 * Returns all possible actions from a specific state
 */
fn possible_actions_from_state(state: &State, blueprint: &Blueprint) -> Vec<Action> {
    let mut possible_actions = Vec::<Action>::new();
    if state.available_resources >= blueprint.ore_robot_cost {
        possible_actions.push(Action::BuildOreRobot);
    }
    if state.available_resources >= blueprint.clay_robot_cost {
        possible_actions.push(Action::BuildClayRobot);
    }
    if state.available_resources >= blueprint.obsidian_robot_cost {
        possible_actions.push(Action::BuildObsidianRobot);
    }
    if state.available_resources >= blueprint.geode_robot_cost {
        possible_actions.push(Action::BuildGeodeRobot);
    }
    if possible_actions.is_empty() {
        possible_actions.push(Action::Wait);
    }
    possible_actions
}

/**
 * Compute the next state from the previous state
 */
fn next_state_with_action(previous_state: &State, next_action: Action, blueprint: &Blueprint) -> State {
    let minute = previous_state.minute + 1;
    let action = next_action;
    let nb_ore_robots = {
        if next_action == Action::BuildOreRobot {
            previous_state.nb_ore_robots + 1
        } else {
            previous_state.nb_ore_robots
        }
    };
    let nb_clay_robots = {
        if next_action == Action::BuildClayRobot {
            previous_state.nb_clay_robots + 1
        } else {
            previous_state.nb_clay_robots
        }
    };
    let nb_obsidian_robots = {
        if next_action == Action::BuildObsidianRobot {
            previous_state.nb_obsidian_robots + 1
        } else {
            previous_state.nb_obsidian_robots
        }
    };
    let nb_geode_robots = {
        if next_action == Action::BuildGeodeRobot {
            previous_state.nb_geode_robots + 1
        } else {
            previous_state.nb_geode_robots
        }
    };
    let available_resources = {
        let mut tmp = previous_state.available_resources;
        tmp.ore += previous_state.nb_ore_robots;
        tmp.clay += previous_state.nb_clay_robots;
        tmp.obsidian += previous_state.nb_obsidian_robots;
        tmp.geode += previous_state.nb_geode_robots;
        match next_action {
            Action::Wait => tmp,
            Action::BuildOreRobot => tmp - blueprint.ore_robot_cost,
            Action::BuildClayRobot => tmp - blueprint.clay_robot_cost,
            Action::BuildObsidianRobot => tmp - blueprint.obsidian_robot_cost,
            Action::BuildGeodeRobot => tmp - blueprint.geode_robot_cost,
        }
    };

    State {
        minute,
        action,
        nb_ore_robots,
        nb_clay_robots,
        nb_obsidian_robots,
        nb_geode_robots,
        available_resources,
    }
}

fn main() {
    let args : Vec<_> = env::args().collect();
    let input_path = &args[1];
    let raw_input = fs::read(input_path).unwrap();
    let raw_string = String::from_utf8_lossy(&raw_input);
    let input : Vec<_> = raw_string.lines().collect();

    // Regex to parse input
    let re = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
    
    // Make a blueprint iterator
    let mut blueprint_iterator = input.iter().map(|line| {
        let capture = re.captures(line).unwrap();
        // Parse the input
        let number: usize = capture.get(1).unwrap().as_str().parse().unwrap();
        let ore_robot_cost = Resources {
            ore: capture.get(2).unwrap().as_str().parse().unwrap(),
            clay: 0,
            obsidian: 0,
            geode: 0,
        };
        let clay_robot_cost = Resources {
            ore: capture.get(3).unwrap().as_str().parse().unwrap(),
            clay: 0,
            obsidian: 0,
            geode: 0,
        };
        let obsidian_robot_cost = Resources {
            ore: capture.get(4).unwrap().as_str().parse().unwrap(),
            clay: capture.get(5).unwrap().as_str().parse().unwrap(),
            obsidian: 0,
            geode: 0,
        };
        let geode_robot_cost = Resources {
            ore: capture.get(6).unwrap().as_str().parse().unwrap(),
            clay: 0,
            obsidian: capture.get(7).unwrap().as_str().parse().unwrap(),
            geode: 0,
        };
        // Return the blueprint
        Blueprint {
            number,
            ore_robot_cost,
            clay_robot_cost,
            obsidian_robot_cost,
            geode_robot_cost,
        }
    });

    // Starting state
    let starting_state = State {
        minute: 0,
        action: Action::Wait,
        nb_ore_robots: 1,
        nb_clay_robots: 0,
        nb_obsidian_robots: 0,
        nb_geode_robots: 0,
        available_resources: Resources {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0
        },
    };

    // Starting node
    let starting_node = Node {
        stack: vec![starting_state],
        score: 0,
        heuristic: 0,
    };

    // For now, we just use the 1st blueprint
    if let Some(blueprint) = blueprint_iterator.next() {

        // Data for the search
        let mut to_visit = BinaryHeap::<Node>::new();       // Priority queue of nodes to visit
        let mut visited = Vec::<ShortNode>::new();            // A list of the best node used to avoid searching non-promising states
        let mut found: Option<Node> = None;                                   // Used to store the best path found
        to_visit.push(starting_node.clone());


        // The Loop
        while let Some(current_node) = to_visit.pop() {
            // Push this node to the list of visited nodes
            visited.push(ShortNode{
                state: current_node.stack.last().unwrap().clone(),
                heuristic: current_node.heuristic,
            });

            // Maybe found a better node
            if current_node.stack.len() >= TIME_LIMIT {
                if let Some(tmp) = &found {
                    // If node has a better score than the previous
                    if tmp.score < current_node.score {
                        // Replace with this node
                        found = Some(current_node);
                    }
                } else {
                    // First node to reach the time limit, it is the best for now
                    found = Some(current_node);
                }
                // Keep looping because we are searching for the best
                continue;
            }

            // Compute the possible actions from the current state
            let current_state = current_node.stack.last().unwrap();
            let possible_actions = possible_actions_from_state(&current_state, &blueprint);

            // For each possible action
            for next_action in possible_actions.into_iter() {
                // Compute the next state when executing this action
                let next_state = next_state_with_action(current_state, next_action, &blueprint);
                // Create a Node object
                let mut stack = current_node.stack.clone();
                stack.push(next_state.clone());
                let next_node = Node {
                    stack,
                    score: Node::score(&next_state, &current_node),
                    heuristic: Node::heuristic(&next_state, &current_node),
                };
                // Add this node to the search if and only if no other node are better
                // if visited.iter().all(|(name, step, h)| name != &next.name || step != &next.stack.len() || h < &next.heuristic) 
                // && to_visit.iter().all(|other| &other.name != &next.name || other.stack.len() != next.stack.len() || &other.heuristic < &next.heuristic)
                // {
                //     to_visit.push(next);
                // }
            }
        }

    } // blueprint loop


    // while !to_visit.is_empty() {
    //     let current = to_visit.pop().unwrap();
    //     visited.push((current.name.clone(), current.stack.len(), current.heuristic));
    //     {
    //         println!("Exploring node with depth: {}, cost: {}, h: {}, search depth: {}", current.stack.len(), current.cost, current.heuristic, to_visit.len());
    //         // for (n, step) in current.stack.iter().enumerate() {
    //         //     println!("======== Step {} ========", n);
    //         //     match &step {
    //         //         Action::WaitAt(s) => println!("Chill at node {}", s),
    //         //         Action::OpenValve(s) => println!("Open valve {}", s),
    //         //         Action::MoveToValve(s) => println!("Move to valve {}", s),
    //         //     }
    //         // }
    //         // println!("");
    //     }
    //     // Stop condition
    //     if current.stack.len() >= DEPTH_LIMIT {
    //         let mut stack = current.stack.clone();
    //         stack.push(current.action.clone());
    //         let mut end = current.clone();
    //         end.stack = stack;
    //         if let Some(tmp) = &found {
    //             if tmp.cost < end.cost {
    //                 found = Some(end);
    //             }
    //         } else {
    //             found = Some(end);
    //         }
    //         continue;
    //     }
    //     // find closed valves for current state
    //     let closed = closed_valves(&current.open_valves, &graph);
    //     // println!("Closed valves: {:?}", closed);
    //     for closed_valve in closed.iter() {
    //         // find shortest path to closed valve
    //         // print!("Shortest path to {} from {}", closed_valve, &current.name);
    //         let shortest_path = shortest_path_to_valve(closed_valve, &current.name, &graph);
    //         // println!(" => {:?}", shortest_path);
    //         let mut stack = current.stack.clone();
    //         stack.push(current.action.clone());
    //         let mut previous = current.clone();
    //         // Simulate the steps to reach this valve
    //         for step in shortest_path.iter().skip(1) {
    //             let mut next = Node { 
    //                 name: step.clone(),
    //                 open_valves: previous.open_valves.clone(),
    //                 action: Action::MoveToValve(step.clone()),
    //                 stack: stack.clone(), 
    //                 flow_tick: previous.flow_tick,
    //                 cost: 0,
    //                 heuristic: 0,
    //             };
    //             next.cost = cost(&next, &previous, &graph);
    //             next.heuristic = heuristic(&next, &previous, &graph);

    //             if next.stack.len() >= DEPTH_LIMIT { break; }

    //             stack.push(next.action.clone());
    //             // println!("Push to stack: {:?}", next.action);
    //             previous = next;
    //         }
    //         // Open the closed valve
    //         let mut open_valves = previous.open_valves.clone();
    //         open_valves.push(previous.name.clone());
    //         let flow_tick = previous.flow_tick + graph.get(&previous.name).unwrap().flow_rate;
    //         let mut next = Node { 
    //             name: previous.name.clone(),
    //             open_valves,
    //             action: Action::OpenValve(previous.name.clone()),
    //             stack: stack,
    //             flow_tick,
    //             cost: 0,
    //             heuristic: 0,
    //         };
    //         next.cost = cost(&next, &previous, &graph);
    //         next.heuristic = heuristic(&next, &previous, &graph);
    //         // println!("Push to queue: {:?}", next.action);
    //         if visited.iter().all(|(name, step, h)| name != &next.name || step != &next.stack.len() || h < &next.heuristic) 
    //         && to_visit.iter().all(|other| &other.name != &next.name || other.stack.len() != next.stack.len() || &other.heuristic < &next.heuristic)
    //         {
    //             to_visit.push(next);
    //         }
    //     }
    //     // Or we can just chill
    //     if closed.len() == 0 {
    //         let mut stack = current.stack.clone();
    //         stack.push(current.action.clone());
    //         let mut next = Node { 
    //             name: current.name.clone(),
    //             open_valves: current.open_valves.clone(),
    //             action: Action::WaitAt(current.name.clone()),
    //             stack: stack, 
    //             flow_tick: current.flow_tick,
    //             cost: 0,
    //             heuristic: 0
    //         };
    //         let cost = cost(&next, &current, &graph);
    //         let heuristic = heuristic(&next, &current, &graph);
    //         next.cost = cost;
    //         next.heuristic = heuristic;
    //         // Add nodes to the list
    //         to_visit.push(next);
    //     }
    // }




    // let node = found.unwrap();
    // for (n, step) in node.stack.iter().enumerate() {
    //     println!("======== Step {} ========", n);
    //     match &step {
    //         Action::WaitAt(s) => println!("Chill at node {}", s),
    //         Action::OpenValve(s) => println!("Open valve {}", s),
    //         Action::MoveToValve(s) => println!("Move to valve {}", s),
    //     }
    // }
    // println!("Released pressure: {}", node.cost);
}
