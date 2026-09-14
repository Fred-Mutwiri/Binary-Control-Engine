use super::subtract::subtract;

pub fn decrement( value: u8) -> ( u8, bool ) {
    let (value1, value2) = subtract( value, 1);


    (value1, value2)
}