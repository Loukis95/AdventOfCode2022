use std::ops::{Deref, DerefMut};
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




trait Index {
    fn is_file(&self) -> bool;
    fn is_directory(&self) -> bool;
    fn name(&self) -> &str;
    fn size(&self) -> usize;
    fn set_parent(&mut self, parent: Option<Rc<Box<Directory>>>);
    fn as_any(&self) -> &dyn Any;
}

struct Directory {
    name: String,
    files: Vec<Rc<Box<dyn Index>>>,
    parent: Option<Rc<Box<Directory>>>,
}

struct File {
    name: String,
    size: usize,
    parent: Option<Rc<Box<Directory>>>,
}

impl Index for Directory {
    fn is_file(&self) -> bool {
        false
    }

    fn is_directory(&self) -> bool {
        true
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn size(&self) -> usize {
        self.files.iter().map(|index| index.size()).sum()
    }

    fn set_parent(&mut self, parent: Option<Rc<Box<Directory>>>) {
        self.parent = parent
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Index for File {
    fn is_file(&self) -> bool {
        true
    }

    fn is_directory(&self) -> bool {
        false
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn size(&self) -> usize {
        self.size
    }

    fn set_parent(&mut self, parent: Option<Rc<Box<Directory>>>) {
        self.parent = parent
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Directory {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            files: vec![],
            parent: None,
        }
    }

    fn push(self: &mut Rc<Box<Self>>, file: Rc<Box<dyn Index>>) {
        file.set_parent(Some(self.clone()));
        self.files.push(file);
    }

    fn iter(&self) -> impl Iterator<Item=&Rc<Box<dyn Index>>> {
        self.files.iter()
    }

    fn iter_mut(&mut self) -> impl Iterator<Item=&mut Rc<Box<dyn Index>>> {
        self.files.iter_mut()
    }
}

impl IntoIterator for Directory {
    type Item = Rc<Box<dyn Index>>;
    type IntoIter = <Vec<Rc<Box<dyn Index>>> as IntoIterator>::IntoIter; // so that you don't have to write std::vec::IntoIter, which nobody remembers anyway
  
    fn into_iter(self) -> Self::IntoIter {
        self.files.into_iter()
    }
}

impl Deref for Directory {
    type Target = [Rc<Box<dyn Index>>];
  
    fn deref(&self) -> &Self::Target {
        &self.files[..]
    }
}

impl DerefMut for Directory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.files[..]
    }
}

impl File {
    fn new(name: &str, size: usize) -> Self {
        Self {
            name: name.to_string(),
            size,
            parent: None,
        }
    }
}

fn main() {
    let args : Vec<_> = env::args().collect();
    let input_path = &args[1];
    let raw_input = fs::read(input_path).unwrap();
    let raw_string = String::from_utf8_lossy(&raw_input);
    let input : Vec<_> = raw_string.lines().collect();
    
    let mut root = Rc::new(Box::new(Directory::new("/")));
    let f1 = File::new("a", 5);
    root.push(Rc::new(Box::new(f1)));
    // let mut d1 = Directory::new("b/");
    // let f2 = File::new("a", 5);
    // d1.push(Rc::new(Box::new(f2)));
    // root.push(Rc::new(Box::new(d1)));
}
