//nie je to plne a korektne spravene :( 
use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;



type RefNodeAVLLLLLL = Rc<RefCell<AVLTreeNode>>;

#[derive(Debug, Clone)]
struct AVLTreeNode {
    key: i32,
    val: i32,
    height: i32,
    left: Option<RefNodeAVLLLLLL>,
    right: Option<RefNodeAVLLLLLL>,

}


impl AVLTreeNode {

    fn new(key: i32) -> RefNodeAVLLLLLL {
        Rc::new(RefCell::new(Self {
            key,
            val: 0,
            height: 1,
            left: None,
            right: None,
        }))
    }

    fn height(node: &Option<RefNodeAVLLLLLL>) -> i32 {
        if let Some(n) = node { n.borrow().height }
        else { 0 }
    }


    fn balance_factor(node: &Option<RefNodeAVLLLLLL>) -> i32 {
        if let Some(n) = node { AVLTreeNode::height(&n.borrow().left) - AVLTreeNode::height(&n.borrow().right) } 
        else { 0 }
    }


    fn rotate_left(node: RefNodeAVLLLLLL) -> RefNodeAVLLLLLL {

        let mut node_borrowed = node.borrow_mut();
        let right_child = node_borrowed.right.take().unwrap();
        let mut right_borrowed = right_child.borrow_mut();

        node_borrowed.right = right_borrowed.left.take();
        right_borrowed.left = Some(node.clone());
        node_borrowed.update_height();
        right_borrowed.update_height();
        drop(right_borrowed);
        drop(node_borrowed);
        right_child
    }

    fn rotate_right(node: RefNodeAVLLLLLL) -> RefNodeAVLLLLLL {

        let mut node_borrowed = node.borrow_mut();
        let left_child = node_borrowed.left.take().unwrap();
        let mut left_borrowed = left_child.borrow_mut();

        node_borrowed.left = left_borrowed.right.take();
        left_borrowed.right = Some(node.clone());
        node_borrowed.update_height();
        left_borrowed.update_height();
        drop(left_borrowed); 
        drop(node_borrowed);
        left_child
    }

    fn balance(node: RefNodeAVLLLLLL) -> RefNodeAVLLLLLL {
        node.borrow_mut().update_height();
        let balance_factor = AVLTreeNode::balance_factor(&Some(node.clone()));

        if balance_factor > 1 {
            if AVLTreeNode::balance_factor(&node.borrow().left) < 0 {
                let left_child = node.borrow().left.as_ref().unwrap().clone();
                node.borrow_mut().left = Some(AVLTreeNode::rotate_left(left_child));
            }

            return AVLTreeNode::rotate_right(node);
        }
        if balance_factor < -1 {
            if AVLTreeNode::balance_factor(&node.borrow().right) > 0 {
                let right_child = node.borrow().right.as_ref().unwrap().clone();
                node.borrow_mut().right = Some(AVLTreeNode::rotate_right(right_child));
            }

            return AVLTreeNode::rotate_left(node);
        }

        node
    }


    fn update_height(&mut self) {
        self.height = 1 + max(AVLTreeNode::height(&self.left), AVLTreeNode::height(&self.right));
    }

    fn insert(node: Option<RefNodeAVLLLLLL>, key: i32) -> RefNodeAVLLLLLL {
        if let Some(n) = node {
            if key < n.borrow().key {
                let left = AVLTreeNode::insert(n.borrow().left.clone(), key);
                n.borrow_mut().left = Some(left);
            } else {
                let right = AVLTreeNode::insert(n.borrow().right.clone(), key);
                n.borrow_mut().right = Some(right);
            }
            AVLTreeNode::balance(n)
        } else {
            AVLTreeNode::new(key)
        }
    }

    fn find(node: &Option<RefNodeAVLLLLLL>, key: i32) -> bool {
        if let Some(n) = node {
            if key == n.borrow().key { true } 
            
            else if key < n.borrow().key { AVLTreeNode::find(&n.borrow().left, key) } 

            else { AVLTreeNode::find(&n.borrow().right, key) }

        } else { false }
    }

    fn min_node(node: &RefNodeAVLLLLLL) -> RefNodeAVLLLLLL {
        if let  Some(ref left) =  node.borrow().left { AVLTreeNode::min_node(left) } 
        else { node.clone() }
    }

    

    fn predecessor(node: &Option<RefNodeAVLLLLLL>, key: i32, best: Option<i32>) -> Option<i32> {
        if let Some(n) = node {

            if key <= n.borrow().key { AVLTreeNode::predecessor(&n.borrow().left, key, best)  } 
            
            else { AVLTreeNode::predecessor(&n.borrow().right, key, Some(n.borrow().key)) }

        } else {
            best
        }
    }

    fn successor(node: &Option<RefNodeAVLLLLLL>, key: i32, best: Option<i32>) -> Option<i32> {
        if let Some(n) = node {

            if key >= n.borrow().key { AVLTreeNode::successor(&n.borrow().right, key, best) } 
            
            else { AVLTreeNode::successor(&n.borrow().left, key, Some(n.borrow().key)) }

        } else {
            best
        }
    }

    fn delete(node: Option<RefNodeAVLLLLLL>, key: i32) -> Option<RefNodeAVLLLLLL> {
        //TODO
        None
    }


}

#[derive(Clone)]
pub struct AVLTree {
    root: Option<RefNodeAVLLLLLL>,
}

impl AVLTree {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, key: i32) {
        self.root = Some(AVLTreeNode::insert(self.root.take(), key));
    }

    pub fn delete(&mut self, key: i32) {
        self.root = AVLTreeNode::delete(self.root.take(), key);
    }

    pub fn find(&self, key: i32) -> bool {
        AVLTreeNode::find(&self.root, key)
    }

    pub fn predecessor(&self, key: i32) -> Option<i32> {
        AVLTreeNode::predecessor(&self.root, key, None)
    }

    pub fn successor(&self, key: i32) -> Option<i32> {
        AVLTreeNode::successor(&self.root, key, None)
    }
}

