use super::add::add;


// this code has had overflow issues when shifting the multiplicand left
// multiplication_overflows is failing.

// pub fn multiply( a: u8, b: u8 ) -> (u8, u8) {
//     let mut multiplicand = a;
//     let mut multiplier = b;

//     let mut result = 0u8;
//     let mut overflow = 0u8;

//     for _ in 0..8 {
//         if multiplier & 1 != 0 {
//             let ( new_result, carry ) = add(result, multiplicand);
//             result = new_result;

//             if carry != 0 {
//                 overflow = 1;
//             }
//         }

//         multiplicand <<= 1;
//         multiplier >>= 1;
//     }

//     (result, overflow)
// }



pub fn multiply(a: u8, b: u8 ) -> (u8, u8) {
    let mut multiplicand_low = a;
    let mut multiplicand_high = 0u8;

    let mut multiplier = b;

    let mut result_low = 0u8;
    let mut result_high = 0u8;

    while multiplier != 0 {

        // let's look at the lowest bit of the multiplier. 
        if multiplier & 1 != 0 {

            // add the low bytes
            let (new_low, carry) = add(result_low, multiplicand_low);
            result_low = new_low;

            //add the high bytes
            let (new_high, carry_high) = add(result_high, multiplicand_high);

            // Add the carry from the low-byte addition.
            let (new_high, carry_from_low) = add(new_high, carry);

            result_high = new_high;

            // For u8 x u8, result_high can never require more than 8 bits.
            // We don't need to store carry_high or carry_from_low for this operation.

            let _ = carry_high;
            let _ = carry_from_low;
        }

        // save the highest bit before shifting the low byte
        let carry = (multiplicand_low >> 7) & 1;

        //shift the low byte.
        multiplicand_low <<= 1;

        //shift the high byte and insert the bit that came out of the low byte.
        multiplicand_high =  (multiplicand_high << 1) | carry;

        //move to the next bit of the multiplier.
        multiplier >>= 1;
    }

    (result_low, result_high)
}




#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn multiplies_by_zero() {
        assert_eq!(multiply(10, 0), (0, 0));
    }

    #[test]
    fn multiplies_by_one() {
        assert_eq!(multiply(10, 1), (10, 0));
    }

    #[test]
    fn multiplies_small_values() {
        assert_eq!(multiply(6, 5), (30, 0));
    }

    #[test]
    fn multiplication_overflows_one_byte() {
        assert_eq!(multiply(20, 20), (144, 1));
    }

    #[test]
    fn multiplication_exactly_256() {
        assert_eq!(multiply(16, 16), (0, 1));
    }

    #[test]
    fn multiplication_128_by_two() {
        assert_eq!(multiply(128, 2), (0, 1));
    }

    #[test]
    fn multiplication_255_by_two() {
        assert_eq!(multiply(255, 2), (254, 1));
    }

    #[test]
    fn multiplication_maximum_values() {
        assert_eq!(multiply(255, 255), (1, 254));
    }
}