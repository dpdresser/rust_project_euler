#[cfg(test)]
mod tests {  
    use crate::functions::*;
    
    #[test]
    fn test_multiples_of_3_or_5() {
        assert_eq!(type_of(&multiples_of_3_or_5(10)), "u64"); // Returns u64
        assert_eq!(multiples_of_3_or_5(49), 543);
        assert_eq!(multiples_of_3_or_5(1000), 233168);
        assert_eq!(multiples_of_3_or_5(8456), 16687353);
        assert_eq!(multiples_of_3_or_5(19564), 89301183);
    }

    #[test]
    fn test_fibonnaci_sum() {
        assert_eq!(type_of(&fibonnaci_sum(10)), "u64"); // Returns u64
        assert_eq!(fibonnaci_sum(10) % 2, 0); // Returns even number
        assert_eq!(fibonnaci_sum(10), 10);
        assert_eq!(fibonnaci_sum(34), 44);
        assert_eq!(fibonnaci_sum(60), 44);
        assert_eq!(fibonnaci_sum(1000), 798);
        assert_eq!(fibonnaci_sum(100000), 60696);
        assert_eq!(fibonnaci_sum(4000000), 4613732);
    }
}