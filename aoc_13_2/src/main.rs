use std::{env, fs, str::FromStr};

#[derive(PartialEq, Eq, Debug)]
enum Element {
    Integer(usize),
    List(Vec<Element>),
}

impl Element {

    #[inline]
    pub const fn is_integer(&self) -> bool {
        matches!(*self, Element::Integer(_))
    }

    #[inline]
    pub const fn is_list(&self) -> bool {
        matches!(*self, Element::List(_))
    }
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if let Element::Integer(a) = self {
            if let Element::Integer(b) = other {
                return a.partial_cmp(b)
            } else {
                let mut list = Vec::<Element>::new();
                list.push(Element::Integer(*a));
                let list_a = Element::List(list);
                return list_a.partial_cmp(other)
            }
        } else {
            if let Element::Integer(b) = other {
                let mut list = Vec::<Element>::new();
                list.push(Element::Integer(*b));
                let list_b = Element::List(list);
                return self.partial_cmp(&list_b)
            } else {
                if let Element::List(list_a) = self {
                    if let Element::List(list_b) = other {
                        let result = list_a.iter().partial_cmp(list_b.iter());
                        // println!("{:?} <=> {:?} => {:?}", list_a, list_b, result);
                        return result
                    }
                    else {
                        panic!("This should never happen")
                    }
                }
                else {
                    panic!("This should never happen")
                }
            }
        }
    }
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl FromStr for Element {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // println!("parse: {}", s);
        let s = s.trim();
        if let Some(s) = s.strip_prefix("[") {
            let mut list = Vec::<Element>::new();
            let mut nesting_level: usize = 0;
            let mut rest = s;
            while let Some((idx, _c)) = rest.char_indices().find(|(_idx, c)| {
                match c {
                    ']' => if nesting_level == 0 { return true; } else { nesting_level -= 1},
                    '[' => nesting_level += 1,
                    ',' => if nesting_level == 0 { return true; },
                    _ => (),
                }
                return false;
            }) {
                let (part, tmp) = rest.split_at(idx);
                rest = tmp.strip_prefix(_c).unwrap();
                // println!("split at {} ({}): \"{}\" - \"{}\"", idx, _c, part, rest);
                if part.len() != 0 {
                    if let Ok(element) = part.parse::<Element>() {
                        list.push(element);
                    } else {
                        return Err(());
                    }
                }
            }
            Ok(Element::List(list))
        } else {
            if let Ok(number) = s.parse::<usize>() {
                return Ok(Element::Integer(number));
            } else {
                println!("Error: could not parse \"{}\"", s);
                return Err(());
            }
        }
    }
}

fn main() {
    let args : Vec<_> = env::args().collect();
    let input_path = &args[1];
    let raw_input = fs::read(input_path).unwrap();
    let raw_string = String::from_utf8_lossy(&raw_input);
    let input : Vec<_> = raw_string.lines().collect();

    let input_1 = input.iter().step_by(3);
    let input_2 = input.iter().skip(1).step_by(3);

    let mut all_pairs: Vec<Element> = input_1.zip(input_2).enumerate().flat_map(|(n,(a, b))| {
        let elem_a : Element = a.parse().unwrap();
        let elem_b : Element = b.parse().unwrap();
        println!("{}: {:?} <=> {:?} => {:?}", n+1, a, b, elem_a.partial_cmp(&elem_b));
        vec![elem_a, elem_b]
    })
    .collect();

    println!("");
    all_pairs.sort();

    let result: usize = all_pairs.iter().enumerate().filter_map(|(n, elem)| {
        println!("{}: {:?}", n+1, elem);
        let mark_1: Element = "[[2]]".parse().unwrap();
        let mark_2: Element = "[[6]]".parse().unwrap();
        if elem == &mark_1 || elem == &mark_2 { Some(n+1) }
        else { None }
    }).product();

    println!("result: {}", result);
}
