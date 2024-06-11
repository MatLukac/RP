use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;


type RefNode = Rc<RefCell<Node>>;

struct Node {
    level: i32,
    key: i32,
    left: Option<RefNode>,
    right: Option<RefNode>,
}

impl Node {
    fn new(level: i32, key: i32) -> Self {
        Node {
            level,
            key,
            left: None,
            right: None,
        }
    }
}

struct XfastTrie {
    w: i32,
    hash_table: Vec<HashMap<i32, RefNode>>,
}

impl XfastTrie {
    fn new(U: i32) -> Self {
        let w = Self::get_digit_count(U);
        let mut hash_table = vec![HashMap::new(); (w + 1) as usize];
        let root = Rc::new(RefCell::new(Node::new(0, 0)));
        hash_table[0].insert(0, root);

        XfastTrie { w, hash_table }
    }

    fn get_digit_count(x: i32) -> i32 {
        let mut count = 0;
        let mut x = x;
        while x > 0 {
            x >>= 1;
            count += 1;
        }
        count
    }

    fn get_leftmost_leaf(&self, parent: RefNode) -> RefNode {
        let mut current = parent;
        loop {
            let next = {
                let parent_ref = current.borrow();
                if let Some(ref left) = parent_ref.left {
                    Rc::clone(left)
                } else if let Some(ref right) = parent_ref.right {
                    Rc::clone(right)
                } else {
                    break;
                }
            };
            if next.borrow().level == self.w {
                return next;
            }
            current = next;
        }
        current
    }

    fn get_rightmost_leaf(&self, parent: RefNode) -> RefNode {
        let mut current = parent;
        loop {
            let next = {
                let parent_ref = current.borrow();
                if let Some(ref right) = parent_ref.right {
                    Rc::clone(right)
                } else if let Some(ref left) = parent_ref.left {
                    Rc::clone(left)
                } else {
                    break;
                }
            };
            if next.borrow().level == self.w {
                return next;
            }
            current = next;
        }
        current
    }

    fn _find(&self, k: i32) -> Option<RefNode> {
        self.hash_table[self.w as usize].get(&k).cloned()
    }

    fn successor(&self, k: i32) -> Option<RefNode> {
        let mut low = 0;
        let mut high = self.w + 1;
        let mut tmp: Option<RefNode> = None;

        while high - low > 1 {
            let mid = (low + high) >> 1;
            let prefix = k >> (self.w - mid);
            if let Some(node) = self.hash_table[mid as usize].get(&prefix) {
                low = mid;
                tmp = Some(Rc::clone(node));
            } else {
                high = mid;
            }
        }

        let tmp = tmp?;

        if tmp.borrow().level == 0 {
            return None;
        }

        if tmp.borrow().level == self.w {
            return Some(tmp);
        }

        let next_node = if (k >> (self.w - tmp.borrow().level - 1)) & 1 != 0 {
            tmp.borrow().right.clone()
        } else {
            tmp.borrow().left.clone()
        }?;

        if next_node.borrow().key < k {
            return next_node.borrow().right.clone();
        }
        Some(next_node)
    }

    fn predecessor(&self, k: i32) -> Option<RefNode> {
        let mut low = 0;
        let mut high = self.w + 1;
        let mut tmp: Option<RefNode> = None;

        while high - low > 1 {
            let mid = (low + high) >> 1;
            let prefix = k >> (self.w - mid);
            if let Some(node) = self.hash_table[mid as usize].get(&prefix) {
                low = mid;
                tmp = Some(Rc::clone(node));
            } else {
                high = mid;
            }
        }

        let tmp = tmp?;

        if tmp.borrow().level == 0 {
            return None;
        }

        if tmp.borrow().level == self.w {
            return Some(tmp);
        }

        let next_node = if (k >> (self.w - tmp.borrow().level - 1)) & 1 != 0 {
            tmp.borrow().right.clone()
        } else {
            tmp.borrow().left.clone()
        }?;

        if next_node.borrow().key > k {
            return next_node.borrow().left.clone();
        }
        Some(next_node)
    }

    fn insert(&mut self, k: i32) {
        let node = Rc::new(RefCell::new(Node::new(self.w, k)));

        let pre = self.predecessor(k);
        let suc = self.successor(k);

        if let Some(pre) = pre {
            node.borrow_mut().right = pre.borrow().right.clone();
            pre.borrow_mut().right = Some(Rc::clone(&node));
            node.borrow_mut().left = Some(Rc::clone(&pre));
        }

        if let Some(suc) = suc {
            node.borrow_mut().left = suc.borrow().left.clone();
            suc.borrow_mut().left = Some(Rc::clone(&node));
            node.borrow_mut().right = Some(Rc::clone(&suc));
        }

        let mut lvl = 1;
        let mut prefix;
        while lvl != self.w {
            prefix = k >> (self.w - lvl);
            if !self.hash_table[lvl as usize].contains_key(&prefix) {
                let inter = Rc::new(RefCell::new(Node::new(lvl, 0)));
                self.hash_table[lvl as usize].insert(prefix, Rc::clone(&inter));
                if prefix & 1 != 0 {
                    self.hash_table[(lvl - 1) as usize][&(prefix >> 1)].borrow_mut().right = Some(Rc::clone(&inter));
                } else {
                    self.hash_table[(lvl - 1) as usize][&(prefix >> 1)].borrow_mut().left = Some(Rc::clone(&inter));
                }
            }
            lvl += 1;
        }
        self.hash_table[self.w as usize].insert(k, Rc::clone(&node));

        if k & 1 != 0 {
            self.hash_table[(self.w - 1) as usize][&(k >> 1)].borrow_mut().right = Some(Rc::clone(&node));
        } else {
            self.hash_table[(self.w - 1) as usize][&(k >> 1)].borrow_mut().left = Some(Rc::clone(&node));
        }

        prefix = k;
        lvl = self.w - 1;
        while lvl != 0 {
            prefix >>= 1;
            let mut entry = self.hash_table[lvl as usize][&prefix].borrow_mut();
            if entry.left.is_none() {
                entry.left = Some(Rc::clone(&self.get_leftmost_leaf(Rc::clone(entry.right.as_ref().unwrap()))));
            } else if entry.right.is_none() {
                entry.right = Some(Rc::clone(&self.get_rightmost_leaf(Rc::clone(entry.left.as_ref().unwrap()))));
            }
            lvl -= 1;
        }
        let mut root = self.hash_table[0][&0].borrow_mut();
        if root.left.is_none() {
            root.left = Some(Rc::clone(&self.get_leftmost_leaf(Rc::clone(root.right.as_ref().unwrap()))));
        }
        if root.right.is_none() {
            root.right = Some(Rc::clone(&self.get_rightmost_leaf(Rc::clone(root.left.as_ref().unwrap()))));
        }
    }
}


struct BinarySearchTree {
    tree: HashMap<i32, i32>,
}

impl BinarySearchTree {
    fn new() -> Self {
        BinarySearchTree {
            tree: HashMap::new(),
        }
    }

    fn successor(&self, k: i32) -> i32 {
        let mut keys: Vec<&i32> = self.tree.keys().collect();
        keys.sort();
        for key in keys {
            if *key > k {
                return *key;
            }
        }
        -1
    }

    fn predecessor(&self, k: i32) -> i32 {
        let mut keys: Vec<&i32> = self.tree.keys().collect();
        keys.sort();
        for key in keys.iter().rev() {
            if **key < k {
                return **key;
            }
        }
        -1
    }
}

pub struct YfastTrie {
    bst: HashMap<i32, BinarySearchTree>,
    xtrie: XfastTrie,
    w: i32,
}

impl YfastTrie {
    pub fn new(u: i32) -> Self {
        YfastTrie {
            bst: HashMap::new(),
            xtrie: XfastTrie::new(u),
            w: XfastTrie::get_digit_count(u),
        }
    }

    pub fn find(&self, k: i32) -> i32 {
        let suc = self.xtrie.successor(k);
        let pre = self.xtrie.predecessor(k);
        if let Some(suc_node) = suc {
            if self.bst[&suc_node.borrow().key].tree.contains_key(&k) {
                return self.bst[&suc_node.borrow().key].tree[&k];
            }
        }
        if let Some(pre_node) = pre {
            if self.bst[&pre_node.borrow().key].tree.contains_key(&k) {
                return self.bst[&pre_node.borrow().key].tree[&k];
            }
        }
        -1
    }

    pub fn successor(&self, k: i32) -> i32 {
        let suc = self.xtrie.successor(k);
        let pre = self.xtrie.predecessor(k);
        let mut x = 2 << 2;
        let mut y = 2 << self.w;
        if let Some(suc_node) = suc {
            x = self.bst[&suc_node.borrow().key].successor(k);
        }
        if let Some(pre_node) = pre {
            y = self.bst[&pre_node.borrow().key].successor(k);
        }
        if x < y { x } else { y }
    }

    pub fn predecessor(&self, k: i32) -> i32 {
        let suc = self.xtrie.successor(k);
        let pre = self.xtrie.predecessor(k);
        let mut x = -1;
        let mut y = -1;
        if let Some(suc_node) = suc {
            x = self.bst[&suc_node.borrow().key].predecessor(k);
        }
        if let Some(pre_node) = pre {
            y = self.bst[&pre_node.borrow().key].predecessor(k);
        }
        if x > y { x } else { y }
    }

    pub fn insert(&mut self, k: i32, val: i32) {
        let suc = self.xtrie.successor(k);
        if suc.is_none() {
            self.xtrie.insert(k);
            let mut bst = BinarySearchTree::new();
            bst.tree.insert(k, val);
            self.bst.insert(k, bst);
        } else {
            let succ = suc.unwrap().borrow().key;
            self.bst.get_mut(&succ).unwrap().tree.insert(k, val);
        }
    }
}
