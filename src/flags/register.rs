
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Flags {
    pub zero: bool,
    pub carry: bool,
    pub borrow: bool,
    pub negative: bool,
    pub overflow: bool,
}


impl Flags {
    
    pub fn empty() -> Self {
        Self {
            zero: false,
            carry: false,
            borrow: false,
            negative: false,
            overflow: false,
        }
    }
}