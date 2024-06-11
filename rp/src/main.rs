
use bit_vector_a::*;
use succinct::Select1Support;
use y_fast::*;

mod bit_vector_a;
mod first_structure;
mod y_fast;



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

 
       /*  let mut yfast = YFastTrie::new();
        yfast.insert(42);
        yfast.insert(100);
        yfast.insert(25);
        println!("Inserted 42, 100, and 25 into Y-fast trie");

        println!("find: {}", yfast.find(90));


    
        let pred = yfast.predecessor(42);
        let succ = yfast.successor(42);
    
        println!("Predecessor of 42: {:?}", pred);
        println!("Successor of 42: {:?}", succ);
    
        let pred_100 = yfast.predecessor(100);
        let succ_25 = yfast.successor(25);
    
        println!("Predecessor of 100: {:?}", pred_100);
        println!("Successor of 25: {:?}", succ_25); */

 


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

        

    

}
