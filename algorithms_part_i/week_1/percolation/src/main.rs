extern crate rand;
use rand::Rng;
use rand::distributions::{IndependentSample, Range};
mod union_find;
mod percolation;
use percolation::Percolator;
fn main() {

    let mut rng = rand::thread_rng();
    let random_index = Range::new(0, 19);
    let mut p = Percolator::new(20,20);
    p.open(0,0);
    for i in 1..6000{
        println!("{}",i);
        p.open(random_index.ind_sample(&mut rng), random_index.ind_sample(&mut rng));
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
