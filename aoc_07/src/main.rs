use std::borrow::BorrowMut;
use std::cell::{RefCell, RefMut};
use std::ops::{Deref, DerefMut};
use std::{env, fs};
use std::rc::Rc;

trait Index {
    fn is_file(&self) -> bool;
    fn is_directory(&self) -> bool;
    fn name(&self) -> &str;
    fn size(&self) -> usize;
}

struct Directory {
    name: String,
    files: Vec<Rc<Box<dyn Index>>>,
    parent: Option<Rc<Box<Directory>>>,
}

struct File {
    name: String,
    size: usize,
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
}

impl Directory {
    fn new(name: &str, parent: Option<Rc<Box<Directory>>>) -> Rc<Box<Self>> {
        Rc::new(Box::new(Self {
            name: name.to_string(),
            files: vec![],
            parent,
        }))
    }

    fn push(&mut self, file: Rc<Box<dyn Index>>) {
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
    fn new(name: &str, size: usize) -> Rc<Box<Self>> {
        Rc::new(Box::new(Self {
            name: name.to_string(),
            size,
        }))
    }
}

fn main() {
    let args : Vec<_> = env::args().collect();
    let input_path = &args[1];
    let raw_input = fs::read(input_path).unwrap();
    let raw_string = String::from_utf8_lossy(&raw_input);
    let input : Vec<_> = raw_string.lines().collect();
    
    let mut root = Directory::new("/", None);
    let f1 = File::new("a", 5);
    root.push(f1);
}
