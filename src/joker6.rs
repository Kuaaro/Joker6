use crate::io::{Reader, Writer};
use crate::cart::{Cart4BitsForwardHandler, CartHandler, self};
use crate::swapper::Swapper;
use sha3::{Shake256, digest::{Update, ExtendableOutput, XofReader}};

pub trait Joker {
    fn encrypt(&self, reader: Box<dyn Reader>, writer: &mut Box<dyn Writer>);
    fn decrypt(&self, reader: Box<dyn Reader>, writer: &mut Box<dyn Writer>);
}

pub struct Joker6 {
    swappers: [Swapper; 16]
}

impl Joker6 {
    pub fn new(pwd: &String) -> Self {
        let mut out = Joker6{swappers: [Swapper::new([0u8; 256]); 16]};

        out.update_key(pwd);

        return out;
    }
    
    pub fn update_key(&mut self, pwd: &String) {
        let mut hash: &[u8] = &pwd.as_bytes();

        for i in 0..16 {
            hash = &mk_hash(hash);
            self.swappers[i] = Swapper::new(mk_key(hash));
        }
    }
}

impl Joker for Joker6 {
    fn encrypt(&self, reader: Box<dyn Reader>, writer: &mut Box<dyn Writer>) {
        let mut carts: [Cart4BitsForwardHandler; 16] = {
            let mut v: Vec<Cart4BitsForwardHandler> = Vec::with_capacity(16);

            for _i in 0..16 {
                v.push(Cart4BitsForwardHandler::new());
            }
            
            v.try_into().expect("")
        };
        
        let mut byte: u8;
        let mut index: u8 = 0;

        while reader.is_readable() {
            byte = reader.read();

            for i in 0..16 {
                byte = self.swappers[i].swap(byte, index);
                byte = carts[i].put(byte);
                byte = self.swappers[i].reswap(byte, index);
            }

            index += 1;
            writer.write(byte);
        }

        let mut close_bytes: Vec<u8>;

        for cart_no in 0..16 { //closing carts
            close_bytes = carts[cart_no].close();
            for byte_no in 0..19 { //byte index
                close_bytes[byte_no] = self.swappers[cart_no].reswap(close_bytes[byte_no], index);
                for swapper_no in cart_no+1..16 { //looping through remaining carts
                    
                    close_bytes[byte_no] = self.swappers[swapper_no].swap(close_bytes[byte_no], index);
                    close_bytes[byte_no] = carts[swapper_no].put(close_bytes[byte_no]);
                    close_bytes[byte_no] = self.swappers[swapper_no].reswap(close_bytes[byte_no], index);

                }
                writer.write(close_bytes[byte_no]);
                index += 1;
            }
        }
    }

    fn decrypt(&self, reader: Box<dyn Reader>, writer: &mut Box<dyn Writer>) {
        
    }
}


fn mk_key(hash: &[u8]) -> [u8; 256] {
    let mut out = [0u8; 256];
    out[0] = hash[0];

    for i in 2..=255 {
        out[i-1] %= i as u8;
    }

    out[255] = hash[255];

    return out;
}

fn mk_hash(input: &[u8]) -> [u8; 256] {
    let mut hasher = Shake256::default();
    hasher.update(input);
    let mut out = [0u8; 256];
    hasher.finalize_xof().read(&mut out);
    return out;
}