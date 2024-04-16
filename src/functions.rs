use std::any::type_name;

// General function to check type

pub fn type_of<T>(_: &T) -> String {
    format!("{}", type_name::<T>())
}

// Problem 1

pub fn multiples_of_3_or_5(n: u64) -> u64 {
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

// Problem 2

fn fibonnaci(n: u64) -> u64 {
    // Return nth Fibonnaci number from sequence
    match n {
        0 => 0,
        1 => 1,
        _ => fibonnaci(n - 1) + fibonnaci(n - 2)
    }
}

pub fn fibonnaci_sum(n: u64) -> u64 {
    // Using n as upper bound, push even Fibonnaci numbers into even_numbers Vec
    let mut even_numbers: Vec<u64> = Vec::new();
    for i in 1..(n-1) {
        let fib_num = fibonnaci(i);
        if fib_num % 2 == 0 {
            even_numbers.push(fib_num);
        }
        if fib_num > n {
            break;
        }
    }

    // Return sum of even Fibonnaci numbers < n
    even_numbers.into_iter().sum()
}