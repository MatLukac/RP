use std::cmp::min;

use crate::y_fast::YfastTrie;
use crate::bit_vector_a::{CardinatlityVector, BitVectorA};


pub struct FirstStructure {
    cardinality_vector: CardinatlityVector,
    t: u64,
    eblocks: Vec<Vec<u64>>, //todo: spravit lepsie kodovanie cisiel: tabulka ki*lg(t) bitov
    fblocks: Vec<YfastTrie>
 
    
}
#[allow(dead_code)]
impl FirstStructure {

    pub fn new(array: &[u64], t: u64) -> FirstStructure {
        
        let bitvec = BitVectorA::create_from_array(array, t);
        let mut fblocks: Vec<YfastTrie> = Vec::new();
        let mut eblocks =  bitvec.get_positions_ones_of_each_block();

        for block in &eblocks {
            let mut yfast = YfastTrie::new(bitvec.t as i32);
            let mut f = Vec::new(); 
            let log = i32::ilog2(block.len() as i32);
            let mut index = 0;
            while(index < block.len()) {
                f.push(block[index]);
                index += (log as  usize);
            }

            for x in  f.len()/2..f.len() {
                yfast.insert(block[x] as i32, x as i32);
            }
            
            for x in  0..f.len()/2 {
                yfast.insert(block[x] as i32, x as i32);
            }

            fblocks.push(yfast);
        }
        
        FirstStructure {
            cardinality_vector: bitvec.create_cardinality_vector(),
            t,
            eblocks,
            fblocks: fblocks
        }

    }


    pub fn rank_one(&mut self, position: u32, ith_block: u32) -> i32 {

        let pred = self.fblocks[ith_block as usize].predecessor(position as i32);
        let mut rank_f = 0;
        if(pred != -1) {
            rank_f = self.fblocks[ith_block as usize].find(pred);
        } else {
            rank_f = 0;
        } 

        let log = i32::ilog2(self.t as i32) as i32;
        let mut left = rank_f*(log );
        let mut right =left + log;

        while right - left > -1 {
            let mid = left + (right - left) / 2;
            let mid_val: u32 = self.eblocks[position as usize][mid as usize] as u32;

            if mid_val == position {
                return mid;
            } else if mid_val < position {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        min(left, right)
    }


    pub fn select_one(&mut self, position: u32, ith_block: u32) -> u32 {
        self.eblocks[ith_block as usize][position as usize] as u32
    }

    pub fn rank(&mut self, j: u32) -> u32 {
        let i = j/(self.t as u32);
        self.cardinality_vector.rank_b((i as u64)*self.t).unwrap() as u32 + self.rank_one((j - i*(self.t as u32)) as u32, i) as u32
    }

    pub fn select(&mut self, j: u32) -> u32 {
        let i = self.cardinality_vector.select_b(j as u64).unwrap();

        self.select_one((j - self.cardinality_vector.rank_b((i-1)*self.t).unwrap() as u32) as u32, i as u32) + ((i as u32-1)*self.t as u32)
    }
}