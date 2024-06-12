use std::collections::{BTreeMap, HashMap};
use crate::x_fast::*;



struct BTS {
    tree: BTreeMap<i32, i32>,
}

impl BTS {
    fn new() -> Self {
        BTS {
            tree: BTreeMap::new(), //zatial takto, potom nahradit za AVL stromy alebo RB stromami
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
        for key in keys.iter().rev() { //.rev() -> dvojita referencia (whut)
            if **key < k {
                return **key;
            }
        }
        -1
    }
}

pub struct YfastTrie {
    BST: HashMap<i32, BTS>,
    xtrie: XfastTrie,
    digits: i32,
}

impl YfastTrie {
    pub fn new(t: i32) -> Self {

        let mut digits = 0;
        
        let mut x = t;
        while x > 0 {
            x >>= 1;
            digits += 1;
        }

        YfastTrie {
            BST: HashMap::new(),
            xtrie: XfastTrie::new(t),
            digits
        }
    }

    pub fn find(&self, k: i32) -> i32 {
        let suc = self.xtrie.successor(k);
        let pre = self.xtrie.predecessor(k);
        
        if let Some(suc_node) = suc {
            if self.BST[&suc_node.borrow().num_prefix].tree.contains_key(&k) {
                return self.BST[&suc_node.borrow().num_prefix].tree[&k];
            }
        }


        if let Some(pre_node) = pre {
            if self.BST[&pre_node.borrow().num_prefix].tree.contains_key(&k) {
                return self.BST[&pre_node.borrow().num_prefix].tree[&k];
            }
        }

        -1
    }

    pub fn successor(&self, k: i32) -> i32 {
        let suc = self.xtrie.successor(k);
        let pre = self.xtrie.predecessor(k);
        let mut x = 2 << 2;
        let mut y = 2 << self.digits; // "nekonecno" / strasne velke cislo



        if let Some(suc_node) = suc { x = self.BST[&suc_node.borrow().num_prefix].successor(k); } // ( if let Some() -> testovanie ci successor alebo predecessor nie su None

        if let Some(pre_node) = pre { y = self.BST[&pre_node.borrow().num_prefix].successor(k); }

        if x < y { x } else { y }
    }

    pub fn predecessor(&self, k: i32) -> i32 {
        let suc = self.xtrie.successor(k);
        let pre = self.xtrie.predecessor(k);
        let mut x = -1;
        let mut y = -1;

        if let Some(suc_node) = suc { x = self.BST[&suc_node.borrow().num_prefix].predecessor(k); }

        if let Some(pre_node) = pre { y = self.BST[&pre_node.borrow().num_prefix].predecessor(k); }

        if x > y { x } else { y }
    }

    //toto som trosku odflakol, trebaspravit aby BBST mali medzi 1/2log(U) az 2log(U) vrcholov 
        // -> da sa to odsimulovat postupnim pridavanim vrcholov kedze mnozinu vrcholov ktore tam dat mam uz danu z F-blokov (vid clanok)

    pub fn insert(&mut self, k: i32, val: i32) { 
        let suc = self.xtrie.successor(k);

        if suc.is_none() {
            self.xtrie.insert(k);
            let mut BST = BTS::new();
            BST.tree.insert(k, val);
            self.BST.insert(k, BST);

        } else {
            let succ = suc.unwrap().borrow().num_prefix;
            self.BST.get_mut(&succ).unwrap().tree.insert(k, val);

        }
    }
}
