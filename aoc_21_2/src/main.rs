use std::{env, fs};

enum Operator {
    Add,
    Sub,
    Div,
    Mul,
}

fn find_in_input<'a>(input: &'a [&str], key: &str) -> Result<&'a str, ()>  {
    if let Ok(pos) = input.binary_search_by(|probe| probe[0..4].cmp(&key)) {
        Ok(input[pos])
    } else {
        Err(())
    }
}

fn build_math_from(input: &[&str], key: &str) -> isize {
    if let Ok(monkey_yell) = find_in_input(&input, key) {
        let mut split = monkey_yell.split_whitespace().skip(1);
        if let Some(operand_1) = split.next() {
            if let Some(operator) = split.next() {
                let operator = match operator {
                    "+" => Operator::Add,
                    "-" => Operator::Sub,
                    "/" => Operator::Div,
                    "*" => Operator::Mul,
                    _ => panic!("Unexpected operator: '{}'", operator),
                };
                if let Some(operand_2) = split.next() {
                    let operand_1 = build_math_from(input, operand_1);
                    let operand_2 = build_math_from(input, operand_2);
                    return match operator {
                        Operator::Add => operand_1 + operand_2,
                        Operator::Sub => operand_1 - operand_2,
                        Operator::Div => operand_1 / operand_2,
                        Operator::Mul => operand_1 * operand_2,
                    };
                } else {
                    panic!("Operation doesn't have 2 operands");
                }
            } else {
                return operand_1.parse().unwrap();
            }
        } else {
            panic!("Can't parse monkey math for {}", key);
        }
    } else {
        panic!("Can't hear monkey {}", key);
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
    let answer = build_math_from(&input, "root");
    println!("answer: {}", answer);
}
