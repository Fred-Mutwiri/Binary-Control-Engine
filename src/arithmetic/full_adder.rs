
pub fn full_adder( a: u8, b: u8, carry_in: u8) -> ( u8, u8 ) {
    let sum =  a ^ b ^ carry_in;
    let carry_out = ( a & b) |  (a & carry_in) | (b & carry_in) ;

    (sum, carry_out)
}