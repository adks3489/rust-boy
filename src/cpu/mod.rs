pub mod instruction;
pub mod registers;
use instruction::*;
use registers::Registers;

struct CPU {
    registers: Registers,
    pc: u16,
    bus: MemoryBus,
}
struct MemoryBus {
    memory: [u8; 0xFFFF],
}
impl MemoryBus {
    fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
}
impl CPU {
    fn step(&mut self) {
        let mut instruction_byte = self.bus.read_byte(self.pc);
        let prefixed = instruction_byte == 0xCB;
        if prefixed {
            instruction_byte = self.bus.read_byte(self.pc + 1);
        }
        let instruction = Instruction::from_byte(instruction_byte, prefixed)
            .unwrap_or_else(|| panic!("Unknown instruction found for: 0x{:x}", instruction_byte));
        self.pc = self.execute(instruction);
    }
    fn execute(&mut self, instruction: Instruction) -> u16 {
        match instruction {
            Instruction::ADD(target) => match target {
                ArithmeticTarget::A => todo!(),
                ArithmeticTarget::B => todo!(),
                ArithmeticTarget::C => {
                    let new_value = self.add(self.registers.c);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::D => todo!(),
                ArithmeticTarget::E => todo!(),
                ArithmeticTarget::H => todo!(),
                ArithmeticTarget::L => todo!(),
            },
            Instruction::INC(_) => todo!(),
        }
    }
    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        new_value
    }
}
