use std::collections::VecDeque;
use std::fmt::Display;
use std::marker::PhantomData;

use self::internal::TreeNode;

mod internal {

    use std::rc::{Weak, Rc};
    use std::cell::RefCell;

    #[derive(Debug, Default)]
    pub struct TreeNode<T> {
        item: T,
        children: Vec<RcTreeNode<T>>,
        parent: WeakTreeNode<T>,
        this: WeakTreeNode<T>,
    }

    pub type WeakTreeNode<T> = Weak<RefCell<TreeNode<T>>>;
    pub type RcTreeNode<T> = Rc<RefCell<TreeNode<T>>>;

    impl<T> TreeNode<T> {
        pub fn new(item: T) -> RcTreeNode<T> {
            let node = Self {
                item,
                children: vec![],
                parent: WeakTreeNode::new(),
                this: WeakTreeNode::new(),
            };
            let rc_node: RcTreeNode<T> = Rc::new(RefCell::new(node));
            let weak_link: WeakTreeNode<T> = Rc::downgrade(&rc_node);
            rc_node.borrow_mut().this = weak_link;
            rc_node
        }

        pub fn get(&self) -> &T {
            &self.item
        }

        pub fn get_mut(&mut self) -> &mut T {
            &mut self.item
        }

        pub fn push(&mut self, item: T) {
            let child = Self::new(item);
            child.borrow_mut().parent = self.this.clone();
            self.children.push(child);
        }
    
        pub fn append(&mut self, node: RcTreeNode<T>) {
            self.children.push(node)
        }

        pub fn parent(&self) -> WeakTreeNode<T> {
            self.parent.clone()
        }

        pub fn children(&self) -> &[RcTreeNode<T>] {
            &self.children
        }

    }

}


#[derive(Debug, Default)]
pub struct Tree<T> {
    node: internal::RcTreeNode<T>,
}

impl<T> Tree<T> {

    pub fn new(item: T) -> Self {
        Self {
            node: internal::TreeNode::new(item),
        }
    }
    
    pub fn push(&mut self, item: T) {
        self.node.borrow_mut().push(item);
    }
    
    pub fn append(&mut self, tree: Tree<T>) {
        self.node.borrow_mut().append(tree.node);
    } 
    
    pub fn get(&self) -> &T {
        let tmp: *const T = (*self.node).borrow().get();
        unsafe { tmp.as_ref().unwrap() }
    }

    pub fn get_mut(&mut self) -> &mut T {
        let tmp: *mut T = self.node.borrow_mut().get_mut();
        unsafe { tmp.as_mut().unwrap() }
    }
    
    pub fn parent(&self) -> Option<Self> {
        if let Some(node) = (*self.node).borrow().parent().upgrade() {
            Some(Self { node: node.clone() })
        } else {
            None
        }
    }

    // ================== Const iterators ==================
    
    pub fn children_iter(&self) -> ChildrenNodesIter<T> {
        ChildrenNodesIter::new(self.node.clone())
    }
    
    pub fn parent_iter(&self) -> ParentNodesIter<T> {
        ParentNodesIter::new(self.node.clone())
    }

    pub fn nodes_breadth_first_iter(&self) -> NodesBreadthFirstIter<T> {
        NodesBreadthFirstIter::new(self.node.clone())
    }

    pub fn nodes_depth_first_iter(&self) -> NodesDepthFirstIter<T> {
        NodesDepthFirstIter::new(self.node.clone())
    }

    // ================== Const iterators ==================

    pub fn items_breadth_first_iter(&self) -> ItemsBreadthFirstIter<T> {
        ItemsBreadthFirstIter::new(self.node.clone())
    }
    
    pub fn items_depth_first_iter(&self) -> ItemsDepthFirstIter<T> {
        ItemsDepthFirstIter::new(self.node.clone())
    }

    // ================== Mutable iterators ==================
    
    pub fn items_breadth_first_iter_mut(&self) -> ItemsBreadthFirstIterMut<T> {
        ItemsBreadthFirstIterMut::new(self.node.clone())
    }
    
    pub fn items_depth_first_iter_mut(&self) -> ItemsDepthFirstIterMut<T> {
        ItemsDepthFirstIterMut::new(self.node.clone())
    }

}

impl<T> Tree<T>
where T: Display
{
    fn format(&self, nesting_level: usize, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for _ in 0..nesting_level {
            write!(f, "|  ")?;
        }
        writeln!(f, "{}", self.get())?;
        for tree in self.children_iter() { 
            tree.format(nesting_level+1, f)?
        }
        Ok(())
    }
}

impl<T> Clone for Tree<T> {
    fn clone(&self) -> Self {
        Self { node: self.node.clone() }
    }
}

impl<T> Display for Tree<T>
where T: Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.format(0, f)
    }
}

pub struct ChildrenNodesIter<'a, T> {
    iterator: std::slice::Iter<'a, internal::RcTreeNode<T>>,
}

impl<'a, T> ChildrenNodesIter<'a, T> {
    fn new(root: internal::RcTreeNode<T>) -> Self {
        let node_ptr: *const TreeNode<T> = (*root).as_ptr();
        let iterator = unsafe {
            let node = node_ptr.as_ref().unwrap();
            node.children().iter()
        };
        Self {
            iterator,
        }
    }
}

impl<'a, T> Iterator for ChildrenNodesIter<'a, T> {
    type Item = Tree<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.iterator.next() {
            Some(Tree{ node: node.clone() })
        } else {
            None
        }
    }
}

impl<'a, T> DoubleEndedIterator for ChildrenNodesIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.iterator.next_back() {
            Some(Tree{ node: node.clone() })
        } else {
            None
        }
    }
}

pub struct ParentNodesIter<'a, T> {
    list: VecDeque<Tree<T>>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> ParentNodesIter<'a, T> {
    fn new(root: internal::RcTreeNode<T>) -> Self {
        let mut list = VecDeque::<Tree<T>>::new();
        let mut current = (*root).borrow().parent().upgrade();
        while let Some(parent) = current {
            current = (*parent).borrow().parent().upgrade();
            list.push_back(Tree{ node: parent });
        }
        Self {
            list,
            _marker: PhantomData,
        }
    }
}

impl<'a, T> Iterator for ParentNodesIter<'a, T> {
    type Item = Tree<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }
}

pub struct ItemsDepthFirstIter<'a, T> {
    stack: VecDeque<internal::RcTreeNode<T>>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> ItemsDepthFirstIter<'a, T> {
    fn new(root: internal::RcTreeNode<T>) -> Self {
        let mut this = Self {
            stack: Default::default(),
            _marker: PhantomData,
        };
        this.stack.push_front(root);
        this
    }
}

impl<'a, T> Iterator for ItemsDepthFirstIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(rc_node) = self.stack.pop_front() {
            for child in (*rc_node).borrow().children().iter().rev() {
                self.stack.push_front(child.clone());
            }
            let item: *const T = (*rc_node).borrow().get();
            unsafe {
                // Need an unbound lifetime to get 'a
                return item.as_ref();
            }
        } else {
            None
        }
    }
}

pub struct ItemsBreadthFirstIter<'a, T> {
    stack: VecDeque<internal::RcTreeNode<T>>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> ItemsBreadthFirstIter<'a, T> {
    fn new(root: internal::RcTreeNode<T>) -> Self {
        let mut this = Self {
            stack: Default::default(),
            _marker: PhantomData,
        };
        this.stack.push_back(root);
        this
    }
}

impl<'a, T> Iterator for ItemsBreadthFirstIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(rc_node) = self.stack.pop_front() {
            for child in (*rc_node).borrow().children() {
                self.stack.push_back(child.clone());
            }
            let item: *const T = (*rc_node).borrow().get();
            unsafe {
                // Need an unbound lifetime to get 'a
                return item.as_ref();
            }
        } else {
            None
        }
    }
}

pub struct NodesDepthFirstIter<'a, T> {
    stack: VecDeque<Tree<T>>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> NodesDepthFirstIter<'a, T> {
    fn new(root: internal::RcTreeNode<T>) -> Self {
        let mut this = Self {
            stack: Default::default(),
            _marker: PhantomData,
        };
        this.stack.push_front( Tree{ node: root } );
        this
    }
}

impl<'a, T> Iterator for NodesDepthFirstIter<'a, T> {
    type Item = Tree<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(tree) = self.stack.pop_front() {
            let mut list = Vec::<Tree<T>>::new();
            for child in tree.children_iter() {
                list.push(child.clone());
            }
            for child in list.into_iter().rev() {
                self.stack.push_front(child);
            }
            Some(tree)
        } else {
            None
        }
    }
}

pub struct NodesBreadthFirstIter<'a, T> {
    stack: VecDeque<Tree<T>>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> NodesBreadthFirstIter<'a, T> {
    fn new(root: internal::RcTreeNode<T>) -> Self {
        let mut this = Self {
            stack: Default::default(),
            _marker: PhantomData,
        };
        this.stack.push_back( Tree{ node: root } );
        this
    }
}

impl<'a, T> Iterator for NodesBreadthFirstIter<'a, T> {
    type Item = Tree<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(tree) = self.stack.pop_front() {
            let mut list = Vec::<Tree<T>>::new();
            for child in tree.children_iter() {
                list.push(child.clone());
            }
            for child in list.into_iter().rev() {
                self.stack.push_back(child);
            }
            Some(tree)
        } else {
            None
        }
    }
}

pub struct ItemsDepthFirstIterMut<'a, T> {
    stack: VecDeque<internal::RcTreeNode<T>>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> ItemsDepthFirstIterMut<'a, T> {
    fn new(root: internal::RcTreeNode<T>) -> Self {
        let mut this = Self {
            stack: Default::default(),
            _marker: PhantomData,
        };
        this.stack.push_front(root);
        this
    }
}

impl<'a, T> Iterator for ItemsDepthFirstIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(rc_node) = self.stack.pop_front() {
            for child in (*rc_node).borrow().children().iter().rev() {
                self.stack.push_front(child.clone());
            }
            let item: *mut T = (*rc_node).borrow_mut().get_mut();
            unsafe {
                // Need an unbound lifetime to get 'a
                return item.as_mut();
            }
        } else {
            None
        }
    }
}

pub struct ItemsBreadthFirstIterMut<'a, T> {
    stack: VecDeque<internal::RcTreeNode<T>>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> ItemsBreadthFirstIterMut<'a, T> {
    fn new(root: internal::RcTreeNode<T>) -> Self {
        let mut this = Self {
            stack: Default::default(),
            _marker: PhantomData,
        };
        this.stack.push_back(root);
        this
    }
}

impl<'a, T> Iterator for ItemsBreadthFirstIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(rc_node) = self.stack.pop_front() {
            for child in (*rc_node).borrow().children() {
                self.stack.push_back(child.clone());
            }
            let item: *mut T = (*rc_node).borrow_mut().get_mut();
            unsafe {
                // Need an unbound lifetime to get 'a
                return item.as_mut();
            }
        } else {
            None
        }
    }
}
