pub mod registers;
use registers::Registers;

enum Instruction {
    ADD(ArithmeticTarget),
}
enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

struct CPU {
    registers: Registers,
}
impl CPU {
    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(target) => match target {
                ArithmeticTarget::A => todo!(),
                ArithmeticTarget::B => todo!(),
                ArithmeticTarget::C => {
                    let new_value = self.add(self.registers.c);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::D => todo!(),
                ArithmeticTarget::E => todo!(),
                ArithmeticTarget::H => todo!(),
                ArithmeticTarget::L => todo!(),
            },
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
