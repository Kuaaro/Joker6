#[derive(Copy, Clone)]
pub struct Swapper {
    swap_table: [u8; 256],
    reswap_table: [u8; 256],
    shift: u8
}

impl Swapper {
    pub fn new(key: [u8; 256]) -> Self {
        let swap_table = mk_swap_table(&key);

        return Swapper {shift: key[0], swap_table, reswap_table: mk_reswap_table(&swap_table)};
    }

    pub fn swap(&self, input: u8, index: u8) -> u8 {
        return self.swap_table[(input + index * self.shift) as usize];
    }

    pub fn reswap(&self, input: u8, index: u8) -> u8 {
        return self.reswap_table[input as usize] - index * self.shift;
    }
}

fn mk_swap_table(key: &[u8; 256]) -> [u8; 256] {
    let mut out = [0u8; 256];

    for i in 1..256 {
        out[i] = i as u8;
        out.swap(i, key[i] as usize);
    }

    return out;
}

fn mk_reswap_table(swap_table: &[u8; 256]) -> [u8; 256] {
    let mut out = [0u8; 256];

    for i in 0..256 {
        out[swap_table[i] as usize] = i as u8;
    }

    return out;
}

