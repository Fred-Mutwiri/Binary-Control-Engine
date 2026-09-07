
use binary_control_engine::bits;


fn main() {

    let value = 0b10110110;
    println!("\n\nThe Binary Control Engine:\n");

    println!("Original: {:08b}\n", value);

    println!("Bit 0: {:?}", bits::get_bit_bit(value, 0));
    println!("Bit 1: {:?}", bits::get_bit_bit(value, 1));
    println!("Bit 2: {:?}\n", bits::get_bit_bit(value, 2));

    println!("   set bit 0: {:08b}", bits::set_bit(value, 0).unwrap());
    println!(" clear bit 2: {:08b}", bits::clear_bit(value, 2).unwrap());
    println!("toggle bit 1: {:08b}\n", bits::set_bit(value, 1).unwrap());
}