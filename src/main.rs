use std::any::type_name;

fn main() {
    // Problem 1
    // Add the sum of natural numbers below n that are multiples of 3 or 5
    assert_eq!(type_of(&multiples_of_3_or_5(10)), "u64"); // Returns u64
    assert_eq!(multiples_of_3_or_5(49), 543);
    assert_eq!(multiples_of_3_or_5(1000), 233168);
    assert_eq!(multiples_of_3_or_5(8456), 16687353);
    assert_eq!(multiples_of_3_or_5(19564), 89301183);
    println!("Problem 1 - Success!");

    // Problem 2
    // Sum of even Fibonnaci numbers below nth term
    assert_eq!(type_of(&fibonnaci_sum(10)), "u64"); // Returns u64
    assert_eq!(fibonnaci_sum(10) % 2, 0); // Returns even number
    assert_eq!(fibonnaci_sum(10), 10);
    assert_eq!(fibonnaci_sum(34), 44);
    assert_eq!(fibonnaci_sum(60), 44);
    assert_eq!(fibonnaci_sum(1000), 798);
    assert_eq!(fibonnaci_sum(100000), 60696);
    assert_eq!(fibonnaci_sum(4000000), 4613732);
    println!("Problem 2 - Success!");
}

fn type_of<T>(_: &T) -> String {
    format!("{}", type_name::<T>())
}

fn multiples_of_3_or_5(n: u64) -> u64 {
    // Using n as upper bound, push multiples of 3 or 5 into numbers Vec
    let mut numbers: Vec<u64> = Vec::new();
    for i in 0..n {
        if i % 3 == 0 || i % 5 == 0 {
            numbers.push(i);
        }
    }

    // Return sum of numbers Vec
    numbers.into_iter().sum()
}

fn fibonnaci(n: u64) -> u64 {
    // Return nth Fibonnaci number from sequence
    match n {
        0 => 0,
        1 => 1,
        _ => fibonnaci(n - 1) + fibonnaci(n - 2)
    }
}

fn fibonnaci_sum(n: u64) -> u64 {
    let mut even_numbers: Vec<u64> = Vec::new();
    for i in 1..(n-1) {
        let fib_num = fibonnaci(i);
        if fib_num % 2 == 0 {
            even_numbers.push(fibonnaci(i));
        }
        if fib_num > n {
            break;
        }
    }
    even_numbers.into_iter().sum()
}
