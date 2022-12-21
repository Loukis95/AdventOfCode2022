use std::{env, fs};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operator {
    Add,
    Sub,
    Div,
    Mul,
    Equal,
}

#[derive(Debug, Clone)]
struct Operation {
    operand_1: Box<Math>,
    operand_2: Box<Math>,
    operator: Operator,
}

#[derive(Debug, Clone)]
enum Math {
    Number(isize),
    Operation(Operation),
    Guess,
}

fn find_in_input<'a>(input: &'a [&str], key: &str) -> Result<&'a str, ()>  {
    if let Ok(pos) = input.binary_search_by(|probe| probe[0..4].cmp(&key)) {
        Ok(input[pos])
    } else {
        Err(())
    }
}

fn build_math_from(input: &[&str], key: &str) -> Box<Math> {
    if key == "humn" {
        return Box::new(Math::Guess);
    }
    if let Ok(monkey_yell) = find_in_input(&input, key) {
        let mut split = monkey_yell.split_whitespace().skip(1);
        if let Some(operand_1) = split.next() {
            if let Some(operator) = split.next() {
                let mut operator = match operator {
                    "+" => Operator::Add,
                    "-" => Operator::Sub,
                    "/" => Operator::Div,
                    "*" => Operator::Mul,
                    _ => panic!("Unexpected operator: '{}'", operator),
                };
                if key == "root" { operator = Operator::Equal; }
                if let Some(operand_2) = split.next() {
                    let operand_1 = build_math_from(input, operand_1);
                    let operand_2 = build_math_from(input, operand_2);
                    if let Math::Number(number_1) = *operand_1 {
                        if let Math::Number(number_2) = *operand_2 {
                            let result = match operator {
                                Operator::Add => number_1 + number_2,
                                Operator::Sub => number_1 - number_2,
                                Operator::Div => number_1 / number_2,
                                Operator::Mul => number_1 * number_2,
                                _ => panic!("Unexpected operator: '{:?}'", operator),
                            };
                            return Box::new(Math::Number(result));
                        }
                    }
                    return Box::new(Math::Operation(Operation { operand_1, operand_2, operator }));
                } else {
                    panic!("Operation doesn't have 2 operands");
                }
            } else {
                let number: isize = operand_1.parse().unwrap();
                return Box::new(Math::Number(number));
            }
        } else {
            panic!("Can't parse monkey math for {}", key);
        }
    } else {
        panic!("Can't hear monkey {}", key);
    }
}

fn internal_resolve_math_tree(math_tree: &Math, expected_result: isize) -> isize {
    if let Math::Operation(operation) = math_tree {
        if let Math::Number(operand_1) = *operation.operand_1 {
            let new_expected_result = match operation.operator {
                Operator::Add => expected_result - operand_1,
                Operator::Sub => operand_1 - expected_result,
                Operator::Div => operand_1 / expected_result,
                Operator::Mul => expected_result / operand_1,
                _ => panic!("Unexpected Operator::Equal in internal_resolve_math_tree"),
            };
            if !matches!(*operation.operand_2, Math::Guess) {
                return internal_resolve_math_tree(&operation.operand_2, new_expected_result);
            } else {
                return new_expected_result;
            }
        }
        else if let Math::Number(operand_2) = *operation.operand_2 {
            let new_expected_result = match operation.operator {
                Operator::Add => expected_result - operand_2,
                Operator::Sub => expected_result + operand_2,
                Operator::Div => expected_result * operand_2,
                Operator::Mul => expected_result / operand_2,
                _ => panic!("Unexpected Operator::Equal in internal_resolve_math_tree"),
            };
            if !matches!(*operation.operand_1, Math::Guess) {
                return internal_resolve_math_tree(&operation.operand_1, new_expected_result);
            } else {
                return new_expected_result;
            }
        }
        else {
            panic!("Operation is expected to have at least one operand to be a number");
        }
    } else {
        panic!("Unexpected call to internal_resolve_math_tree");
    }
}

fn resolve_math_tree(math_tree: &Math) -> isize {
    if let Math::Operation(operation) = math_tree {
        if operation.operator == Operator::Equal {
            if let Math::Number(operand_1) = *operation.operand_1 {
                return internal_resolve_math_tree(&operation.operand_2, operand_1);
            }
            else if let Math::Number(operand_2) = *operation.operand_2 {
                return internal_resolve_math_tree(&operation.operand_1, operand_2);
            }
            else {
                panic!("FIXME")
            }
        } else {
            panic!("Unexpected !");
        }
    } else {
        panic!("Unexpected 2 !");
    }
}

fn main() {
    let args : Vec<_> = env::args().collect();
    let input_path = &args[1];
    let raw_input = fs::read(input_path).unwrap();
    let raw_string = String::from_utf8_lossy(&raw_input);
    let mut  input : Vec<_> = raw_string.lines().collect();

    // Sort the input lines for faster search with binary search
    input.sort_unstable();

    // Find monkey named root
    let math_tree = build_math_from(&input, "root");

    // Resolve the math tree
    let answer = resolve_math_tree(&math_tree);

    println!("humn needs to yell: {:?}", answer);
}
