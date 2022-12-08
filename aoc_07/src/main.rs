use std::borrow::Borrow;
use std::fmt::Display;
use std::ops::{Deref, DerefMut};
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
        self.children.push(child);
    }

    fn parent(&self) -> TreeParent<T> {
        self.parent.clone()
    }

    fn children(&self) -> &[TreeChild<T>] {
        &self.children
    }

    fn iter(&self) -> TreeChildrenIterator<'a, T> {
        
    }
}

impl<T> Display for TreeNode<T>
    where T: Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.item)?;
        for item in self.children.iter() {
            write!(f, "| {}", (**item).borrow())?;
        }
        Ok(())
    }
}

struct TreeChildrenIterator<'a, T> {
    current: TreeParent<T>,
    it: Option<std::slice::Iter<'a, TreeChild<T>>>,
}

impl<'a, T> TreeChildrenIterator<'a, T> {
    fn new(current: TreeParent<T>) -> Self {
        Self {
            current,
            it: None,
        }
    }
}

impl<'a, T> Iterator for TreeChildrenIterator<'a, T> {
    type Item = TreeChild<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.it.is_none() {
            if let Some(node) = self.current.upgrade() {
                self.it = Some((*node).borrow().children.iter());
            } else {
                return None;
            }
        }
        if let Some(it) = self.it {
            return it.next();
        }
    }
}




struct Index {
    name: String,
    size: usize,
    is_file: bool
}

impl Index {
    fn new_file(name: &str, size: usize) -> Self {
        Self {
            name: name.to_string(),
            size,
            is_file: true,
        }
    }

    fn new_dir(name: &str) -> Self {
        Self {
            name: name.to_string(),
            size: 0,
            is_file: false,
        }
    }
}

impl Display for Index {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_file {
            writeln!(f, "{} : {}", self.name, self.size)?;
        } else {
            writeln!(f, "{}", self.name)?;
        }
        Ok(())
    }
}

fn main() {
    let args : Vec<_> = env::args().collect();
    let input_path = &args[1];
    let raw_input = fs::read(input_path).unwrap();
    let raw_string = String::from_utf8_lossy(&raw_input);
    let input : Vec<_> = raw_string.lines().collect();
    
    let root = TreeNode::<Index>::new(Index::new_dir("/"));
    root.borrow_mut().push(Index::new_file("a", 5));
    root.borrow_mut().push(Index::new_dir("dir/"));

    if let Some(dir) = (*root).borrow().children.iter().find(|item| {
        if (***item).borrow().item.name == "dir/" {
            true
        } else {
            false
        }
    })
    {
        dir.borrow_mut().push(Index::new_file("b", 5));
    }

    print!("{}", (*root).borrow());
}
