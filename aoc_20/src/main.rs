struct Link<T> {
    item: T,
    next: Option<Rc<RefCell<T>>>,
    previous: Option<Rc<RefCell<T>>>,
    origin_next: Option<Rc<RefCell<T>>>,
    origin_previous: Option<Rc<RefCell<T>>>,
    this: Option<Rc<RefCell<T>>>,
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
    
    fn insert_before(&self, item: T) {
        let previous_link = self.previous.clone();
        let this = self.this.clone();
        let link = Self::new(item);
        link.next = Some(this.clone());
        link.previous = Some(previous_link.clone());
        link.origin_next = Some(this.clone());
        link.origin_previous = Some(previous_link.clone());
        let rc_link = Rc::new(RefCell::new(link));
        let link_clone = rc_link.clone();
        let borrowed_link = (*rc_link).borrow_mut();
        borrowed_link.this = Some(link_clone.clone());
        let borrowed_prev = (*previous_link).borrow_mut();
        borrowed_prev.next = Some(link_clone.clone());
        borrowed_prev.origin_next = Some(link_clone.clone());
        this.previous = Some(link_clone.clone());
        this.origin_previous = Some(link_clone);
    }
    
    fn insert_after(&self, item: T) {
        let next_link = self.next.clone();
        let this = self.this.clone();
        let link = Self::new(item);
        link.next = Some(next_link.clone());
        link.previous = Some(this.clone());
        link.origin_next = Some(next_link.clone());
        link.origin_previous = Some(this.clone());
        let rc_link = Rc::new(RefCell::new(link));
        let link_clone = rc_link.clone();
        let borrowed_link = (*rc_link).borrow_mut();
        borrowed_link.this = Some(link_clone.clone());
        let borrowed_next = (*next_link).borrow_mut();
        borrowed_prev.previous = Some(link_clone.clone());
        borrowed_prev.origin_previous = Some(link_clone.clone());
        this.next = Some(link_clone.clone());
        this.origin_next = Some(link_clone);
    }
}

struct LinkedList<T> {
    head: Option<Link<T>>,
    len: usize,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        Self {
            head: None,
            len: 0,
        }
    }
    
    fn push_back(&mut self, item: T) {
        if self.len == 0 {
            self.insert_first_item(item);
        } else {
            (*self.head).borrow_mut().insert_before(item);
            self.len += 1;
        }
    }
    
    fn insert_first_item(&mut self, item: T) {
        let link = Link::new(item);
        let rc_link = Rc::new(RefCell::new(link));
        let link_clone = rc_link.clone();
        let borrowed_link = (*rc_link).borrow_mut();
        borrowed_link.next = Some(link_clone.clone());
        borrowed_link.previous = Some(link_clone.clone());
        borrowed_link.origin_next = Some(link_clone.clone());
        borrowed_link.origin_previous = Some(link_clone.clone());
        borrowed_link.this = Some(link_clone);
        self.head = Some(rc_link);
        self.len = 1;
    }
}

struct LinkedListOriginIterator<'a, T> {
    
}

fn main() {
    println!("Hello, world!");
}
