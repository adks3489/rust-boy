use crate::gpu::GPU;

pub struct MemoryBus<'a> {
    memory: [u8; 0xFFFF],
    gpu: &'a GPU,
}
impl<'a> MemoryBus<'a> {
    pub fn new(boot_rom: Vec<u8>, gpu: &'a GPU) -> Self {
        let mut bus = MemoryBus {
            memory: [0; 0xFFFF],
            gpu: gpu,
        };
        let boot_rom: [u8; 256] = boot_rom.try_into().unwrap();
        bus.memory[0..256].copy_from_slice(&boot_rom);
        bus
    }
    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
    pub fn write_byte(&mut self, address: u16, byte: u8) {
        self.memory[address as usize] = byte;
    }
    pub fn write_word(&mut self, address: u16, word: u16) {
        self.memory[address as usize] = (word & 0xFF) as u8;
        self.memory[address as usize + 1] = ((word & 0xFF00) >> 8) as u8;
    }
}
