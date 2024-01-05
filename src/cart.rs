use rand::{Rng, thread_rng};

pub trait ForwardCart {
    fn new() -> Self;
    fn put(&mut self, input: u8) -> u8;
    fn close(&self, out: &mut [u8]);
}

pub trait BackwardCart {
    fn new(input: &[u8]) -> Self;
    fn put(&mut self, input: u8) -> u8;
}

pub struct Cart4BitsForward {
    entrance: u8,
    seats: [u8; 16],
    exit: u8
}

impl ForwardCart for Cart4BitsForward {
    fn new() -> Self {
        let mut rng = thread_rng();

        return Cart4BitsForward {
            entrance: rng.gen(),
            seats: rand::thread_rng().gen(),
            exit: rng.gen()
        };
    }

    fn put(&mut self, input: u8) -> u8 {
        let out: u8 = self.exit;
        let mut index: usize;

        self.exit = self.entrance;
        self.entrance = 0;

        index = (self.exit & 15) as usize;
        self.entrance |= self.seats[index] & 15;
        self.seats[index] = (self.seats[index] & 240) | (input & 15);

        index = (self.exit >> 4) as usize;
        self.entrance |= self.seats[index] & 240;
        self.seats[index] = (self.seats[index] & 15) | (input & 240);

        return out;
    }

    fn close(&self, out: &mut [u8]) {
        out[0] = self.entrance;
        out[1] = self.exit;

        out[2..].clone_from_slice(&self.seats);
    }
}