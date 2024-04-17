#[cfg(test)]
mod tests {  
    use crate::functions::*;

    // Problem 1
    #[test]
    fn test_multiples_of_3_or_5() {
        assert_eq!(type_of(&multiples_of_3_or_5(10)), "u64");
        assert_eq!(multiples_of_3_or_5(49), 543);
        assert_eq!(multiples_of_3_or_5(1000), 233168);
        assert_eq!(multiples_of_3_or_5(8456), 16687353);
        assert_eq!(multiples_of_3_or_5(19564), 89301183);
    }

    // Problem 2
    #[test]
    fn test_fibonnaci_sum() {
        assert_eq!(type_of(&fibonnaci_sum(10)), "u64");
        assert_eq!(fibonnaci_sum(10) % 2, 0);
        assert_eq!(fibonnaci_sum(10), 10);
        assert_eq!(fibonnaci_sum(34), 44);
        assert_eq!(fibonnaci_sum(60), 44);
        assert_eq!(fibonnaci_sum(1000), 798);
        assert_eq!(fibonnaci_sum(100000), 60696);
        assert_eq!(fibonnaci_sum(4000000), 4613732);
    }

    // Problem 3
    #[test]
    fn test_largest_prime_factor() {
        let primes = prime_sieve_sundaram(7000);
        assert_eq!(type_of(&largest_prime_factor(2, &primes)), "usize");
        assert_eq!(largest_prime_factor(1, &primes), 1);
        assert_eq!(largest_prime_factor(2, &primes), 2);
        assert_eq!(largest_prime_factor(3, &primes), 3);
        assert_eq!(largest_prime_factor(5, &primes), 5);
        assert_eq!(largest_prime_factor(7, &primes), 7);
        assert_eq!(largest_prime_factor(8, &primes), 2);
        assert_eq!(largest_prime_factor(13195, &primes), 29);
        assert_eq!(largest_prime_factor(600851475143, &primes), 6857);
    }
}