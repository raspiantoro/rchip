pub struct Memory {
    cell: [u8; 4096],
}

impl Memory {
    pub fn new() -> Self {
        Memory { cell: [0; 4096] }
    }

    pub fn assign(&mut self, address: usize, value: u8) {
        self.cell[address] = value;
    }

    pub fn get(&self, pos: usize) -> u8 {
        self.cell[pos]
    }
}
