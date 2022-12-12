use std::{env, fs, collections::VecDeque, fmt::{Display, Debug}, ops::{Mul, MulAssign, AddAssign, Add, Div, DivAssign}};

#[derive(Debug, Default)]
struct ModuloValue {
    remainder: usize,
    divider: usize,
}

impl ModuloValue {
    fn new(value:usize, divider:usize) -> Self {
        Self {
            remainder: value % divider,
            divider,
        }
    }
}

#[derive(Debug, Default)]
struct ModuloArray {
    remainders: HashMap<usize, ModuloValue>,
}

impl ModuloArray {
    fn new(value: usize, dividers: &[usize]) -> Self {
        let mut this: Self = Default::default();
        for div in dividers {
            this.remainders[div] = ModuloValue::new(value, div);
        }
        this
    }
}

struct Monkey {
    name: String,
    items: VecDeque<ModuloArray>,
    operation: Box<dyn Fn(&mut ModuloArray)>,
    test: Box<dyn Fn(&ModuloArray) -> bool>,
    action: Box<dyn Fn(&mut[Monkey], bool, ModuloArray) -> ()>,
    activity: usize,
}

impl Monkey
{
    fn new( name:       &str,
            items:      VecDeque<ModuloArray>,
            operation:  Box<dyn Fn(&mut ModuloArray)>,
            test:       Box<dyn Fn(&ModuloArray) -> bool>,
            action:     Box<dyn Fn(&mut[Monkey], bool, ModuloArray) -> ()>) -> Self
    {
        Self { name: name.to_string(), items, operation, test, action, activity: 0 }
    }

    fn run(&mut self, monkeys: &mut[Monkey]) {
        while let Some(mut worry_level) = self.items.pop_front() {
            self.activity += 1;
            (self.operation)(&mut worry_level);
            let result = (self.test)(&worry_level);
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
    // let args : Vec<_> = env::args().collect();
    // let input_path = &args[1];
    let input_path = "src/input.txt";
    let raw_input = fs::read(input_path).unwrap();
    let raw_string = String::from_utf8_lossy(&raw_input);
    let input : Vec<_> = raw_string.lines().collect();

    let mut monkeys = Vec::<Monkey>::new();

    let mut line_iterator = input.iter();
    let mut next_line = line_iterator.next();
    while let Some(line) = next_line {
        // Read Monkey name
        if !line.starts_with("Monkey") { panic!("Expected line to start with \"Monkey\", got \"{}\"", line) }
        let name = &line[0..line.len()-1];
        
        // Read starting worry levels
        next_line = line_iterator.next();
        let line = next_line.unwrap().trim();
        if !line.starts_with("Starting items:") { panic!("Expected line to start with \"Starting items:\", got \"{}\"", line) }
        let worry_levels = line[15..].trim();
        let mut items = VecDeque::<PrimeFactor>::new();
        for worry_level in worry_levels.split(", ") {
            let n = worry_level.parse().unwrap();
            items.push_back(PrimeFactor::new(n));
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
        let operation: Box<dyn Fn(PrimeFactor)->PrimeFactor> = match operand {
            "old" => {
                match operator {
                    "*" => Box::new(move |old: PrimeFactor| -> PrimeFactor { old.clone() * old.clone() }),
                    "+" => Box::new(move |old: PrimeFactor| -> PrimeFactor { old.clone() + old.clone() }),
                    _ => panic!("Unexpected operator \"{}\" in Operation", operator),
                }
            },
            number => {
                let n = PrimeFactor::new(number.parse().unwrap());
                match operator {
                    "*" => Box::new(move |old: PrimeFactor| -> PrimeFactor { old * n.clone() }),
                    "+" => Box::new(move |old: PrimeFactor| -> PrimeFactor { old + n.clone() }),
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
        let bignum = PrimeFactor::new(n);
        let test: Box<dyn Fn(&PrimeFactor)->bool> = Box::new(move |worry_level| worry_level.is_divisible_by(&bignum));
        
        // Read action
        next_line = line_iterator.next();
        let line = next_line.unwrap().trim();
        if !line.starts_with("If true: throw to monkey") { panic!("Expected line to start with \"If true: throw to monkey\", got \"{}\"", line) }
        let idx_if_true = line[24..].trim().parse::<usize>().unwrap();

        next_line = line_iterator.next();
        let line = next_line.unwrap().trim();
        if !line.starts_with("If false: throw to monkey") { panic!("Expected line to start with \"If false: throw to monkey\", got \"{}\"", line) }
        let idx_if_false = line[25..].trim().parse::<usize>().unwrap();

        let action: Box<dyn Fn(&mut[Monkey], bool, PrimeFactor)->()> = Box::new(move |monkeys, result, worry_level| {
            if result {
                println!("Throw to Monkey {}", idx_if_true);
                monkeys[idx_if_true].items.push_back(worry_level);
            } else {
                println!("Throw to Monkey {}", idx_if_false);
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
        println!("======= Round {} =======", _round+1);
        for monkey_idx in 0..monkeys.len() {
            println!("---- Monkey {} ----", monkey_idx);
            println!("Before: {:?}", monkeys[monkey_idx]);
            let monkey: *mut Monkey = &mut monkeys[monkey_idx];
            unsafe { monkey.as_mut().unwrap() }.run(&mut monkeys);
            println!("After : {:?}", monkeys[monkey_idx]);
        }
    }
    println!("");

    monkeys.iter().for_each(|monkey| {
        println!("{} activity: {}", monkey.name, monkey.activity);
    });
    println!("");

    // Find the monkeys with the highest activity
    monkeys.sort_unstable_by_key(|monkey| monkey.activity);

    let result = monkeys.iter().rev().take(2).fold(1, |state, monkey| {
        println!("{} activity: {}", monkey.name, monkey.activity);
        state * monkey.activity
    });
    println!("Result: {}", result)
}
