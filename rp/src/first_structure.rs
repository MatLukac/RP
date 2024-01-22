use succinct::*;

use crate::bit_vector_A::{CardinatlityVector, BitVectorA};


pub struct FirstStructure {
    cardinality_vector: CardinatlityVector,
    t: u64,
    eblocks: Vec<BitVector<u32>>
    //fblocks: Vec<BitVectorA> -> TODO: implementovat y-fast stromy (nie su v zadnej crate) 
    
}

impl FirstStructure {

    pub fn new(array: &[u64], t: u64) -> FirstStructure {
        
        let bitvec = BitVectorA::create_from_array(array, t);
        let eblocks: Vec<BitVectorA> = Vec::new(); //TODO: zistit ci v eblokoch sa uchovava realtivne alebo absolutne (ja implementujem ako relativne, ked tak zmenit)
        //let bits_in_f = (t as f64).log2().ceil();
        //let ecounter: u64 = 0;
        


        for i in 1..array.len() { 

            let indices: Vec<u64> = Vec::new();

        }

        
        FirstStructure {
            cardinality_vector: bitvec.create_cardinality_vector(),
            t,
            eblocks: {
                vec![]

            }
        }

    }

}