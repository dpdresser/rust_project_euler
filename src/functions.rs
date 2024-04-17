use std::any::type_name;

// General function to check type

pub fn type_of<T>(_: &T) -> String {
    format!("{}", type_name::<T>())
}

// Problem 1

pub fn multiples_of_3_or_5(n: u64) -> u64 {
    // Using n as upper bound, add multiples of 3 or 5 to sum

    // Slow version with loop
    // let mut sum: u64 = 0;
    // for i in 0..n {
    //     if i % 3 == 0 || i % 5 == 0 {
    //         sum += i;
    //     }
    // }

    // Return sum
    // sum

    // Fast version, sum of arithmetic series less overlap
    let n = n - 1;
    let (p3, p5, p15) = (n / 3, n / 5, n / 15);
    3 * p3 * (p3 + 1) / 2 + 5 * p5 * (p5 + 1) / 2 - 15 * p15 * (p15 + 1) / 2
}

// Problem 2
pub fn fibonnaci_sum(n: u64) -> u64 {
    // Using n as upper bound, add even Fibonnaci numbers to sum
    let mut sum: u64 = 0;
    let (mut a, mut b):(u64, u64) = (0, 1);

    while a <= n {
        if a % 2 == 0 {
            sum += a;
        }
        let temp = a;
        a = b;
        b = temp + b;
    }

    // Return sum of even Fibonnaci numbers < n
    sum
}