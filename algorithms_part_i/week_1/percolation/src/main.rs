extern crate rand;
use rand::Rng;
use rand::distributions::{IndependentSample, Range};
mod union_find;
mod percolation;
use percolation::Percolator;
fn main() {

    let mut rng = rand::thread_rng();
    let random_index = Range::new(0, 20);
    let mut p = Percolator::new(20,20);
    p.open(0,0);
    for i in 1..6000{
        let row = random_index.ind_sample(&mut rng);
        let col = random_index.ind_sample(&mut rng);
        println!("run:{} row:{} col:{}",i, row, col);

        p.open(row, col);
        if p.is_percolated(){
            println!("{}",p);
            println!("{:?}",p);
            break;
        }
    }

    if !p.is_percolated(){
        println!("did not connect after looping");
        println!("{}",p);
        println!("{:?}",p);
    }

}
