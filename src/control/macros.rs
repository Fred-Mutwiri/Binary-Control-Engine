#[macro_export]
macro_rules! repeat_eight{
    ($body:block) => {
        for _ in 0..8 {
           $body 
        }
    };
}


#[macro_export]
macro_rules! opcode {
    ($name: ident, $value:expr) => {
        pub const $name: u8 = $value;
    };
}
