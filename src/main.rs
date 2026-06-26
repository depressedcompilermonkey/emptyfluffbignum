fn main() {
    println!("Welcome to the main function for testing the emptyfluffbignum library.");
    println!("Most stuff is not implemented yet. Please bear with us.");
    println!("Currently you can create a BigIntArbitraryBase object.");
    println!("This is a signed integer with an arbitrary base.");
    println!("Digits are stored in an i32 and therefore the highest base is 4 294 967 295 (2^32 - 1).");
    println!("It is recommended to use base 1 000 000 000 for applications where the math mostly stays in base10.");
}

mod emptyfluffbignum {

    struct BigIntArbitraryBase {
        values: Vec<u32>,
        radix: u32,
        negative_flag: bool,
    }

    impl BigIntArbitraryBase {
        fn new(radix: u32) -> Self {
            let mut result = Self {
                values: vec![0],
                radix: radix,
                negative_flag: false,
            };
            result
        }

        // not zero if more than one element in values
        // assume negative zero does not exist
        // assume leading zeros will always be pre-cleared
        // assume that all code sets any zero value to clear the self.negative_flag
        // assume self.values will always hold at least 1 value
        // ignores the negative_flag entirely
        fn is_zero(self: &Self) -> bool {
            if (self.values.len() > 1) {
                return false;
            }
            return self.values[0] == 0;
        }

        // assume negative zero cannot exist
        fn is_negative(self: &Self) -> bool {
            if self.is_zero() {
                return false;
            }
            return self.negative_flag;
        }

        // true if not zero and not negative
        fn is_positive(self: &Self) -> bool {
            return !self.is_zero() && !self.is_negative();
        }

    }
}
