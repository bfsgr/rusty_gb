pub trait BitUtils {
    fn set_bit(&mut self, bit: u8);
    fn reset_bit(&mut self, bit: u8);
    fn test_bit(&self, bit: u8) -> bool;
}

macro_rules! impl_BitUtils {
    (for $($t:ty),+) => {
        $(impl BitUtils for $t {
            fn set_bit(&mut self, bit: u8){
                let x = 1 << bit; //>
                *self = *self | x;
            }
            fn reset_bit(&mut self, bit: u8){
                let x = 1 << bit; //>
                *self = *self & !x;
            }
            fn test_bit(&self, bit: u8) -> bool {
                let x = 1 << bit; //>
                
                if (*self & x) == x {
                    true
                } else {
                    false
                }
            }
        })*
    }
}

impl_BitUtils!(for u8, u16);