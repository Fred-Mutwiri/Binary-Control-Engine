pub mod half_adder;
pub mod full_adder;

pub mod add;
pub mod subtract;
pub mod increment;
pub mod decrement;
pub mod compare;
pub mod multiply;
pub mod divide;

pub mod engine;

pub use half_adder::half_adder;
pub use full_adder::full_adder;

pub use add::add;
pub use subtract::subtract;

pub use increment::increment;
pub use decrement::decrement;

pub use compare::{
    compare,
    Ordering,
};

pub use multiply::multiply;
pub use divide::divide;


pub use engine::{
    ArithmeticResult,
    add_operation,
    subtract_operation,
    multiply_operation,
    divide_operation,
    increment_operation,
    decrement_operation,
};