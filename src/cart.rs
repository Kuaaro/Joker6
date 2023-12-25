use std::fmt::LowerExp;

use rand::{thread_rng, Rng};

pub trait CartHandler {
    fn put(&mut self, input: u8) -> u8;
    fn close(&self) -> Vec<u8>;
}

struct Cart4BitsForward {
    entrance: u8,
    seats: [u8; 16],
    exit: u8
}

impl Cart4BitsForward {
    fn new(input: u8) -> Self {
        let mut rand_gen = thread_rng();
        let mut seats = [0u8; 16];
        
        for i in 0..16 {
            seats[i] = rand_gen.gen::<u8>() % 16;
        }

        let mut out = Cart4BitsForward{entrance: rand_gen.gen::<u8>() % 16, seats, exit: 0};

        out.put(input);

        return out;
    }

    fn put(&mut self, input: u8) -> u8 {
        let out = self.exit;

        self.exit = self.entrance;
        self.entrance = self.seats[self.exit as usize];
        self.seats[self.exit as usize] = input;
        
        return out;
    }

    fn close(&self) -> [u8; 19] {
        let mut out = [0u8; 19];

        out[0] = self.exit;
        out[1] = self.entrance;
        out[2] = self.seats[self.entrance as usize];

        for i in 0..15 {
            out[i+3] = self.seats[i];
        }

        out[self.entrance as usize + 3] = thread_rng().gen::<u8>() % 16;

        return out;
    }
}

pub struct Cart4BitsForwardHandler {
    upper_cart: Cart4BitsForward,
    lower_cart: Cart4BitsForward
}

impl CartHandler for Cart4BitsForwardHandler {
    fn put(&mut self, input: u8) -> u8 {
        return (self.upper_cart.put(input >> 4) << 4) | self.lower_cart.put(input & 15);
    }

    fn close(&self) -> Vec<u8> {
        return self.upper_cart.close().iter().zip(self.lower_cart.close().iter()).map(|(&upper, &lower)| (upper << 4) | lower).collect();
    }
}

impl Cart4BitsForwardHandler {
    pub fn new(input: u8) -> Self {
        return Cart4BitsForwardHandler{upper_cart: Cart4BitsForward::new(input >> 4), lower_cart: Cart4BitsForward::new(input & 15)};
    }
}