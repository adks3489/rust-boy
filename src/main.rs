mod cpu;
mod gpu;
mod memory_bus;
use crate::{cpu::CPU, gpu::GPU, memory_bus::MemoryBus};
use minifb::{Key, Window, WindowOptions};
use std::{fs::OpenOptions, io::Read};

fn read_buffer(path: &str) -> std::io::Result<Vec<u8>> {
    let mut file = OpenOptions::new().read(true).open(path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    Ok(buf)
}

fn main() {
    let boot_rom = read_buffer("dmg_boot.bin").expect("Open boot_rom failed");
    let mut gpu = GPU::new();
    let mut memory_bus = MemoryBus::new(boot_rom, &mut gpu.ram, &mut gpu.sprite);
    let mut cpu = CPU::new(&mut memory_bus);

    const WIDTH: usize = 160;
    const HEIGHT: usize = 144;
    let mut window = Window::new("RustBoy", WIDTH, HEIGHT, WindowOptions::default()).unwrap();
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    while window.is_open() && !window.is_key_down(Key::Escape) {
        cpu.step();
        window
            .update_with_buffer(&gpu.canvas, WIDTH, HEIGHT)
            .unwrap();
    }
}
