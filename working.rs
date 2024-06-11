use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

type NodeRef = Rc<RefCell<Node>>;

struct Node {
    level: i32,
    key: i32,
    left: Option<NodeRef>,
    right: Option<NodeRef>,
}

impl Node {
    fn new() -> Self {
        Node {
            level: -1,
            key: 0,
            left: None,
            right: None,
        }
    }
}

struct XfastTrie {
    w: i32,
    hash_table: Vec<HashMap<i32, NodeRef>>,
}

impl XfastTrie {
    fn new(U: i32) -> Self {
        let mut trie = XfastTrie {
            w: Self::get_digit_count(U),
            hash_table: Vec::new(),
        };
        trie.hash_table.resize_with((trie.w + 1) as usize, HashMap::new);
        let root = Rc::new(RefCell::new(Node {
            level: 0,
            ..Node::new()
        }));
        trie.hash_table[0].insert(0, root);
        trie
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

    fn left_child(x: i32) -> i32 {
        x << 1
    }

    fn right_child(x: i32) -> i32 {
        (x << 1) | 1
    }

    fn get_leftmost_leaf(&self, mut parent: NodeRef) -> NodeRef {
        while parent.borrow().level != self.w {
            let next = if let Some(ref left) = parent.borrow().left {
                left.clone()
            } else {
                parent.borrow().right.clone().unwrap()
            };
            parent = next;
        }
        parent
    }

    fn get_rightmost_leaf(&self, mut parent: NodeRef) -> NodeRef {
        while parent.borrow().level != self.w {
            let next = if let Some(ref right) = parent.borrow().right {
                right.clone()
            } else {
                parent.borrow().left.clone().unwrap()
            };
            parent = next;
        }
        parent
    }

    fn find(&self, k: i32) -> Option<NodeRef> {
        self.hash_table[self.w as usize].get(&k).cloned()
    }

    fn successor(&self, k: i32) -> Option<NodeRef> {
        let (mut low, mut high) = (0, self.w + 1);
        let mut tmp: Option<NodeRef> = None;
        while high - low > 1 {
            let mid = (low + high) >> 1;
            let prefix = k >> (self.w - mid);
            if self.hash_table[mid as usize].get(&prefix).is_none() {
                high = mid;
            } else {
                low = mid;
                tmp = self.hash_table[mid as usize].get(&prefix).cloned();
            }
        }
        tmp
    }

    fn predecessor(&self, k: i32) -> Option<NodeRef> {
        let (mut low, mut high) = (0, self.w + 1);
        let mut tmp: Option<NodeRef> = None;
        while high - low > 1 {
            let mid = (low + high) >> 1;
            let prefix = k >> (self.w - mid);
            if self.hash_table[mid as usize].get(&prefix).is_none() {
                high = mid;
            } else {
                low = mid;
                tmp = self.hash_table[mid as usize].get(&prefix).cloned();
            }
        }
        tmp
    }

    fn insert(&mut self, k: i32) {
        let mut low = 0;
        let mut high = self.w + 1;
        let mut tmp: Option<NodeRef> = None;
        while high - low > 1 {
            let mid = (low + high) >> 1;
            let prefix = k >> (self.w - mid);
            if self.hash_table[mid as usize].get(&prefix).is_none() {
                high = mid;
            } else {
                low = mid;
                tmp = self.hash_table[mid as usize].get(&prefix).cloned();
            }
        }
        if high != self.w + 1 {
            let new_node = Rc::new(RefCell::new(Node {
                level: high,
                key: k,
                ..Node::new()
            }));
            self.hash_table[high as usize].insert(k >> (self.w - high), new_node);
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
