use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;


type RefNode = Rc<RefCell<Node>>; // 'lepsie' pointre v ruste -> pocita kolko krat je niekde jeho hodnota, ak nula hodnota sa uvolni

pub struct Node {
    pub level: i32, // hlbka
    pub num_prefix: i32, // cisla -> bin retazce -> doteraz najdeny prefix prehladavanim od korena
    pub left: Option<RefNode>,
    pub right: Option<RefNode>,
}

impl Node {
    pub fn new(level: i32, num_prefix: i32) -> Self {
        Node {
            level,
            num_prefix,
            left: None,
            right: None,
        }
    }
}

pub struct XfastTrie {
    digits: i32, 
    levels: Vec<HashMap<i32, RefNode>>, // snaha udrziavania node v leveloch
}

impl XfastTrie {
    pub fn new(t: i32) -> Self {
        let mut digits = 0;
        
        let mut x = t;
        while x > 0 {
            x >>= 1;
            digits += 1;
        }

        let mut levels = vec![HashMap::new(); (digits + 1) as usize];
        let root = Rc::new(RefCell::new(Node::new(0, 0)));
        levels[0].insert(0, root);

        XfastTrie { digits, levels }
    }

    pub fn most_left_list(&self, parent: RefNode) -> RefNode {
        let mut current = parent;
        loop {
            let next = {
                let parent_ref = current.borrow();

                if let Some(ref left) = parent_ref.left { Rc::clone(left) } 
                else if let Some(ref right) = parent_ref.right { Rc::clone(right) } 
                else { break; }

            };

            if next.borrow().level == self.digits { return next; }

            current = next;

        }
        current

    }



    pub fn right_most_list(&self, parent: RefNode) -> RefNode {
        let mut current = parent;
        loop {
            let next = {
                let parent_ref = current.borrow();  

                if let Some(ref right) = parent_ref.right { Rc::clone(right) } 
                else if let Some(ref left) = parent_ref.left { Rc::clone(left) } 
                else { break; }
            };

            if next.borrow().level == self.digits {
                return next;

            }

            current = next;


        }
        current

    }


    pub fn _find(&self, k: i32) -> Option<RefNode> {
        self.levels[self.digits as usize].get(&k).cloned()
    }

    pub fn successor(&self, k: i32) -> Option<RefNode> {
        let mut low = 0;
        let mut high = self.digits + 1;
        let mut tmp: Option<RefNode> = None;

        while high - low > 1 {
            let mid = (low + high) >> 1;
            let prefix = k >> (self.digits - mid);

            if let Some(node) = self.levels[mid as usize].get(&prefix) {
                low = mid;
                tmp = Some(Rc::clone(node));

            } else {
                high = mid;
            }
        }


        let tmp = tmp?;

        if tmp.borrow().level == 0 { return None;  }

        if tmp.borrow().level == self.digits { return Some(tmp); }

        let next_node = 
            if (k >> (self.digits - tmp.borrow().level - 1)) & 1 != 0 { tmp.borrow().right.clone() } 
            else { tmp.borrow().left.clone() }?;

        if next_node.borrow().num_prefix < k { return next_node.borrow().right.clone(); }

        Some(next_node)

    }

    pub fn predecessor(&self, k: i32) -> Option<RefNode> {
        let mut low = 0;
        let mut high = self.digits + 1;
        let mut tmp: Option<RefNode> = None;

        while high - low > 1 {
            let mid = (low + high) >> 1;
            let prefix = k >> (self.digits - mid);

            if let Some(node) = self.levels[mid as usize].get(&prefix) {
                low = mid;
                tmp = Some(Rc::clone(node)); 

            } else {
                high = mid;
            }
        }

        let tmp = tmp?;  //? -> cool sh*t

        if tmp.borrow().level == 0 { return None;  }

        if tmp.borrow().level == self.digits {  return Some(tmp); }

        let next_node = 
            if (k >> (self.digits - tmp.borrow().level - 1)) & 1 != 0 { tmp.borrow().right.clone() } 
            else { tmp.borrow().left.clone() }?;


        if next_node.borrow().num_prefix > k { return next_node.borrow().left.clone(); }

        Some(next_node)

    }

    pub fn insert(&mut self, k: i32) {
        let node = Rc::new(RefCell::new(Node::new(self.digits, k)));

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

        let mut ll = 1;
        let mut prefix;
        while ll != self.digits {
            prefix = k >> (self.digits - ll);
            if !self.levels[ll as usize].contains_key(&prefix) {
                let inter = Rc::new(RefCell::new(Node::new(ll, 0)));
                self.levels[ll as usize].insert(prefix, Rc::clone(&inter));
                if prefix & 1 != 0 { self.levels[(ll - 1) as usize][&(prefix >> 1)].borrow_mut().right = Some(Rc::clone(&inter)); } 
                else { self.levels[(ll - 1) as usize][&(prefix >> 1)].borrow_mut().left = Some(Rc::clone(&inter)); }
            }
            ll += 1;
        }


        self.levels[self.digits as usize].insert(k, Rc::clone(&node));

        if k & 1 != 0 { self.levels[(self.digits - 1) as usize][&(k >> 1)].borrow_mut().right = Some(Rc::clone(&node)); } 
        else {  self.levels[(self.digits - 1) as usize][&(k >> 1)].borrow_mut().left = Some(Rc::clone(&node)); }



        prefix = k;
        ll = self.digits - 1;

        while ll != 0 {
            prefix >>= 1;
            let mut entry = self.levels[ll as usize][&prefix].borrow_mut();
            if entry.left.is_none() { entry.left = Some(Rc::clone(&self.most_left_list(Rc::clone(entry.right.as_ref().unwrap()))));  } 
            else if entry.right.is_none() { entry.right = Some(Rc::clone(&self.right_most_list(Rc::clone(entry.left.as_ref().unwrap())))); }
            else {;}
            ll -= 1;
        }
        
        let mut root = self.levels[0][&0].borrow_mut();

        if root.left.is_none() { root.left = Some(Rc::clone(&self.most_left_list(Rc::clone(root.right.as_ref().unwrap())))); }

        if root.right.is_none() { root.right = Some(Rc::clone(&self.right_most_list(Rc::clone(root.left.as_ref().unwrap())))); }
    }
}
