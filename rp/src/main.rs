

use bit_vector_a::*;
use succinct::Select1Support;
use y_fast::*;
use x_fast::*;
use alv_tree::AVLTree;

mod bit_vector_a;
mod first_structure;
mod y_fast;
mod x_fast;
mod alv_tree;



fn main() {


    /* println!("Hello, world!");
    let a: [u64; 20] = [1, 2, 1, 1, 10, 10, 1, 8, 2, 5, 10, 9, 6, 10, 7, 9, 5, 6, 10, 1];


    let bitvec: BitVectorA = BitVectorA::create_from_array(&a, 10);
 */
    /* 


    println!("{}", bitvec);
    println!("{:?}", bitvec.rank_a(1,1));
    println!("{:?}", bitvec.select_a(1,7));
    println!("{:?}", bitvec.select(1,7));
    println!("{:?}", bitvec.jr.rank1(2));
    println!("{:?}", bitvec.select.select1(35));
    println!("{:?}", bitvec.rank(1,29));

    let mut count = 0;
    for index in 1..21 {
        println!("{:?}", bitvec.cardinality_of_block(index));
        count += bitvec.cardinality_of_block(index);
    }   
    println!("cardinality {}", count);

    println!("{:?}", bitvec.rank_b(4*bitvec.t));
    println!("rank_one: {:?}", bitvec.rank_a(1, 1));
    println!("select_one: {:?}", bitvec.select_one(1, 3));

    let card = bitvec.create_cardinality_vector();
    println!("{}", card);

    */
    /* println!("{}", bitvec);
    println!("rank_a: {:?}", bitvec.rank(1,1));
    println!("select_a: {:?}", bitvec.select.select1(1));
    println!("select_b: {:?}", bitvec.select_b(3));
    println!("rank_one: {:?}", bitvec.rank_one(9, 2));
    println!("select_one: {:?}", bitvec.select_one(1, 0)); //bug pri indexovani blokov od 0
    //println!("select_one: {:?}", bitvec.select_one(1, 3));

    println!("{}", (bitvec.select_a(1, 4).unwrap() as f64));
    println!("{}", (bitvec.select_a(1, 4).unwrap() as f64 / 10 as f64).floor()); */


    let mut trie = YfastTrie::new(1 << 5);
    println!("insert 1, 5, 11, 12");
    trie.insert(5, 2);
    trie.insert(11, 2);
    trie.insert(12, 2);
    trie.insert(1, 2);

    println!("find 5: {}", trie.find(5));
    println!("find 1: {}", trie.find(1));
    println!("find 11: {}", trie.find(11));
    println!("find 12: {}", trie.find(12));


    println!("Successor of key 2:");
    let tmp = trie.successor(2);
    if tmp != i32::MAX {
        println!("{}\nvalue stored = {}", tmp, trie.find(tmp));
    }

    println!("Predecessor of key 13:");
    let tmp = trie.predecessor(13);
    if tmp != -1 {
        println!("{}\nvalue stored = {}", tmp, trie.find(tmp));
    }


/*  let mut tree = AVLTree::new();
        
 // Test insertion
 tree.insert(10);
 tree.insert(20);
 tree.insert(5);
 tree.insert(6);
 tree.insert(15);

 // Check the in-order traversal of the tree
 //assert_eq!(tree.inorder(), vec![5, 6, 10, 15, 20]);

 // Test find operation
 assert!(tree.find(10));
 assert!(!tree.find(100));

 // Test predecessor and successor
 assert_eq!(tree.predecessor(15), Some(10));
 assert_eq!(tree.successor(15), Some(20));

 // Test deletion
 tree.delete(10);
 assert!(!tree.find(10)); */
}

    

        

    


