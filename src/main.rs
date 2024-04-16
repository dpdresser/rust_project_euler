pub mod functions;
use functions::*;
use std::time::Instant;

fn main() {
    // Problem 1
    // Add the sum of natural numbers below n that are multiples of 3 or 5
    let start = Instant::now();
    assert_eq!(type_of(&multiples_of_3_or_5(10)), "u64"); // Returns u64
    assert_eq!(multiples_of_3_or_5(49), 543);
    assert_eq!(multiples_of_3_or_5(1000), 233168);
    assert_eq!(multiples_of_3_or_5(8456), 16687353);
    assert_eq!(multiples_of_3_or_5(19564), 89301183);
    let duration = start.elapsed();
    println!("Problem 1 - Success! {:#?}", duration);

    // Problem 2
    // Sum of even Fibonnaci numbers below nth term
    let start = Instant::now();
    assert_eq!(type_of(&fibonnaci_sum(10)), "u64"); // Returns u64
    assert_eq!(fibonnaci_sum(10) % 2, 0); // Returns even number
    assert_eq!(fibonnaci_sum(10), 10);
    assert_eq!(fibonnaci_sum(34), 44);
    assert_eq!(fibonnaci_sum(60), 44);
    assert_eq!(fibonnaci_sum(1000), 798);
    assert_eq!(fibonnaci_sum(100000), 60696);
    assert_eq!(fibonnaci_sum(4000000), 4613732);
    let duration = start.elapsed();
    println!("Problem 2 - Success! {:#?}", duration);
}
