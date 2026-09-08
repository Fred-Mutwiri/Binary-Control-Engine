

pub fn single_bit_mask(position: u8) -> Option<u8> {
    if position >= 8 {
        return None;
    }

    Some(1u8 << position)
}

pub fn field_mask( start: u8, width: u8 ) -> Option<u8> {

    if width > 8 {
        return None;
    }

    if start > 8 {
        return None;
    } 

    if start + width > 8 {
        return None;
    }

    let ones = ( 1 << width ) - 1 ;
    Some ( ones << start )
}







#[cfg(test)]
mod tests{

    use super::*;


    #[test]
    fn position_invalid() {
        assert_eq!( single_bit_mask(8), None)
    }

    #[test] 
    fn mask_at_two() {
        assert_eq!( single_bit_mask(2), Some(0b0000_0100))
    }

    #[test]
    fn mask_at_highest_bit() {
        assert_eq!( single_bit_mask(7), Some(0b1000_0000))
    }

    #[test]
    fn mask_at_lowest_bit() {
        assert_eq!( single_bit_mask(0), Some(0b0000_0001))
    }

    #[test]
    fn invalid_field_mask() {
        assert_eq!( field_mask(1, 8), None)
    }

    #[test]
    fn invalid_field_mask_two() {
        assert_eq!( field_mask(8, 1), None)
    } 

    #[test]
    fn invalid_field_mask_three() {
        assert_eq!( field_mask(6, 4), None)
    }

    #[test]
    fn  extract_field_mask() {
        assert_eq!( field_mask(2, 3), Some(0b0001_1100 ) )
    }

}
