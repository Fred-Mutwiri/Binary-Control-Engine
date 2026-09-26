
use binary_control_engine::{bits, arithmetic, cpu};


fn main() {

    let value = 0b10110110;
    println!("\n\nThe Binary Control Engine:\n");

    println!("\n\nThe Binary Control Engine: [**********************BIT MANIPULATION*****************]\n");
    println!("Original: {:08b}\n", value);

    println!("Bit 0: {:?}", bits::get_bit_bit(value, 0));
    println!("Bit 1: {:?}", bits::get_bit_bit(value, 1));
    println!("Bit 2: {:?}\n", bits::get_bit_bit(value, 2));

    println!("   set bit 0: {:08b}", bits::set_bit(value, 0).unwrap());
    println!(" clear bit 2: {:08b}", bits::clear_bit(value, 2).unwrap());
    println!("toggle bit 1: {:08b}\n", bits::set_bit(value, 1).unwrap());

    println!("Original:         {:08b}\n", value);

    println!( "Bit 2:           {:?}", bits::get_bit_bit(value, 2).unwrap() );

    println!( "Bit mask 2:      {:08b}", bits::single_bit_mask(2).unwrap() );

    println!( "Field mask:      {:08b}", bits::field_mask(2, 3).unwrap() );

    println!( "Extract field:   {:08b}", bits::extract_field(value, 2, 3).unwrap());

    println!( "Population:      {}", bits::population_count(value));

    println!( "Population fast: {}", bits::population_count_fast(value) );

    println!( "Lowest bit:      {:08b}", bits::lowest_set_bit(value).unwrap() );

    println!( "Lowest fast:     {:08b}", bits::lowest_set_bit_fast(value).unwrap() );

    println!( "Reversed:        {:08b}", bits::reverse_bits(value) );





    println!("\n\nThe Binary Control Engine: [********************** ARITHMETIC ENGINE *****************]\n");
    let add = arithmetic::add_operation(200, 100);

    println!("ADD");
    println!("value  = {:08b}", add.value);
    println!("carry  = {}", add.carry);

    let sub = arithmetic::subtract_operation(3, 10);

    println!();
    println!("SUBTRACT");
    println!("value  = {:08b}", sub.value);
    println!("borrow = {}", sub.borrow);

    let mul = arithmetic::multiply_operation(20, 20);

    println!();
    println!("MULTIPLY");
    println!("value  = {:08b}", mul.value);
    println!("carry  = {}", mul.carry);

    let div = arithmetic::divide_operation(23, 5)
        .unwrap();

    println!();
    println!("DIVIDE");
    println!("quotient  = {}", div.value);
    println!("remainder = {}", div.carry);



    let program = vec![
        0b0001_0101, // LOAD 5
        0b0010_0011, // ADD 3
        0b0011_0010, // SUB 2
        0b1111_0000, // HALT
    ];

    

    let machine = cpu::run(program);
    println!("\n\nThe Binary Control Engine: [********************** CPU ENGINE *****************]\n");

    println!("Accumulator: {}", machine.accumulator);
    println!("Accumulator binary: {:08b}",  machine.accumulator);
    println!("Program Counter: {}", machine.pc.value);
    println!("Zero flag: {}", machine.flags.zero );
    println!("Carry flag: {}", machine.flags.carry);
    println!("Borrow flag: {}", machine.flags.borrow);
    println!("Halted: {}", machine.halted);


}