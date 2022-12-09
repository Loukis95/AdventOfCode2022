use std::borrow::Borrow;
use std::cell::Ref;
use std::collections::VecDeque;
use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::rc::Weak;
use std::{env, fs};
use std::any::Any;

use std::{cell::RefCell, rc::Rc};

struct ListNode<T> {
    item: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> ListNode<T> {
    fn new(item: T) -> Self {
        Self {
            item,
            next: None,
            prev: None,
        }
    }
}

type Link<T> = Option<Rc<RefCell<ListNode<T>>>>;

#[derive(Default)]
pub struct DoublyLinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    size: usize,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            size: 0,
        }
    }

    pub fn push_back(&mut self, item: T) {
        let node = Rc::new(RefCell::new(ListNode::new(item)));
        if let Some(prev_tail) = self.tail.take() {
            prev_tail.borrow_mut().next = Some(Rc::clone(&node));
            node.borrow_mut().prev = Some(prev_tail);
            self.tail = Some(node);
            self.size += 1;
        } else {
            self.head = Some(Rc::clone(&node));
            self.tail = Some(node);
            self.size = 1;
        }
    }

    pub fn push_front(&mut self, item: T) {
        let node = Rc::new(RefCell::new(ListNode::new(item)));
        if let Some(prev_head) = self.head.take() {
            prev_head.borrow_mut().prev = Some(Rc::clone(&node));
            node.borrow_mut().next = Some(prev_head);
            self.head = Some(node);
            self.size += 1;
        } else {
            self.head = Some(Rc::clone(&node));
            self.tail = Some(node);
            self.size = 1;
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|prev_tail| {
            self.size -= 1;
            match prev_tail.borrow_mut().prev.take() {
                Some(node) => {
                    node.borrow_mut().next = None;
                    self.tail = Some(node);
                }
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(prev_tail).ok().unwrap().into_inner().item
        })
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|prev_head| {
            self.size -= 1;
            match prev_head.borrow_mut().next.take() {
                Some(node) => {
                    node.borrow_mut().prev = None;
                    self.head = Some(node);
                }
                None => {
                    self.tail.take();
                }
            }
            Rc::try_unwrap(prev_head).ok().unwrap().into_inner().item
        })
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        self.size
    }
}

impl<T> IntoIterator for DoublyLinkedList<T> {
    type Item = <ListIterator<T> as Iterator>::Item;

    type IntoIter = ListIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl<T> Drop for DoublyLinkedList<T> {
    fn drop(&mut self) {
        while let Some(node) = self.head.take() {
            let _ = node.borrow_mut().prev.take();
            self.head = node.borrow_mut().next.take();
        }
        self.tail.take();
    }
}

pub struct ListIterator<T> {
    list: DoublyLinkedList<T>,
}

impl<T> ListIterator<T> {
    fn new(list: DoublyLinkedList<T>) -> Self {
        Self { list }
    }
}

impl<T> Iterator for ListIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }
}

impl<T> DoubleEndedIterator for ListIterator<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.list.pop_back()
    }
}





struct TreeNode<T> {
    item: T,
    nested_level: usize,
    children: Vec<TreeChild<T>>,
    parent: TreeParent<T>,
    this: TreeParent<T>,
}

type TreeParent<T> = Weak<RefCell<TreeNode<T>>>;
type TreeChild<T> = Rc<RefCell<TreeNode<T>>>;

impl<T> TreeNode<T> {
    fn new(item: T) -> TreeChild<T> {
        let node = Self {
            item,
            nested_level: 0,
            children: vec![],
            parent: TreeParent::new(),
            this: TreeParent::new(),
        };
        let rc_node = Rc::new(RefCell::new(node));
        let weak_link = Rc::downgrade(&rc_node);
        rc_node.borrow_mut().this = weak_link;
        rc_node
    }

    fn push(&mut self, item: T) {
        let child = TreeNode::new(item);
        child.borrow_mut().parent = self.this.clone();
        child.borrow_mut().nested_level = self.nested_level+1;
        self.children.push(child);
    }

    fn parent(&self) -> TreeParent<T> {
        self.parent.clone()
    }

    fn children(&self) -> &[TreeChild<T>] {
        &self.children
    }

    fn iter(&self) -> BreadthFirstSearchIterator<T> {
        BreadthFirstSearchIterator::new(self.this.clone())
    }
}

impl<T> Display for TreeNode<T>
    where T: Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.item)?;
        for item in self.children.iter() {
            for _i in 0..(**item).borrow().nested_level { write!(f, "| ").unwrap(); }
            write!(f, "{}", (**item).borrow())?;
        }
        Ok(())
    }
}

struct BreadthFirstSearchIterator<'a, T> {
    stack: VecDeque<TreeChild<T>>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> BreadthFirstSearchIterator<'a, T> {
    fn new(root: TreeParent<T>) -> Self {
        let mut stack = VecDeque::<TreeChild<T>>::new();
        if let Some(r) = root.upgrade() {
            stack.push_back(r);
        }
        Self {
            stack: stack,
            _marker: PhantomData,
        }
    }
}

impl<'a, T> Iterator for BreadthFirstSearchIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(rc_node) = self.stack.pop_front() {
            for child in (*rc_node).borrow().children() {
                self.stack.push_back(child.clone());
            }
            let item: *const T = &(*(*rc_node).borrow()).item;
            unsafe { return item.as_ref(); }
        } else {
            None
        }
    }
}







struct Index {
    name: String,
    size: Option<usize>,
    is_file: bool
}

impl Index {
    fn new_file(name: &str, size: usize) -> Self {
        Self {
            name: name.to_string(),
            size: Some(size),
            is_file: true,
        }
    }

    fn new_dir(name: &str) -> Self {
        Self {
            name: name.to_string(),
            size: None,
            is_file: false,
        }
    }
}

impl Display for Index {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_file {
            writeln!(f, "{} : {}", self.name, self.size.unwrap_or(0))?;
        } else {
            writeln!(f, "{}/", self.name)?;
        }
        Ok(())
    }
}




fn compute_size(root: TreeChild<Index>) -> usize {
    let mut sum: usize = (*root).borrow().item.size.unwrap_or(0);
    for child in (*root).borrow().children() {
        sum += compute_size(child.clone());
    }
    if !(*root).borrow().item.is_file {
        (*root).borrow_mut().item.size = Some(sum);
    }
    sum
}


fn main() {
    let args : Vec<_> = env::args().collect();
    let input_path = &args[1];
    let raw_input = fs::read(input_path).unwrap();
    let raw_string = String::from_utf8_lossy(&raw_input);
    let input : Vec<_> = raw_string.lines().collect();

    let mut root = TreeNode::<Index>::new(Index::new_dir(""));
    let mut current = root.clone();
    let mut program = input.iter().skip(1);
    
    let mut line_opt = program.next();
    if line_opt.is_none() { panic!("Program is empty"); }
    let mut line = line_opt.unwrap();
    let mut is_command = line.starts_with("$");
    loop {
        if line_opt.is_none() { break; }
        line = line_opt.unwrap();
        is_command = line.starts_with("$");
        if is_command {
            let mut command_line = line.split_whitespace().skip(1);
            if let Some(command) = command_line.next() {
                match command {
                    "cd" => {
                        let path = Path::new(command_line.next().unwrap());
                        for component in path.iter() {
                            match component.to_str().unwrap() {
                                "/" => {
                                    current = root.clone();
                                },
                                ".." => {
                                    let tmp = (*current).borrow().parent.upgrade().unwrap().clone();
                                    current = tmp;
                                },
                                name => {
                                    let mut tmp:Option<TreeChild<Index>> = None;
                                    for child in (*current).borrow().children() {
                                        if (**child).borrow().item.name == name {
                                            tmp = Some(child.clone());
                                            break;
                                        }
                                    }
                                    if tmp.is_some() { current = tmp.unwrap(); }
                                    else {
                                        panic!("Path not found");
                                    }
                                },
                            }
                        }
                    },
                    "ls" => {
                        loop {
                            line_opt = program.next();
                            if line_opt.is_none() { break; }
                            line = line_opt.unwrap();
                            is_command = line.starts_with("$");
                            if is_command { break }
                            else {
                                if line.starts_with("dir") {
                                    current.borrow_mut().push(Index::new_dir(line.split_whitespace().nth(1).unwrap()));
                                } else {
                                    let size = line.split_whitespace().nth(0).unwrap().parse::<usize>().unwrap();
                                    let name = line.split_whitespace().nth(1).unwrap();
                                    current.borrow_mut().push(Index::new_file(name, size));
                                }
                            }
                        }
                        continue;
                    },
                    _ => panic!("unknown command"),
                }
            } else {
                panic!("Command is empty !");
            }
        }
        line_opt = program.next();
    }

    println!("{}", (*root).borrow());



    compute_size(root.clone());

    let max_filesystem_size: usize = 70000000;
    let required_filesystem_size: usize = 30000000;
    let remaining_size: usize = max_filesystem_size - (*root).borrow().item.size.unwrap();
    let minimum_size_to_free: usize = required_filesystem_size - remaining_size;

    let min: usize = (*root).borrow().iter()
        .filter(|item| !item.is_file && item.size.unwrap() >= minimum_size_to_free)
        .map(|item| item.size.unwrap_or(0))
        .min().unwrap();

    println!("Min: {}", min);
}
