use crate::io::{Reader, Writer};
use crate::cart::Cart4BitsForwardHandler;
use crate::swapper::{Swapper, self};
use sha3::{Shake256, digest::{Update, ExtendableOutput, XofReader}};

pub trait Joker {
    fn encrypt(&self, reader: Box<dyn Reader>) -> Box<dyn Writer>;
    fn decrypt(&self, reader: Box<dyn Reader>) -> Box<dyn Writer>;
}

pub struct Joker6 {
    keys: [Swapper; 16]
}

impl Joker6 {
    pub fn new(pwd: &String) -> Self {
        let mut out = Joker6{keys: [Swapper::new([0u8; 256]); 16]};

        out.update_key(pwd);

        return out;
    }
    
    pub fn update_key(&mut self, pwd: &String) {
        let mut hash = mk_hash(pwd.as_bytes());

        for i in 0..14 {
            self.keys[i] = Swapper::new(mk_key(&hash));
            hash = mk_hash(&hash);
        }
        self.keys[15] = Swapper::new(mk_key(&hash));
    }
}

impl Joker for Joker6 {
    fn encrypt(&self, reader: Box<dyn Reader>) -> Box<dyn Writer> {
        let carts: [Cart4BitsForwardHandler; 16] = [
            Cart4BitsForwardHandler::new(input)
        ];
        
    }

    fn decrypt(&self, reader: Box<dyn Reader>) -> Box<dyn Writer> {
        
    }
}


fn mk_key(hash: &[u8; 256]) -> [u8; 256] {
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