#[warn(dead_code)]
//use fixedbitset::FixedBitSet;
use succinct::*;
use succinct::select::Select0Support;
use std::fmt;
#[allow(dead_code)]


pub struct BitVectorA{
    pub vec: BitVector<u32>,
    pub n: u64,
    pub t: u64,
    pub rank_select: BinSearchSelect<JacobsonRank<BitVector<u32>>>
}

pub struct CardinatlityVector{
    pub vec: BitVector<u32>,
    pub n: u64,
    pub t: u64,
    pub rank_select: BinSearchSelect<JacobsonRank<BitVector<u32>>>
}

impl fmt::Display for CardinatlityVector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        {
            
            for x in self.vec.iter() {
                if x { print!("1 "); }

                else { print!{"0 "} }; 
            }
        }
        print!("\n\n");
        write!(f, "\n{:?}\n lenght : {}\n values: {}\n", self.vec, self.n, self.t)
    }

}

impl CardinatlityVector{
    pub fn create_from_bit_vector_a(bitvec: &BitVectorA) -> CardinatlityVector {
        let mut vec: BitVector<u32> = BitVector::new();
        vec.resize(2*bitvec.n, false);
        let mut index: u64 = 0;
        for block in 1..(bitvec.n+1) {
            for _cardinality in 0..bitvec.cardinality_of_block(block) {
                vec.set_bit(index, true);
                index += 1;
            }
            vec.set_bit(index, false);
            index += 1;
           
        }
        let rank = JacobsonRank::new(vec.clone());
        CardinatlityVector {
            vec: vec,
            n: bitvec.n,
            t: bitvec.t,

            rank_select: BinSearchSelect::new(rank)
        }
    }
}

impl fmt::Display for BitVectorA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        {
            let mut t = 1;
            for x in self.vec.iter() {
                if x { print!("1 "); }

                else { print!{"0 "}; }

                if  t%self.t == 0  { print!("| "); }
                
                t += 1;
            }
            

        }
        print!("\n\n");
        write!(f, "\n{:?}\n lenght : {}\n values: {}\n", self.vec, self.n, self.t)
    }
}

#[warn(dead_code)]
impl BitVectorA {

    pub fn create_from_array(array: &[u64], t: u64) -> BitVectorA {
        //let mut v = FixedBitSet::with_capacity(array.len()*t as usize);
        let mut bitvec:BitVector<u32> = BitVector::new();
                bitvec.resize((array.len()*t as usize).try_into().unwrap(), false);
      


        for (i,x) in (0u64..).zip(array) {
            let z = ((x-1)*array.len() as u64 + i) as u64;
            println!("{z}");
            
            bitvec.set_bit(z, true);
            //v.insert(z as usize);
        }
        
        let rank = JacobsonRank::new(bitvec.clone());
        

        BitVectorA {
            vec: bitvec, 
            n: array.len() as u64,
            t,
            rank_select : BinSearchSelect::new(rank)
        }
    }

    
    pub(crate) fn rank_a(&self, c: u8, i: u64) -> Option<u64> {
        //panic if c is neither 1 or 0 and i is out of size

        if i==0 {return Option::Some(0);}

        if c == 1 { return Option::Some(self.rank_select.rank1(i-1)); }

        else { return Option::Some(self.rank_select.rank0(i-1)); }
    }
    
    pub fn select_a(&self, c: usize, i: u64) -> Option<u64> { 
        if i == 0 {return Option::None}

        if c == 1 { return self.rank_select.select1(i); }

        else { return self.rank_select.select0(i); }   
    }


    pub fn rank_b(&self, it: u64) -> Option<u64> { 
        self.rank_a(1, it)
    }
    
    pub fn select_b(&self, i: u64) -> Option<u64> { 

        self.select_a(1, i).map( |x: u64| -> u64 {(x as f64 / self.t as f64).floor() as u64})

    }

    pub fn rank(&self, c: u64, i: u64) -> Option<u64> {
        if i > self.n {return Option::None}

        let x = self.rank_a(1, (c-1)*self.n + i);
        let y = self.rank_a(1, (c-1)*self.n);

        if x.is_some() && y.is_some() {
            return Option::Some(x.unwrap() - y.unwrap())
        }

        else {
            return Option::None;
        }

    }
    #[allow(dead_code)]
    pub fn select(&self, c: u64, i: u64) -> Option<u64> {
        //TODO : checking the code for None values in select and rank
        let x = self.rank_a(1, (c-1)*self.n);
        x.map(|x: u64| -> u64 {self.select_a(1, x + i).unwrap()} ) //this is a problem
    
        //return self.select_a(1, self.rank_a(1, (c-1)*self.n) as u64 + i);
    }

    pub fn cardinality_of_block(&self, i: u64) -> u64 {
        self.rank_b(i*self.t).unwrap() - self.rank_b((i-1)*self.t).unwrap()
        
    }

    pub fn rank_one(&self, j: u64, i: u64) -> Option<u64> {
        self.rank_a(1, j + i*(self.t)).map( |x: u64| -> u64 { x - self.rank_a(1, i*self.t).unwrap() } )
    }

    pub fn select_one(&self, j: u64, i: u64) -> Option<u64> {
        
        let ret: Option<u64> = self.rank_a(1, (i-1)*self.t).map(|x: u64| -> u64 { self.select_a(1, j + x).unwrap() });  //toto je problem
        ret
        /* 
        if ret.is_some() && ret.unwrap() > i*self.t {
            return Option::None;
        }
        ret
        */
    }

    pub fn full_rank(&self, j: u64) -> Option<u64> {
        let i = (j as f64 / self.t as f64).floor() as u64;
        self.rank_b(i*self.t).map(|x| -> u64 { x + self.rank_one(j - i*self.t, i).unwrap() })
    }

    pub fn full_select(&self, j: u64) -> Option<u64> {
        let i = self.select_b(j);
        if i.is_none() { return Option::None; }
        self.rank_b((i.unwrap()-1)*self.t).map(|x:u64| -> u64 {self.select_one( j- x, i.unwrap()).unwrap() + (i.unwrap()-1)*self.t})
    }

    pub fn create_cardinality_vector(&self) -> CardinatlityVector {
        CardinatlityVector::create_from_bit_vector_a(self)
    }

    pub fn get_positions_ones_of_each_block(&self) -> Vec<Vec<u64>> {
        let mut ret = Vec::new();
        
        


        let mut block = 0;
        let mut block: Vec<u64> = Vec::new();
        for (bit,index ) in self.vec.iter().zip(0..(self.n*self.t)) {
            if(index % self.t == 0) {
                ret.push(block);
                block = Vec::new();
            }

            if(bit) { block.push(index%self.t)}  
        }


        ret
    }

}


#[allow(dead_code)]
impl CardinatlityVector{

    pub fn rank_b(&self, it: u64) -> Option<u64> {
        self.rank_select.select0(it/self.t).map(|x: u64| -> u64 {self.rank_select.rank1(x)})    
    }

    pub fn select_b(&self, i: u64) -> Option<u64> {
        self.rank_select.select1(i).map(|x: u64| -> u64 {self.rank_select.rank0(x)})
    }
    
}
