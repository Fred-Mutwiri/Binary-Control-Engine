use super::{ add, subtract, increment, decrement, multiply, divide };


pub struct ArithmeticResult {
    pub value: u8,
    pub carry: u8,
    pub borrow: bool,
}

pub fn add_operation( a: u8, b: u8 ) -> ArithmeticResult {
    let (value, carry) = add(a, b);

    ArithmeticResult{
        value,
        carry,
        borrow: false,
    }
}

pub fn subtract_operation( a: u8, b: u8 ) -> ArithmeticResult {
    let (value, borrow) = subtract( a, b );

    ArithmeticResult{
        value,
        carry: 0,
        borrow,
    }
}

pub fn multiply_operation( a: u8, b: u8 ) -> ArithmeticResult {
    let (value, carry) = multiply(a, b);

    ArithmeticResult{
        value,
        carry,
        borrow: false,
    }
}

pub fn divide_operation( a: u8, b: u8) -> Option<ArithmeticResult> {
    let (value, remainder) = divide(a, b)?;

    Some( 
        ArithmeticResult {
            value,
            carry: remainder,
            borrow: false,
        }
    )
}

pub fn increment_operation( value: u8) -> ArithmeticResult {
    let (value, carry) = increment(value);

    ArithmeticResult {
        value,
        carry,
        borrow: false,
    }
}

pub fn decrement_operation( value: u8) -> ArithmeticResult {
    let (value, borrow) = decrement(value);

    ArithmeticResult {
        value,
        carry: 0,
        borrow,
    }
}