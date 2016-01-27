mod union_find;
mod percolation;
use percolation::Percolator;
fn main() {
    let mut p = Percolator::new(20,20);
    p.open(0,0);
    println!("{}",p);
    println!("{:?}",p);
}
