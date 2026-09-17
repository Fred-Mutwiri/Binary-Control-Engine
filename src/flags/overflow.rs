use crate::bits::get_bit_bit;

pub fn addition_overflow( a: u8, b: u8, result: u8 ) -> bool {
    let a_sign = get_bit_bit(a, 7).unwrap();
    let b_sign = get_bit_bit(b, 7).unwrap();
    let result_sign = get_bit_bit(result, 7).unwrap();

    a_sign == b_sign && a_sign != result_sign
}