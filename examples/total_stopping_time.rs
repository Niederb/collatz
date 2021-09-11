use collatz;
use rayon::prelude::*;
use std::time::Instant;

fn main() {
    let end = 10_000_000;

    let mut s: Vec<u128> = (1..=end).collect();
    let now = Instant::now();
    s.iter_mut()
        .for_each(|p| *p = collatz::total_stopping_time(*p as u128) as u128);
    println!("Time single threaded: {}ms", now.elapsed().as_millis());

    let mut r: Vec<u128> = (1..=end).collect();
    let now = Instant::now();
    r.par_iter_mut()
        .for_each(|p| *p = collatz::total_stopping_time(*p as u128) as u128);
    println!("Time with rayon: {}ms", now.elapsed().as_millis());

    // Verify the results are the same
    for (i, val1) in s.iter().enumerate() {
        if *val1 != r[i] {
            panic!("Values should be the same!");
        }
    }
}
