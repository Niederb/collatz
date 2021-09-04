use std::time::Instant;
use collatz;
use rayon::prelude::*;

fn main() {
    let end = 100_000_000;
    let mut r: Vec<u128> = (1..=end).collect();
    let now = Instant::now();
    r.iter_mut().for_each(|p| *p = collatz::count_steps(*p as u128));

    /*for (i, n) in r.enumerate() {
        steps[i + 1] = collatz::count_steps(n as u128);
    }*/
    println!("{}", now.elapsed().as_millis());
    
    /*for (i, s) in steps.iter().enumerate() {
        println!("{} has {} steps", i, s);
    }*/
}