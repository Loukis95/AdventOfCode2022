use std::collections::VecDeque;
use std::marker::PhantomData;

mod internal {

    use std::fmt::Display;
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

        pub fn parent(&self) -> WeakTreeNode<T> {
            self.parent.clone()
        }

        pub fn children(&self) -> &[RcTreeNode<T>] {
            &self.children
        }

    }

    impl<T> Display for TreeNode<T>
    where T: Display
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.item)
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
    
    pub fn get(&self) -> &T {
        let tmp: *const T = self.node.borrow().get();
        unsafe { tmp.as_ref().unwrap() }
    }

    pub fn get_mut(&mut self) -> &mut T {
        let tmp: *mut T = self.node.borrow_mut().get_mut();
        unsafe { tmp.as_mut().unwrap() }
    }
    
    pub fn parent(&self) -> Option<Self> {
        if let Some(node) = self.node.borrow().parent().upgrade() {
            Some(Self { node: node.clone() })
        } else {
            None
        }
    }

    // ================== Const iterators ==================

    pub fn children_iter(&self) -> ChildrenNodesIter<T> {
        todo!();
    }

    pub fn parent_iter(&self) -> ParentNodesIter<T> {
        todo!();
    }

    pub fn items_breadth_first_iter(&self) -> ItemsBreadthFirstIter<T> {
        ItemsBreadthFirstIter::new(self.node.clone())
    }
    
    pub fn items_depth_first_iter(&self) -> ItemsDepthFirstIter<T> {
        todo!()
    }

    pub fn nodes_breadth_first_iter(&self) -> NodesBreadthFirstIter<T> {
        todo!()
    }
    
    pub fn nodes_depth_first_iter(&self) -> NodesDepthFirstIter<T> {
        todo!()
    }

    // ================== Mutable iterators ==================

    pub fn children_iter_mut(&self) -> ChildrenNodesIterMut<T> {
        todo!()
    }

    pub fn parent_iter_mut(&self) -> ParentNodesIterMut<T> {
        todo!()
    }

    pub fn items_breadth_first_iter_mut(&self) -> ItemsBreadthFirstIterMut<T> {
        todo!()
    }
    
    pub fn items_depth_first_iter_mut(&self) -> ItemsDepthFirstIterMut<T> {
        todo!()
    }

    pub fn nodes_breadth_first_iter_mut(&self) -> NodesBreadthFirstIterMut<T> {
        todo!()
    }
    
    pub fn nodes_depth_first_iter_mut(&self) -> NodesDepthFirstIterMut<T> {
        todo!()
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
