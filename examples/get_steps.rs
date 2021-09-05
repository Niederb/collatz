use collatz;
use rayon::prelude::*;
use std::time::Instant;

fn main() {
    let end = 10_000_000;
    let mut r: Vec<u128> = (1..=end).collect();
    let now = Instant::now();
    r.par_iter_mut()
        .for_each(|p| *p = collatz::get_steps(*p as u128).len() as u128);

    println!("{}", now.elapsed().as_millis());
}
