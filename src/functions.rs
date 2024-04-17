#![allow(dead_code)]

use std::any::type_name;

// General function to check type
pub fn type_of<T>(_: &T) -> String {
    format!("{}", type_name::<T>())
}

////////// Problem 1 //////////
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

////////// Problem 2 //////////
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

////////// Problem 3 //////////
// Simple algorithm to generate prime numbers up to and including n
// https://en.wikipedia.org/wiki/Sieve_of_Sundaram
pub fn prime_sieve_sundaram(n: usize) -> Vec<usize> {
    let k = (n - 1) / 2;
    let mut int_list = vec![1 as usize; k + 1];
    let mut primes: Vec<usize> = Vec::new();

    for i in 1..=k {
        let mut j = i;
        while i + j + 2 * i * j <= k {
            let index = i + j + 2 * i * j;
            int_list[index] = 0;
            j += 1;
        }
    }

    for i in 0..=k {
        if int_list[i] == 1 {
            primes.push(2 * i + 1);
        }
    }

    primes.insert(1, 2);

    primes
}

pub fn largest_prime_factor(n: usize, primes: &Vec<usize>) -> usize {
    let mut n = n;
    if n == 1 {
        return 1;
    }
    // For prime numbers 2, 3, 5, 7, ... divide n by each prime while
    // there is no remainder, then if n / prime == 1, return prime
    for prime in &primes[1..] {
        while n % prime == 0 {
            n = n / prime;
            if n == 1 {
                return *prime;
            }
        }
    }
    n
}