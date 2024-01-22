
use bit_vector_A::*;
use succinct::Select1Support;
mod bit_vector_A;
mod first_structure;


fn main() {


    println!("Hello, world!");
    let a: [u64; 20] = [1, 2, 1, 1, 10, 10, 1, 8, 2, 5, 10, 9, 6, 10, 7, 9, 5, 6, 10, 1];


    let bitvec: BitVectorA = BitVectorA::create_from_array(&a, 10);

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
    println!("{}", bitvec);
    println!("rank_a: {:?}", bitvec.rank(1,1));
    println!("select_a: {:?}", bitvec.select.select1(1));
    println!("select_b: {:?}", bitvec.select_b(3));
    println!("rank_one: {:?}", bitvec.rank_one(9, 2));
    println!("select_one: {:?}", bitvec.select_one(1, 0)); //bug pri indexovani blokov od 0
    //println!("select_one: {:?}", bitvec.select_one(1, 3));

    println!("{}", (bitvec.select_a(1, 4).unwrap() as f64));
    println!("{}", (bitvec.select_a(1, 4).unwrap() as f64 / 10 as f64).floor());


}
