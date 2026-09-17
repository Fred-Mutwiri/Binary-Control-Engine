use crate::arithmetic;
use crate::bits;
use super::register::Flags;

pub fn add_flags( a: u8, b: u8 ) -> ( u8, Flags) {
    let (result, carry) = arithmetic::add(a, b);

    let flags = Flags {
        zero: result == 0,
        carry: carry != 0,
        borrow: false,
        negative: bits::get_bit_bit(result, 7) == Some(1), 
        overflow: 
            super::overflow::addition_overflow( a, b, result ),
    };


    (result, flags)
}