mod cpu;
use cpu::CPU;
use std::{fs::OpenOptions, io::Read};

fn read_buffer(path: &str) -> std::io::Result<Vec<u8>> {
    let mut file = OpenOptions::new().read(true).open(path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    Ok(buf)
}

fn main() {
    let boot_rom = read_buffer("dmg_boot.bin").expect("Open boot_rom failed");

    let mut cpu = CPU::new(boot_rom);
    loop {
        cpu.step();
    }
}
