

pub fn half_adder(a: u8, b: u8) -> ( u8, u8 ) {
    let sum = a ^ b;
    let carry = a & b;

    (sum, carry)


}