use super::full_adder::full_adder;

pub fn add( a: u8, b: u8 ) -> ( u8, u8 ) {
    let mut result = 0;
    let mut carry = 0;

    //because this is a u8
    for position in 0..8 {
        let a_bit = ( a >> position ) & 1;
        let b_bit = ( b >> position ) & 1;

        let (sum, carry_in) = full_adder(a_bit, b_bit, carry );

        result |= sum << position;
        carry = carry_in
    }

    (result, carry)

}