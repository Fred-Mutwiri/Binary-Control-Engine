pub fn lowest_set_bit(value: u8) -> Option<u8> {
    if value == 0 {
        return None;
    }

    for position in 0..8 {
        let mask = 1u8 << position;

        if value & mask != 0 {
            return Some(mask);
        }
    }

    None
}


pub fn lowest_set_bit_fast(value: u8) -> Option<u8> {
    if value == 0 {
        return None;
    }

    let negated_value = (!value).wrapping_add(1);

    Some(value & negated_value)
}


