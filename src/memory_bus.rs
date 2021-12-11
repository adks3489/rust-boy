use crate::{gpu, gpu::GPU};
use static_assertions::const_assert;

const BOOT_ROM_FIRST: usize = 0x0;
const BOOT_ROM_LAST: usize = 0xFF;
const BOOT_ROM_SIZE: usize = BOOT_ROM_LAST - BOOT_ROM_FIRST + 1;

const CARTRIDGE_ROM_FIRST: usize = 0x100;
const CARTRIDGE_ROM_LAST: usize = 0x7FFF;
const CARTRIDGE_ROM_SIZE: usize = CARTRIDGE_ROM_LAST - CARTRIDGE_ROM_FIRST + 1;

const GRAPHICS_RAM_FIRST: usize = 0x8000;
const GRAPHICS_RAM_LAST: usize = 0x9FFF;
const_assert!(gpu::RAM_SIZE == GRAPHICS_RAM_LAST - GRAPHICS_RAM_FIRST + 1);

const CARTRIDGE_RAM_FIRST: usize = 0xA000;
const CARTRIDGE_RAM_LAST: usize = 0xBFFF;
const CARTRIDGE_RAM_SIZE: usize = CARTRIDGE_RAM_LAST - CARTRIDGE_RAM_FIRST + 1;

const WORKING_RAM_FIRST: usize = 0xC000;
const WORKING_RAM_LAST: usize = 0xFDFF;
const WORKING_RAM_SIZE: usize = WORKING_RAM_LAST - WORKING_RAM_FIRST + 1;

const GRAPHICS_SPRITE_FIRST: usize = 0xFE00;
const GRAPHICS_SPRITE_LAST: usize = 0xFE9F;
const_assert!(gpu::SPRITE_SIZE == GRAPHICS_SPRITE_LAST - GRAPHICS_SPRITE_FIRST + 1);

const IO_FIRST: usize = 0xFF00;
const IO_LAST: usize = 0xFF7F;
const IO_SIZE: usize = IO_LAST - IO_FIRST + 1;

const ZERO_PAGE_FIRST: usize = 0xFF80;
const ZERO_PAGE_LAST: usize = 0xFFFF;
const ZERO_PAGE_SIZE: usize = ZERO_PAGE_LAST - ZERO_PAGE_FIRST + 1;

pub struct MemoryBus<'a> {
    boot_rom: [u8; BOOT_ROM_SIZE],
    cartridge_rom: [u8; CARTRIDGE_ROM_SIZE],
    memory: [u8; WORKING_RAM_SIZE],
    cartridge_ram: [u8; CARTRIDGE_RAM_SIZE],
    gpu_ram: &'a mut [u8; gpu::RAM_SIZE],
    gpu_sprite: &'a mut [u8; gpu::SPRITE_SIZE],
}

impl<'a> MemoryBus<'a> {
    pub fn new(
        boot_rom: Vec<u8>,
        vram: &'a mut [u8; gpu::RAM_SIZE],
        gpu_sprite: &'a mut [u8; gpu::SPRITE_SIZE],
    ) -> Self {
        MemoryBus {
            boot_rom: boot_rom.try_into().unwrap(),
            cartridge_rom: [0; CARTRIDGE_ROM_SIZE],
            memory: [0; WORKING_RAM_SIZE],
            cartridge_ram: [0; CARTRIDGE_RAM_SIZE],
            gpu_ram: vram,
            gpu_sprite,
        }
    }
    pub fn read_byte(&self, address: u16) -> u8 {
        let address = address as usize;
        match address {
            BOOT_ROM_FIRST..=BOOT_ROM_LAST => self.boot_rom[address],
            CARTRIDGE_ROM_FIRST..=CARTRIDGE_ROM_LAST => {
                self.cartridge_rom[address - CARTRIDGE_ROM_FIRST]
            }
            GRAPHICS_RAM_FIRST..=GRAPHICS_RAM_LAST => self.gpu_ram[address - GRAPHICS_RAM_FIRST],
            CARTRIDGE_RAM_FIRST..=CARTRIDGE_RAM_LAST => panic!(),
            WORKING_RAM_FIRST..=WORKING_RAM_LAST => self.memory[address - WORKING_RAM_FIRST],
            GRAPHICS_SPRITE_FIRST..=GRAPHICS_SPRITE_LAST => {
                self.gpu_sprite[address - GRAPHICS_SPRITE_FIRST]
            }
            IO_FIRST..=IO_LAST => panic!(),
            ZERO_PAGE_FIRST..=ZERO_PAGE_LAST => panic!(),
            _ => panic!("invalid read address: {}", address),
        }
    }
    pub fn write_byte(&mut self, address: u16, byte: u8) {
        let address = address as usize;
        match address {
            GRAPHICS_RAM_FIRST..=GRAPHICS_RAM_LAST => {
                self.gpu_ram[address - GRAPHICS_RAM_FIRST] = byte
            }
            CARTRIDGE_RAM_FIRST..=CARTRIDGE_RAM_LAST => panic!(),
            WORKING_RAM_FIRST..=WORKING_RAM_LAST => self.memory[address - WORKING_RAM_FIRST] = byte,
            GRAPHICS_SPRITE_FIRST..=GRAPHICS_SPRITE_LAST => {
                self.gpu_sprite[address - GRAPHICS_SPRITE_FIRST] = byte
            }
            IO_FIRST..=IO_LAST => panic!(),
            ZERO_PAGE_FIRST..=ZERO_PAGE_LAST => panic!(),
            _ => panic!("invalid write address: {}", address),
        }
    }
    pub fn write_word(&mut self, address: u16, word: u16) {
        let address = address as usize;
        let lsb = (word & 0xFF) as u8;
        let msb = ((word & 0xFF00) >> 8) as u8;
        match address {
            GRAPHICS_RAM_FIRST..=GRAPHICS_RAM_LAST => {
                self.gpu_ram[address - GRAPHICS_RAM_FIRST] = lsb;
                self.gpu_ram[address - GRAPHICS_RAM_FIRST + 1] = msb;
            }
            CARTRIDGE_RAM_FIRST..=CARTRIDGE_RAM_LAST => panic!(),
            WORKING_RAM_FIRST..=WORKING_RAM_LAST => {
                self.memory[address - WORKING_RAM_FIRST] = lsb;
                self.memory[address - WORKING_RAM_FIRST + 1] = msb;
            }
            GRAPHICS_SPRITE_FIRST..=GRAPHICS_SPRITE_LAST => {
                self.gpu_sprite[address - GRAPHICS_SPRITE_FIRST] = lsb;
                self.gpu_sprite[address - GRAPHICS_SPRITE_FIRST + 1] = msb;
            }
            IO_FIRST..=IO_LAST => panic!(),
            ZERO_PAGE_FIRST..=ZERO_PAGE_LAST => panic!(),
            _ => panic!("invalid write address: {}", address),
        };
    }
}
