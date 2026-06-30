fn main() {
    println!("Welcome to the main function for testing the emptyfluffbignum library.");
    println!("BigIntArbitraryBase is a struct that allows you to create arbitrarily large signed integers with arbitrary bases from 2 to 2**32.");
    println!("Most stuff is not implemented yet. Please bear with us.");
    println!("The stuff that is implemented is not tested, and often incorrect; we are working on our testing framework.");
    println!("Currently you can create a BigIntArbitraryBase object from a string, make comparisons, do addition.");
    println!("Subtraction, multiplication, division, etc are not implemented yet.");
    println!("Math is performed primarly in u64, to catch all of the carries and etc.");
    println!("Digits are stored in an u32 and therefore the highest base is 4 294 967 295 (2^32 - 1).");
    println!("It is recommended to use base 1 000 000 000 for applications where the math mostly stays in base10.");

    println!("");
    use emptyfluffbignum::BigIntArbitraryBase;
    let mut num_a: BigIntArbitraryBase = BigIntArbitraryBase::new(10);
    let mut num_b: BigIntArbitraryBase = BigIntArbitraryBase::from_string("10r5_5_5_5".to_string());
    let mut num_c: BigIntArbitraryBase = BigIntArbitraryBase::from_string("10r-3_3_3_3".to_string());
    let mut num_d: BigIntArbitraryBase = BigIntArbitraryBase::from_string("10r1_1_1_1".to_string());
    let mut num_e: BigIntArbitraryBase = BigIntArbitraryBase::from_string("10r9_9_9_9".to_string());
    println!("");
    println!("Number created: {}", num_a.to_string());
    println!("Number created: {}", num_b.to_string());
    println!("Number created: {}", num_b.add(&num_b).to_string());
    println!("Number created: {}", num_b.add(&num_b).add(&num_b).to_string());

    if (num_a.add(&num_b)).compare_less(&num_a) {
        println!("num_a is less");
    } else {
        println!("{} is equal or greater than {}", num_a.add(&num_b).to_string(), &num_a.to_string());
    }

    println!("{} - {} = {}", num_e.to_string(), num_b.to_string(), num_e.multiply(&num_b).to_string());
}

mod emptyfluffbignum {

    pub struct BigIntArbitraryBase {
        values: Vec<u32>,
        radix: u32,
        negative_flag: bool,
    }

    impl BigIntArbitraryBase {
        pub fn new(radix: u32) -> Self {
            let mut result = Self {
                values: vec![0],
                radix: radix,
                negative_flag: false,
            };
            result
        }

        // create a maximum value radix number (radix of 2**32)
        fn new_with_max_radix() -> Self {
            return Self::new((2 as u32).pow(32));
        }

        /*
         * TODO: incomplete
         * Creates a BigIntArbitraryBase from a string.
         * Normal decimal digits are accepted.
         * Arbitrary radix numbers must be encoded by specifying the radix then listing the digits separated by underscores.
         * Each digit listed this way must be in base ten.
         * The generic format is ###rDD_DDD_DD_DDD - this specifies a radix of ### and has 4 digits in that radix.
         * Examples:
         * 32789723 - no base specified, assumed base ten
         * 10r7_2_8_9_2 - base 10 with 5 digits separated by underscores
         * 207r62_47_108_206 - base 207 with 4 digits separated by underscores
         * 207r-62_47_108_206 - as above, but negative
         */
         // function returns a 0 bigint on a fail
        pub fn from_string(whole_text: String) -> Self {

            let mut negative_flag: bool = false;

            // text must be ascii
            if !whole_text.is_ascii() {
                return Self::new(10);
            }

            // exactly 1 instance of 'r' is allowed
            if whole_text.chars().filter(|c| *c == 'r').count() != 1 {
                return Self::new(10);
            }

            // split at 'r', only exactly 2 non-empty strings is allowed
            let text_parts: Vec<String> = whole_text.split("r").filter(|s| s.chars().count() > 0).map(|s| s.to_string()).collect();
            if text_parts.len() != 2 {
                return Self::new(10);
            }
            let mut radix_text: String = text_parts[0].clone();
            println!("radix_text: {}", &radix_text);
            let mut numbers_text: String = text_parts[1].clone();

            // get the radix as a number
            let mut radix: u32 = 2;
            let mut radix: u32 = radix_text.parse().unwrap();
            println!("radix: {}", &radix);

            // print the number text
            println!("number_text: {}", numbers_text);

            // split the numbers part by '-'
            let mut numbers_text_split: Vec<String> = numbers_text.split("-").map(|c| c.to_string()).collect();

            // there must be only 1 or 2 elements in the list
            if numbers_text_split.len() > 2 {
                println!("Parse failed: Only one '-' allowed.");
                return Self::new(10);
            }

            // '-' is only allowed to exist at the front of the number
            if numbers_text_split.len() > 1 && numbers_text_split[0].len() > 0 {
                println!("Parse failed: Only allowed one '-' per number.");
                return Self::new(10);
            }

            // set negative_flag if appropriate
            if numbers_text_split.len() > 1 {
                println!("negative");
                negative_flag = true;
            }

            // convert the numbers text into a list of numbers
            let mut numbers: Vec<u32> = numbers_text.chars().filter(|c| *c != '-').collect::<String>().split("_").map(|s| s.parse().unwrap()).collect();

            // ensure they are all less than the radix
            if !numbers.iter().all(|i| i < &radix) {
                println!("One of the parts of the number is not smaller than the radix. Invalid format.");
                return Self::new(10);
            }

            // convert integer list to a number
            let mut result: BigIntArbitraryBase = Self::new(10);
            result.values.clear();
            for x in numbers.iter().rev() {
                result.values.push(*x);
            }

            result.negative_flag = negative_flag;

            result
        }

        pub fn to_string(self: &Self) -> String {
            println!("");
            println!("to_string() called. values.len(): {}", self.values.len());
            let mut result: String = String::new();
            result = self.radix.to_string() + "r";
            if self.is_negative() {
                println!("Printing a negative number.");
                result += "-";
            }
            for i in (0..self.values.len()).rev() {
                result += &self.values[i].to_string();
                if i > 0 {
                    result += "_";
                }
            }

            result
        }

        // zero if only one element in the array and it is the value zero
        // assume leading zeros will always be pre-cleared
        pub fn is_zero(self: &Self) -> bool {
            if self.values.len() != 1 {
                return false;
            }
            return self.values[0] == 0;
        }

        // ignores negative flag if actually zero
        pub fn is_negative(self: &Self) -> bool {
            if self.is_zero() {
                return false;
            }
            return self.negative_flag;
        }

        // true if not zero and not negative
        pub fn is_positive(self: &Self) -> bool {
            return !self.is_zero() && !self.is_negative();
        }

        /*
         * Convert $operand to the same radix as self.radix.
         * For practical reasons, this uses 2**32 as an intermediate radix.
         *
         * The general algorithm is as follows:
         * The multiplier starts at 1.
         * The running total is begun at 0, in base 2**32.
         * Add (self.values[0] * multiplier) to the running total (which is in base 2**32).
         * Multiply the multiplier by self.radix
         * Repeat for each digit of self.values[].
         */
         // TODO: incomplete
        pub fn as_new_radix(self: &Self, new_radix: &u32) -> Self {
            let mut result: Self = Self::new((2 as u32).pow(32));
            let mut multiplier: u32 = 1;
            for i in 0..self.values.len() {
                // result = result.add(self.values[i]);
            }

            result
        }

        /*
         * Remove any leading zeros from self.values.
         * self.values[0] will always remain untouched.
         */
        fn trim_leading_zeros(self: &mut Self) {
            while self.values.len() > 1 {
                if *self.values.last().unwrap() == 0 {
                    self.values.pop();
                } else {
                    break;
                }
            }
        }

        pub fn compare_less(self: &Self, operand: &Self) -> bool {

            // if radix mismatch, convert operand and compare that instead
            if self.radix != operand.radix {
                return self.compare_less(&operand.as_new_radix(&self.radix));
            }

            let mut sign_self: i32 = self.get_sign_as_int();
            let mut sign_operand: i32 = operand.get_sign_as_int();

            if sign_self < sign_operand {
                return true;
            }

            if sign_self > sign_operand {
                return false;
            }

            if sign_self == 0 && sign_self == 0 {
                return false;
            }

            // signs must be equal at this point
            // the previous comparisons mean this must be true
            if sign_self == 1 { // they're both positive
                self.compare_less_absolutes(operand);
            }

            // the only scenario that was not handled is that both numbers are negative
            return operand.compare_less_absolutes(self);
        }

        // assumes the radix is the same
        fn compare_less_absolutes(self: &Self, operand: &Self) -> bool {
            let mut result: bool = false;

            // cache the value lengths
            let mut len_self = self.values.len();
            let mut len_operand = operand.values.len();
            if len_self < len_operand {
                return true;
            }

            if len_self > len_operand {
                return false;
            }

            // assume that they have the same number of digits
            for i in (0..len_self.max(len_operand)).rev() {
                let mut num_self: u32 = self.values[i];
                let mut num_operand: u32 = operand.values[i];
                if num_self == num_operand {
                    continue;
                }
                if num_self < num_operand {
                    return true;
                }
                return false; // num_self must be larger, so function result is false
            }

            return false;
        }

        /*
         * return a value indicating the sign {1, 0, -1}
         */
        pub fn get_sign_as_int(self: &Self) -> i32 {
            if self.is_positive() {
                return 1;
            }
            if self.is_negative() {
                return -1;
            }
            return 0;
        }
    }

    impl BigIntArbitraryBase {
        /*
         * if the operand is in the wrong radix, convert it to self.radix then add
         * returns a number with self.radix
         *
         * checks sign then delegates to either subtract or add_absolute
         */
        pub fn add(self: &Self, operand: &Self) -> Self {
            // mismatched radix must be converted first
            if self.radix != operand.radix {
                return self.add(&operand.as_new_radix(&self.radix));
            }

            if operand.is_negative() {
                return self.subtract(operand);
            }

            if self.is_negative() {
                return operand.subtract(self);
            }

            return self.add_absolutes(operand);
        }

        // adds absolute value of two numbers together
        // assumes radix is the same
        pub fn add_absolutes(self: &Self, operand: &Self) -> Self {

            // determine the highest_index
            let mut result: Self = Self::new(self.radix);
            result.values.clear();
            let mut max_index = self.values.len().max(operand.values.len());
            let mut carry = 0;
            for i in 0..=max_index { // for each digit

                // determine digits for self and operand
                let mut num_self: u64 = 0;
                if i < self.values.len() {
                    num_self = self.values[i] as u64;
                }
                let mut num_operand: u64 = 0;
                if i < operand.values.len() {
                    num_operand = operand.values[i] as u64;
                }

                // add and split into result digit and carry
                let mut radix: u64 = self.radix as u64;
                let mut sum: u64 = num_self + num_operand + carry;
                let mut high_part: u64 = sum / radix;
                let mut low_part: u64 = sum - high_part * radix;
                carry = high_part;
                result.values.push(low_part as u32);
            }
            result.trim_leading_zeros();

            result
        }

        // TODO: incomplete
        pub fn subtract(self: &Self, operand: &Self) -> Self {

            let mut result: Self = Self::new(10);

            result
        }

        // does not check sign
        // assumes operand is less than self
        // assumes same radix
        pub fn subtract_absolutes(self: &Self, operand: &Self) -> Self {
            let mut result: Self = Self::new(10);
            result.values.clear();
            let mut radix: i64 = self.radix as i64;
            let mut max_index = self.values.len().max(operand.values.len());

            let mut borrow_flag: bool = false;
            for i in 0..=max_index {
                let mut num_self: i64 = 0;
                if i < self.values.len() {
                    num_self = self.values[i] as i64;
                }
                let mut num_operand: i64 = 0;
                if i < operand.values.len() {
                    num_operand = operand.values[i] as i64;
                }
                let mut temp_result = num_self - num_operand;
                if borrow_flag {
                    temp_result -= 1;
                }
                borrow_flag = false;
                if temp_result < 0 {
                    borrow_flag = true;
                    temp_result += radix;
                }
                result.values.push(temp_result as u32);
            }

            result.trim_leading_zeros();

            result
        }

        // TODO: incomplete
        pub fn multiply(self: &Self, operand: &Self) -> Self {
            let mut result: Self = Self::new(10);

            // ensure the radix is the same
            if self.radix != operand.radix {
                return self.multiply(&operand.as_new_radix(&self.radix));
            }

            let mut result: Self = Self::new(10);
            result.values.clear();

            /*
             * for each digit in self
             *     carry = 0
             *     for each digit in operand + 1
             *         temp = 0, push 0s for each i_self
             *         temp = self[i] * operand[j] + carry
             *         high_part = temp / radix
             *         low_part = temp - high_part * radix
             *         carry = carry = high_part
             *     result += temp
             * return result
             */

             let radix: u64 = self.radix as u64;
             for i_self in 0..self.values.len() {
                 let mut carry: u64 = 0;
                 let mut temp_sum: Self = Self::new(self.radix);
                 temp_sum.values.clear();
                 for i in 0..i_self {
                     temp_sum.values.push(0);
                 }
                 for i_operand in 0..=operand.values.len() {
                     let mut temp_product: u64 = 0;
                     let mut num_self: u64 = 0;
                     if i_self < self.values.len() {
                         num_self = self.values[i_self] as u64;
                     }
                     let mut num_operand: u64 = 0;
                     if i_operand < operand.values.len() {
                         num_operand = operand.values[i_operand] as u64;
                     }
                     temp_product = carry + num_self * num_operand;
                     let high_part: u64 = temp_product / radix;
                     let low_part: u64 = temp_product - high_part * radix;
                     temp_sum.values.push(low_part as u32);
                     carry = high_part;
                 }
                 result = result.add(&temp_sum);
             }




            result.negative_flag = self.negative_flag ^ operand.negative_flag;

            result
        }

        // TODO: incomplete
        pub fn divide(self: &Self, operand: &Self) -> Self {
            return Self::new(10);
        }
    }
}
