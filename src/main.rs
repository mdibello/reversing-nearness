extern crate rand;

mod solution;
use solution::*;

fn main() {
    let mut s = Solution::new(5);
    println!("{}\n", s);
    s.mutate(100);
    println!("{}", s);
}
