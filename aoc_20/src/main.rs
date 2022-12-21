use std::{rc::Rc, cell::RefCell, marker::PhantomData, env, fs};

struct Link<T> {
    item: T,
    next: Option<Rc<RefCell<Link<T>>>>,
    previous: Option<Rc<RefCell<Link<T>>>>,
    origin_next: Option<Rc<RefCell<Link<T>>>>,
    origin_previous: Option<Rc<RefCell<Link<T>>>>,
    this: Option<Rc<RefCell<Link<T>>>>,
}

impl<T> Link<T> {
    fn new(item: T) -> Self {
        Self {
            item,
            next: None,
            previous: None,
            origin_next: None,
            origin_previous: None,
            this: None,
        }
    }
    
    fn get(&self) -> &T {
        &self.item
    }
    
    fn get_mut(&mut self) -> &mut T {
        &mut self.item
    }
    
    fn insert_before(&mut self, item: T) {
        let previous_link = self.previous.clone().unwrap();
        let this = self.this.clone().unwrap();
        let mut link = Self::new(item);
        link.next = Some(this.clone());
        link.previous = Some(previous_link.clone());
        link.origin_next = Some(this.clone());
        link.origin_previous = Some(previous_link.clone());
        let rc_link = Rc::new(RefCell::new(link));
        let link_clone = rc_link.clone();
        let mut borrowed_link = (*rc_link).borrow_mut();
        borrowed_link.this = Some(link_clone.clone());
        let borrowed_prev = (*previous_link).as_ptr();
        unsafe {
            let mut borrowed_prev: &mut Link<T> = borrowed_prev.as_mut().unwrap();
            borrowed_prev.next = Some(link_clone.clone());
            borrowed_prev.origin_next = Some(link_clone.clone());
        }
        self.previous = Some(link_clone.clone());
        self.origin_previous = Some(link_clone);
    }
    
    fn insert_after(&mut self, item: T) {
        let next_link = self.next.clone().unwrap();
        let this = self.this.clone().unwrap();
        let mut link = Self::new(item);
        link.next = Some(next_link.clone());
        link.previous = Some(this.clone());
        link.origin_next = Some(next_link.clone());
        link.origin_previous = Some(this.clone());
        let rc_link = Rc::new(RefCell::new(link));
        let link_clone = rc_link.clone();
        let mut borrowed_link = (*rc_link).borrow_mut();
        borrowed_link.this = Some(link_clone.clone());
        let mut borrowed_next = (*next_link).borrow_mut();
        borrowed_next.previous = Some(link_clone.clone());
        borrowed_next.origin_previous = Some(link_clone.clone());
        self.next = Some(link_clone.clone());
        self.origin_next = Some(link_clone);
    }

}

struct LinkedList<T> {
    head: Option<Rc<RefCell<Link<T>>>>,
    len: usize,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        Self {
            head: None,
            len: 0,
        }
    }

    fn iter<'a>(&'a mut self) -> LinkedListIterator<'a, T> {
        LinkedListIterator::<'a, T>::new(self.head.clone(), self.len)
    }

    fn origin_iter_mut<'a>(&'a mut self) -> LinkedListOriginIterator<'a, T> {
        LinkedListOriginIterator::<'a, T>::new(self.head.clone(), self.len)
    }
    
    fn push_back(&mut self, item: T) {
        if self.len == 0 {
            self.insert_first_item(item);
        } else {
            let head = self.head.clone().unwrap();
            (*head).borrow_mut().insert_before(item);
            self.len += 1;
        }
    }
    
    fn insert_first_item(&mut self, item: T) {
        let link = Link::new(item);
        let rc_link = Rc::new(RefCell::new(link));
        let link_clone = rc_link.clone();
        let mut borrowed_link = (*rc_link).borrow_mut();
        borrowed_link.next = Some(link_clone.clone());
        borrowed_link.previous = Some(link_clone.clone());
        borrowed_link.origin_next = Some(link_clone.clone());
        borrowed_link.origin_previous = Some(link_clone.clone());
        borrowed_link.this = Some(link_clone);
        self.head = Some(rc_link.clone());
        self.len = 1;
    }
}

#[derive(Clone)]
struct LinkedListOriginIterator<'a, T> {
    next: Option<Rc<RefCell<Link<T>>>>,
    len: usize,
    _phantom: PhantomData<&'a T>,
}

impl<'a, T> LinkedListOriginIterator<'a, T> {
    fn new(next: Option<Rc<RefCell<Link<T>>>>, len: usize) -> Self {
        Self { next, len, _phantom: PhantomData }
    }
}

impl<'a, T> Iterator for LinkedListOriginIterator<'a, T> {
    type Item = &'a Link<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 { return None; }
        if let Some(rc_link) = &self.next {
            let rc_link = rc_link.clone();
            self.next = (*rc_link).borrow().origin_next.clone();
            self.len -= 1;
            unsafe {
                let ret: &Link<T> = &(*rc_link).borrow();
                let ret: *const Link<T> = ret;
                return ret.as_ref();
            }
        }
        return None;
    }
}

#[derive(Clone)]
struct LinkedListIterator<'a, T> {
    next: Option<Rc<RefCell<Link<T>>>>,
    len: usize,
    _phantom: PhantomData<&'a T>,
}

impl<'a, T> LinkedListIterator<'a, T> {
    fn new(next: Option<Rc<RefCell<Link<T>>>>, len: usize) -> Self {
        Self { next, len, _phantom: PhantomData }
    }
}

impl<'a, T> Iterator for LinkedListIterator<'a, T> {
    type Item = &'a mut Link<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 { return None; }
        if let Some(rc_link) = &self.next {
            let rc_link = rc_link.clone();
            self.next = (*rc_link).borrow().next.clone();
            self.len -= 1;
            unsafe {
                let ret: &mut Link<T> = &mut (*rc_link).borrow_mut();
                let ret: *mut Link<T> = ret;
                return ret.as_mut();
            }
        }
        return None;
    }
}

fn main() {
    let args : Vec<_> = env::args().collect();
    let input_path = &args[1];
    let raw_input = fs::read(input_path).unwrap();
    let raw_string = String::from_utf8_lossy(&raw_input);
    let input : Vec<_> = raw_string.lines().collect();

    // Create the list
    let mut list = LinkedList::<isize>::new();

    // Fill the list with input file
    input.iter()
        .map(|line| line.parse::<isize>().unwrap())
        .for_each(|item| list.push_back(item));

    // Inspect the list
    // println!("List from input:");
    // list.iter().for_each(|item| print!("{} ; ", item.get()));
    // println!("");
    // println!("");

    // Get the list length
    let list_length: isize = list.len as isize;
    println!("List length: {}", list_length);

    // Apply the algorithm for part 1
    list.origin_iter_mut().for_each(|item| {
        let value = *item.get();
        let shift = value % (list_length-1);
        let this = item.this.clone().unwrap();
        if shift != 0 {
            // Remove item from where it was
            let this_prev = (this.clone()).borrow().previous.clone().unwrap();
            let this_next = (this.clone()).borrow().next.clone().unwrap();
            this_prev.borrow_mut().next = Some(this_next.clone());
            this_next.borrow_mut().previous = Some(this_prev.clone());
            // Find new insert position
            let mut insert_pos = this.clone();
            if shift > 0 {
                for _i in 0..shift {
                    let new_pos = (*insert_pos).borrow().next.as_ref().unwrap().clone();
                    insert_pos = new_pos;
                }
                // Insert item after the new position
                let new_prev = insert_pos.clone();
                let new_next = (*insert_pos.clone()).borrow().next.clone().unwrap();
                new_prev.borrow_mut().next = Some(this.clone());
                new_next.borrow_mut().previous = Some(this.clone());
                this.borrow_mut().next = Some(new_next);
                this.borrow_mut().previous = Some(new_prev);
            } else {
                for _i in shift..0 {
                    let new_pos = (*insert_pos).borrow().previous.as_ref().unwrap().clone();
                    insert_pos = new_pos;
                }
                // Insert item before the new position
                let new_next = insert_pos.clone();
                let new_prev = (*insert_pos.clone()).borrow().previous.clone().unwrap();
                new_prev.borrow_mut().next = Some(this.clone());
                new_next.borrow_mut().previous = Some(this.clone());
                this.borrow_mut().next = Some(new_next);
                this.borrow_mut().previous = Some(new_prev);
            }
            // Inspect
            // println!("Handling value {}", value);
            // let mut current = Some(this.clone());
            // for _ in 0..list_length {
            //     let link = current.clone().unwrap();
            //     let value: isize = *(*link).borrow().get();
            //     current = (*link).borrow().next.clone();
            //     print!("{} ; ", value);
            // }
            // println!("");
        } else {
            // println!("{} doesn't move", value);
            // let mut current = Some(this.clone());
            // for _ in 0..list_length {
            //     let link = current.clone().unwrap();
            //     let value: isize = *(*link).borrow().get();
            //     current = (*link).borrow().next.clone();
            //     print!("{} ; ", value);
            // }
            // println!("");
        }
    });
    println!("");

    // Inspect the list
    // println!("List after reordering:");
    // list.iter().for_each(|item| print!("{} ; ", item.get()));
    // println!("");
    // println!("");
    
    // Iterate over new positions now
    println!("Items at:");
    let sum: isize = list.iter()
    .map(|item| item.get())
        .cycle()
        .skip_while(|item| item != &&0isize)
        .step_by(1000 % list_length as usize)
        .take(4)
        .skip(1)
        .enumerate()
        .inspect(|(n, item)| println!("{}: {}", (n+1)*1000, item))
        .map(|(_, item)| item)
        .sum();

    println!("");
    println!("Result: {}", sum);
}
