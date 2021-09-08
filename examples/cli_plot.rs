use std::io;
use textplots::{Chart, Plot, Shape};

fn main() {
    let mut input_line = String::new();
    println!("Please enter a positive integer");
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");
    let number: u128 = input_line.trim().parse().expect("Input not an integer");

    println!("Collatz steps for number {}", number);
    let mut steps = collatz::get_steps(number);
    steps.insert(0, number); // Include actual number
    println!("Collatz steps {:?}", steps);
    let data = steps.iter().enumerate().map(|(x,y)| (x as f32, *y as f32)).collect::<Vec<_>>();
    Chart::new(180, 60, 0.0, steps.len() as f32)
        .lineplot(&Shape::Lines(&data))
        .display();
}