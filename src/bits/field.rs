

use super::mask::field_mask;

pub fn extract_field( value: u8, start: u8, width: u8) -> Option<u8> {
    let mask = field_mask(start, width)?;

    Some((value & mask) >> start )
}