pub mod zero;
pub mod carry;
pub mod borrow;
pub mod negative;
pub mod overflow;
pub mod register;
pub mod from_arithmetic;
pub mod packed;

pub use zero::is_zero;
pub use carry::is_carry;
pub use borrow::is_borrow;
pub use negative::is_negative;

pub use overflow::addition_overflow;

pub use register::Flags;

pub use from_arithmetic::add_flags;

pub use packed::{
    ZERO,
    CARRY,
    BORROW,
    NEGATIVE,
    OVERFLOW,
};