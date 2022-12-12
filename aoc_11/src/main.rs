use std::{env, fs, collections::VecDeque, fmt::{Display, Debug}};

struct Monkey {
    name: String,
    items: VecDeque<usize>,
    operation: Box<dyn Fn(usize) -> usize>,
    test: Box<dyn Fn(usize) -> bool>,
    action: Box<dyn Fn(&mut Vec<Monkey>, bool, usize) -> ()>,
    activity: usize,
}

impl Monkey
{
    fn new( name:       &str,
            items:      VecDeque<usize>,
            operation:  Box<dyn Fn(usize) -> usize>,
            test:       Box<dyn Fn(usize) -> bool>,
            action:     Box<dyn Fn(&mut Vec<Monkey>, bool, usize) -> ()>) -> Self
    {
        Self { name: name.to_string(), items, operation, test, action, activity: 0 }
    }

    fn run(&mut self, monkeys: &mut Vec<Monkey>) {
        while let Some(mut worry_level) = self.items.pop_front() {
            self.activity += 1;
            worry_level = (self.operation)(worry_level);
            worry_level /= 3;
            let result = (self.test)(worry_level);
            (self.action)(monkeys, result, worry_level);
        }
    }
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Monkey").field("name", &self.name).field("items", &self.items).field("activity", &self.activity).finish()
    }
}

impl Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.activity)
    }
}

fn main() {
    let args : Vec<_> = env::args().collect();
    let input_path = &args[1];
    let raw_input = fs::read(input_path).unwrap();
    let raw_string = String::from_utf8_lossy(&raw_input);
    let input : Vec<_> = raw_string.lines().collect();

    let mut monkeys = Vec::<Monkey>::new();

    let mut line_iterator = input.iter();
    let mut next_line = line_iterator.next();
    while let Some(line) = next_line {
        // Read Monkey name
        if !line.starts_with("Monkey") { panic!("Expected line to start with \"Monkey\", got \"{}\"", line) }
        let name = line;
        
        // Read starting worry levels
        next_line = line_iterator.next();
        let line = next_line.unwrap().trim();
        if !line.starts_with("Starting items:") { panic!("Expected line to start with \"Starting items:\", got \"{}\"", line) }
        let worry_levels = line[15..].trim();
        let mut items = VecDeque::<usize>::new();
        for worry_level in worry_levels.split(", ") {
            items.push_back(worry_level.parse().unwrap());
        }
        
        // Read operation
        next_line = line_iterator.next();
        let line = next_line.unwrap().trim();
        if !line.starts_with("Operation:") { panic!("Expected line to start with \"Operation:\", got \"{}\"", line) }
        let operation_text = line[10..].trim();
        if !operation_text.starts_with("new = old") { panic!("Expected operation to start with \"new = old\", got \"{}\"", operation_text) }
        let operation_text = operation_text[9..].trim();
        let operator = operation_text.split_whitespace().nth(0).unwrap();
        let operand = operation_text.split_whitespace().nth(1).unwrap();
        let operation: Box<dyn Fn(usize)->usize> = match operand {
            "old" => {
                match operator {
                    "*" => Box::new(move |old: usize| -> usize { old * old }),
                    "+" => Box::new(move |old: usize| -> usize { old + old }),
                    _ => panic!("Unexpected operator \"{}\" in Operation", operator),
                }
            },
            number => {
                let n : usize = number.parse().unwrap();
                match operator {
                    "*" => Box::new(move |old: usize| -> usize { old * n }),
                    "+" => Box::new(move |old: usize| -> usize { old + n }),
                    _ => panic!("Unexpected operator \"{}\" in Operation", operator),
                }
            }
        };
        
        // Read test
        next_line = line_iterator.next();
        let line = next_line.unwrap().trim();
        if !line.starts_with("Test:") { panic!("Expected line to start with \"Test:\", got \"{}\"", line) }
        let test_text = line[5..].trim();
        if !test_text.starts_with("divisible by") { panic!("Expected test to start with \"divisible by\", got \"{}\"", test_text) }
        let n = test_text[12..].trim().parse::<usize>().unwrap();
        let test: Box<dyn Fn(usize)->bool> = Box::new(move |worry_level| (worry_level % n) == 0);
        
        // Read action
        next_line = line_iterator.next();
        let line = next_line.unwrap().trim();
        if !line.starts_with("If true: throw to monkey") { panic!("Expected line to start with \"If true: throw to monkey\", got \"{}\"", line) }
        let idx_if_true = line[24..].trim().parse::<usize>().unwrap();

        next_line = line_iterator.next();
        let line = next_line.unwrap().trim();
        if !line.starts_with("If false: throw to monkey") { panic!("Expected line to start with \"If false: throw to monkey\", got \"{}\"", line) }
        let idx_if_false = line[25..].trim().parse::<usize>().unwrap();

        let action: Box<dyn Fn(&mut Vec<Monkey>, bool, usize)->()> = Box::new(move |monkeys, result, worry_level| {
            if result {
                monkeys[idx_if_true].items.push_back(worry_level);
            } else {
                monkeys[idx_if_false].items.push_back(worry_level);
            }
        });

        monkeys.push(Monkey::new(name, items, operation, test, action));

        // Read next line
        line_iterator.next();
        next_line = line_iterator.next();
    }

    // Run simulation
    for _round in 0..20 {
        for monkey_idx in 0..monkeys.len() {
            let monkey: *mut Monkey = &mut monkeys[monkey_idx];
            unsafe { monkey.as_mut().unwrap() }.run(&mut monkeys);
        }
    }

    // Find the monkeys with the highest activity
    monkeys.sort_unstable_by_key(|monkey| monkey.activity);

    let result = monkeys.iter().rev().take(2).fold(1, |state, monkey| {
        println!("{} activity: {}", monkey.name, monkey.activity);
        state * monkey.activity
    });
    println!("Result: {}", result)
}
