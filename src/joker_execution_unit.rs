use crate::swapper::Swapper;
use crate::cart::{Cart4BitsForward, ForwardCart};

pub trait JokerExecutionUnitForward {
    fn new(key: &[u8; 256]) -> Self;
    fn put(&mut self, input: u8, index: u8) -> u8;
    fn close(&self, out: &mut [u8], index: u8);
}

pub trait JokerExecutionUnitBackward {
    fn new(key: &[u8; 256], cart_state: &[u8]) -> Self;
    fn put(&mut self, input: u8, index: u8) -> u8;
}

pub struct StandardEncryptionJokerExecutionUnit {
    swapper: Swapper,
    cart: Cart4BitsForward
}

impl JokerExecutionUnitForward for StandardEncryptionJokerExecutionUnit {
    fn new(key: &[u8; 256]) -> Self {
        return StandardEncryptionJokerExecutionUnit {swapper: Swapper::new(key), cart: Cart4BitsForward::new()};
    }

    fn put(&mut self, input: u8, index: u8) -> u8 {
        return self.swapper.reswap(self.cart.put(self.swapper.swap(input, index)), index);
    }

    fn close(&self, out: &mut [u8], index: u8) {
        self.cart.close(out);

        for i in 0..18 {
            out[i] = self.swapper.reswap(out[i], index + i as u8)
        }
    }
}