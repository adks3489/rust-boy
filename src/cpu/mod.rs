pub mod instruction;
pub mod registers;
use instruction::*;
use registers::Registers;

struct CPU {
    registers: Registers,
    pc: u16,
    sp: u16,
    bus: MemoryBus,
    is_halted: bool,
}
struct MemoryBus {
    memory: [u8; 0xFFFF],
}
impl MemoryBus {
    fn new() -> Self {
        MemoryBus {
            memory: [0; 0xFFFF],
        }
    }
    fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
    fn write_byte(&mut self, address: u16, byte: u8) {
        self.memory[address as usize] = byte;
    }
    fn write_word(&mut self, address: u16, word: u16) {
        self.memory[address as usize] = (word & 0xFF) as u8;
        self.memory[address as usize + 1] = ((word & 0xFF00) >> 8) as u8;
    }
}
impl CPU {
    fn new() -> Self {
        CPU {
            registers: Registers::new(),
            pc: 0,
            sp: 0,
            bus: MemoryBus::new(),
            is_halted: false,
        }
    }
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
    fn read_next_byte(&self) -> u8 {
        self.bus.read_byte(self.pc + 1)
    }
    fn read_next_word(&self) -> u16 {
        let lsb = self.bus.read_byte(self.pc + 1);
        let msb = self.bus.read_byte(self.pc + 2);
        (msb << 8) as u16 | lsb as u16
    }
    fn execute(&mut self, instruction: Instruction) -> u16 {
        if self.is_halted {
            return self.pc;
        }
        match instruction {
            Instruction::ADD(target) => {
                let value = match target {
                    ArithmeticTarget::A => self.add(self.registers.a, false),
                    ArithmeticTarget::B => self.add(self.registers.b, false),
                    ArithmeticTarget::C => self.add(self.registers.c, false),
                    ArithmeticTarget::D => self.add(self.registers.d, false),
                    ArithmeticTarget::E => self.add(self.registers.e, false),
                    ArithmeticTarget::H => self.add(self.registers.h, false),
                    ArithmeticTarget::L => self.add(self.registers.l, false),
                    ArithmeticTarget::HLI => {
                        self.add(self.bus.read_byte(self.registers.get_hl()), false)
                    }
                    ArithmeticTarget::D8 => {
                        let v = self.add(self.read_next_byte(), false);
                        let _ = self.pc.wrapping_add(1);
                        v
                    }
                };
                self.registers.a = value;
                self.pc.wrapping_add(1)
            }
            Instruction::ADC(source) => {
                let value = match source {
                    Source::A => self.add(self.registers.a, true),
                    Source::B => self.add(self.registers.b, true),
                    Source::C => self.add(self.registers.c, true),
                    Source::D => self.add(self.registers.d, true),
                    Source::E => self.add(self.registers.e, true),
                    Source::H => self.add(self.registers.h, true),
                    Source::L => self.add(self.registers.l, true),
                    Source::IndirectHL => {
                        self.add(self.bus.read_byte(self.registers.get_hl()), true)
                    }
                    Source::D8 => {
                        let v = self.add(self.read_next_byte(), true);
                        let _ = self.pc.wrapping_add(1);
                        v
                    }
                    _ => panic!("unsupported ADC source"),
                };
                self.registers.a = value;
                self.pc.wrapping_add(1)
            }
            Instruction::SUB(source) => {
                let value = match source {
                    Source::A => self.sub(self.registers.a, false),
                    Source::B => self.sub(self.registers.b, false),
                    Source::C => self.sub(self.registers.c, false),
                    Source::D => self.sub(self.registers.d, false),
                    Source::E => self.sub(self.registers.e, false),
                    Source::H => self.sub(self.registers.h, false),
                    Source::L => self.sub(self.registers.l, false),
                    Source::IndirectHL => {
                        self.sub(self.bus.read_byte(self.registers.get_hl()), false)
                    }
                    Source::D8 => {
                        let v = self.sub(self.read_next_byte(), false);
                        let _ = self.pc.wrapping_add(1);
                        v
                    }
                    _ => panic!("unsupported SUB source"),
                };
                self.registers.a = value;
                self.pc.wrapping_add(1)
            }
            Instruction::SBC(source) => {
                let value = match source {
                    Source::A => self.sub(self.registers.a, true),
                    Source::B => self.sub(self.registers.b, true),
                    Source::C => self.sub(self.registers.c, true),
                    Source::D => self.sub(self.registers.d, true),
                    Source::E => self.sub(self.registers.e, true),
                    Source::H => self.sub(self.registers.h, true),
                    Source::L => self.sub(self.registers.l, true),
                    Source::IndirectHL => {
                        self.sub(self.bus.read_byte(self.registers.get_hl()), true)
                    }
                    Source::D8 => {
                        let v = self.sub(self.read_next_byte(), true);
                        let _ = self.pc.wrapping_add(1);
                        v
                    }
                    _ => panic!("unsupported SBC source"),
                };
                self.registers.a = value;
                self.pc.wrapping_add(1)
            }
            Instruction::AND(source) => {
                let value = match source {
                    Source::A => self.and(self.registers.a),
                    Source::B => self.and(self.registers.b),
                    Source::C => self.and(self.registers.c),
                    Source::D => self.and(self.registers.d),
                    Source::E => self.and(self.registers.e),
                    Source::H => self.and(self.registers.h),
                    Source::L => self.and(self.registers.l),
                    Source::IndirectHL => self.and(self.bus.read_byte(self.registers.get_hl())),
                    Source::D8 => {
                        let v = self.and(self.read_next_byte());
                        let _ = self.pc.wrapping_add(1);
                        v
                    }
                    _ => panic!("unsupported AND source"),
                };
                self.registers.a = value;
                self.pc.wrapping_add(1)
            }
            Instruction::XOR(source) => {
                let value = match source {
                    Source::A => self.xor(self.registers.a),
                    Source::B => self.xor(self.registers.b),
                    Source::C => self.xor(self.registers.c),
                    Source::D => self.xor(self.registers.d),
                    Source::E => self.xor(self.registers.e),
                    Source::H => self.xor(self.registers.h),
                    Source::L => self.xor(self.registers.l),
                    Source::IndirectHL => self.xor(self.bus.read_byte(self.registers.get_hl())),
                    Source::D8 => {
                        let v = self.xor(self.read_next_byte());
                        let _ = self.pc.wrapping_add(1);
                        v
                    }
                    _ => panic!("unsupported XOR source"),
                };
                self.registers.a = value;
                self.pc.wrapping_add(1)
            }
            Instruction::OR(source) => {
                let value = match source {
                    Source::A => self.or(self.registers.a),
                    Source::B => self.or(self.registers.b),
                    Source::C => self.or(self.registers.c),
                    Source::D => self.or(self.registers.d),
                    Source::E => self.or(self.registers.e),
                    Source::H => self.or(self.registers.h),
                    Source::L => self.or(self.registers.l),
                    Source::IndirectHL => self.or(self.bus.read_byte(self.registers.get_hl())),
                    Source::D8 => {
                        let v = self.or(self.read_next_byte());
                        let _ = self.pc.wrapping_add(1);
                        v
                    }
                    _ => panic!("unsupported OR source"),
                };
                self.registers.a = value;
                self.pc.wrapping_add(1)
            }
            Instruction::CP(source) => {
                match source {
                    Source::A => self.cp(self.registers.a),
                    Source::B => self.cp(self.registers.b),
                    Source::C => self.cp(self.registers.c),
                    Source::D => self.cp(self.registers.d),
                    Source::E => self.cp(self.registers.e),
                    Source::H => self.cp(self.registers.h),
                    Source::L => self.cp(self.registers.l),
                    Source::IndirectHL => self.cp(self.bus.read_byte(self.registers.get_hl())),
                    Source::D8 => {
                        self.cp(self.read_next_byte());
                        let _ = self.pc.wrapping_add(1);
                    }
                    _ => panic!("unsupported CP source"),
                };
                self.pc.wrapping_add(1)
            }
            Instruction::INC(target) => {
                match target {
                    Target::BC => self
                        .registers
                        .set_bc(self.registers.get_bc().wrapping_add(1)),
                    Target::DE => self
                        .registers
                        .set_de(self.registers.get_de().wrapping_add(1)),
                    Target::HL => self
                        .registers
                        .set_hl(self.registers.get_hl().wrapping_add(1)),
                    Target::SP => self.sp = self.sp.wrapping_add(1),
                    Target::B => self.registers.b = self.inc(self.registers.b),
                    Target::D => self.registers.d = self.inc(self.registers.d),
                    Target::H => self.registers.h = self.inc(self.registers.h),
                    Target::C => self.registers.c = self.inc(self.registers.c),
                    Target::E => self.registers.e = self.inc(self.registers.e),
                    Target::L => self.registers.l = self.inc(self.registers.l),
                    Target::A => self.registers.a = self.inc(self.registers.a),
                    Target::IndirectHL => {
                        let addr = self.registers.get_hl();
                        let mut val = self.bus.read_byte(addr);
                        val = self.inc(val);
                        self.bus.write_byte(addr, val);
                    }
                    _ => panic!("unsupported INC target"),
                };
                self.pc.wrapping_add(1)
            }
            Instruction::DEC(target) => {
                match target {
                    Target::BC => self
                        .registers
                        .set_bc(self.registers.get_bc().wrapping_sub(1)),
                    Target::DE => self
                        .registers
                        .set_de(self.registers.get_de().wrapping_sub(1)),
                    Target::HL => self
                        .registers
                        .set_hl(self.registers.get_hl().wrapping_sub(1)),
                    Target::SP => self.sp = self.sp.wrapping_sub(1),
                    Target::B => self.registers.b = self.dec(self.registers.b),
                    Target::D => self.registers.d = self.dec(self.registers.d),
                    Target::H => self.registers.h = self.dec(self.registers.h),
                    Target::C => self.registers.c = self.dec(self.registers.c),
                    Target::E => self.registers.e = self.dec(self.registers.e),
                    Target::L => self.registers.l = self.dec(self.registers.l),
                    Target::A => self.registers.a = self.dec(self.registers.a),
                    Target::IndirectHL => {
                        let addr = self.registers.get_hl();
                        let mut val = self.bus.read_byte(addr);
                        val = self.dec(val);
                        self.bus.write_byte(addr, val);
                    }
                    _ => panic!("unsupported DEC target"),
                }
                self.pc.wrapping_add(1)
            }
            Instruction::JP(test) => self.jump(self.should_jump(&test)),
            Instruction::LD(load_type) => match load_type {
                LoadType::Byte(target, source) => {
                    let source_value = match source {
                        LoadByteSource::A => self.registers.a,
                        LoadByteSource::B => self.registers.b,
                        LoadByteSource::C => self.registers.c,
                        LoadByteSource::D => self.registers.d,
                        LoadByteSource::E => self.registers.e,
                        LoadByteSource::H => self.registers.h,
                        LoadByteSource::L => self.registers.l,
                        LoadByteSource::D8 => self.read_next_byte(),
                        LoadByteSource::N16I => self.bus.read_byte(self.read_next_word()),
                        LoadByteSource::HLI => self.bus.read_byte(self.registers.get_hl()),
                        LoadByteSource::BCI => self.bus.read_byte(self.registers.get_bc()),
                        LoadByteSource::DEI => self.bus.read_byte(self.registers.get_de()),
                        LoadByteSource::N8I => {
                            self.bus.read_byte(0xFF00 | self.read_next_byte() as u16)
                        }
                        LoadByteSource::CI => self.bus.read_byte(0xFF00 | self.registers.c as u16),
                        LoadByteSource::HLINCR => {
                            let addr = self.registers.get_hl();
                            self.registers
                                .set_hl(self.registers.get_hl().wrapping_add(1));
                            self.bus.read_byte(addr)
                        }
                        LoadByteSource::HLDECR => {
                            let addr = self.registers.get_hl();
                            self.registers
                                .set_hl(self.registers.get_hl().wrapping_sub(1));
                            self.bus.read_byte(addr)
                        }
                    };
                    match target {
                        LoadByteTarget::A => self.registers.a = source_value,
                        LoadByteTarget::B => self.registers.b = source_value,
                        LoadByteTarget::C => self.registers.c = source_value,
                        LoadByteTarget::D => self.registers.d = source_value,
                        LoadByteTarget::E => self.registers.e = source_value,
                        LoadByteTarget::H => self.registers.h = source_value,
                        LoadByteTarget::L => self.registers.l = source_value,
                        LoadByteTarget::N16I => {
                            self.bus.write_byte(self.read_next_word(), source_value);
                            let _ = self.pc.wrapping_add(2);
                        }
                        LoadByteTarget::HLI => {
                            self.bus.write_byte(self.registers.get_hl(), source_value)
                        }
                        LoadByteTarget::BCI => {
                            self.bus.write_byte(self.registers.get_bc(), source_value)
                        }
                        LoadByteTarget::DEI => {
                            self.bus.write_byte(self.registers.get_de(), source_value)
                        }
                        LoadByteTarget::CI => self
                            .bus
                            .write_byte(0xFF00 | self.registers.c as u16, source_value),

                        LoadByteTarget::HLINCR => {
                            let addr = self.registers.get_hl();
                            self.registers
                                .set_hl(self.registers.get_hl().wrapping_add(1));
                            self.bus.write_byte(addr, source_value)
                        }
                        LoadByteTarget::HLDECR => {
                            let addr = self.registers.get_hl();
                            self.registers
                                .set_hl(self.registers.get_hl().wrapping_sub(1));
                            self.bus.write_byte(addr, source_value)
                        }
                        LoadByteTarget::N8I => self
                            .bus
                            .write_byte(0xFF00 | self.read_next_byte() as u16, source_value),
                    };
                    match source {
                        LoadByteSource::D8 => self.pc.wrapping_add(2),
                        LoadByteSource::N16I => self.pc.wrapping_add(3),
                        LoadByteSource::N8I => self.pc.wrapping_add(2),
                        _ => self.pc.wrapping_add(1),
                    }
                }
                LoadType::Word(target, source) => {
                    let source_value = match source {
                        LoadWordSource::D16 => self.read_next_word(),
                        LoadWordSource::SP => self.sp,
                        LoadWordSource::HL => self.registers.get_hl(),
                    };
                    match target {
                        LoadWordTarget::BC => self.registers.set_bc(source_value),
                        LoadWordTarget::DE => self.registers.set_de(source_value),
                        LoadWordTarget::HL => self.registers.set_hl(source_value),
                        LoadWordTarget::N16I => {
                            self.bus.write_word(self.read_next_word(), source_value);
                            let _ = self.pc.wrapping_add(2);
                        }
                        LoadWordTarget::SP => self.sp = source_value,
                    }

                    match source {
                        LoadWordSource::D16 => self.pc.wrapping_add(3),
                        _ => self.pc.wrapping_add(1),
                    }
                }
                LoadType::HLFromSPN => {
                    let value = self.read_next_byte() as i8 as i16 as u16;
                    let result = self.sp.wrapping_add(value);
                    self.registers.set_hl(result);
                    self.registers.f.zero = false;
                    self.registers.f.subtract = false;
                    self.registers.f.half_carry = (self.sp & 0xF) + (value & 0xF) > 0xF;
                    self.registers.f.carry = (self.sp & 0xFF) + (value & 0xFF) > 0xFF;
                    self.pc.wrapping_add(2)
                }
            },
            Instruction::PUSH(target) => {
                let value = match target {
                    StackTarget::BC => self.registers.get_bc(),
                    StackTarget::DE => self.registers.get_de(),
                    StackTarget::HL => self.registers.get_hl(),
                    StackTarget::AF => self.registers.get_af(),
                };
                self.push(value);
                self.pc.wrapping_add(1)
            }
            Instruction::POP(target) => {
                let value = self.pop();
                match target {
                    StackTarget::BC => self.registers.set_bc(value),
                    StackTarget::DE => self.registers.set_de(value),
                    StackTarget::HL => self.registers.set_hl(value),
                    StackTarget::AF => self.registers.set_af(value),
                }
                self.pc.wrapping_add(1)
            }
            Instruction::CALL(test) => self.call(self.should_jump(&test)),
            Instruction::RET(test) => self.return_(self.should_jump(&test)),
            Instruction::NOP => self.pc.wrapping_add(1),
            Instruction::HALT => {
                self.is_halted = true;
                self.pc.wrapping_add(1)
            }
            Instruction::STOP => todo!(),
            Instruction::ADDHL(source) => {
                // F: - 0 H C
                let value = match source {
                    Source::BC => self.registers.get_bc(),
                    Source::DE => self.registers.get_de(),
                    Source::HL => self.registers.get_hl(),
                    Source::SP => self.sp,
                    _ => panic!("unsupported ADDHL source"),
                };
                let hl = self.registers.get_hl();
                let (new_value, did_overflow) = hl.overflowing_add(value);
                self.registers.f.subtract = false;
                // Half carry tests if we flow over the 11th bit i.e. does adding the two
                // numbers together cause the 11th bit to flip
                let mask = 0b111_1111_1111; // mask out bits 11-15
                self.registers.f.half_carry = (value & mask) + (hl & mask) > mask;
                self.registers.f.carry = did_overflow;

                self.registers.set_hl(new_value);
                self.pc.wrapping_add(1)
            }
            Instruction::ADDSP => {
                // F: 0 0 H C
                let value = self.read_next_byte() as i8 as u8 as u16;
                let new_value = value.wrapping_add(value);
                self.registers.f.zero = false;
                self.registers.f.subtract = false;
                let half_carry_mask = 0xF;
                self.registers.f.half_carry =
                    (self.sp & half_carry_mask) + (value & half_carry_mask) > half_carry_mask;
                let carry_mask = 0xFF;
                self.registers.f.carry = (self.sp & carry_mask) + (value & carry_mask) > carry_mask;

                self.sp = new_value;
                self.pc.wrapping_add(2)
            }
        }
    }
    fn add(&mut self, value: u8, with_carry: bool) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        let carry = if with_carry && self.registers.f.carry {
            1
        } else {
            0
        };
        let (new_value, did_overflow2) = new_value.overflowing_add(carry);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow || did_overflow2;
        self.registers.f.half_carry = ((self.registers.a & 0xF) + (value & 0xF) + carry) > 0xF;
        new_value
    }
    fn should_jump(&self, test: &JumpTest) -> bool {
        let jump_condition = match test {
            JumpTest::NotZero => !self.registers.f.zero,
            JumpTest::Zero => self.registers.f.zero,
            JumpTest::NotCarry => !self.registers.f.carry,
            JumpTest::Carry => self.registers.f.carry,
            JumpTest::Always => true,
        };
        jump_condition
    }
    fn jump(&mut self, should_jump: bool) -> u16 {
        if should_jump {
            let least_significant_byte = self.bus.read_byte(self.pc + 1) as u16;
            let most_significant_byte = self.bus.read_byte(self.pc + 2) as u16;
            (most_significant_byte << 8) | least_significant_byte
        } else {
            self.pc.wrapping_add(3)
        }
    }
    fn push(&mut self, value: u16) {
        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, ((value & 0xFF00) >> 8) as u8);

        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, (value & 0xFF) as u8);
    }
    fn pop(&mut self) -> u16 {
        let least_significant_byte = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);
        let most_significant_byte = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);
        (most_significant_byte << 8) | least_significant_byte
    }
    fn call(&mut self, should_jump: bool) -> u16 {
        let next_pc = self.pc.wrapping_add(3);
        if should_jump {
            self.push(next_pc);
            self.read_next_word()
        } else {
            next_pc
        }
    }
    fn return_(&mut self, should_jump: bool) -> u16 {
        if should_jump {
            self.pop()
        } else {
            self.pc.wrapping_add(1)
        }
    }
    fn inc(&mut self, value: u8) -> u8 {
        let new_value = value.wrapping_add(1);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        // Half Carry is set if the lower nibble of the value is equal to 0xF.
        // If the nibble is equal to 0xF (0b1111) that means incrementing the value
        // by 1 would cause a carry from the lower nibble to the upper nibble.
        self.registers.f.half_carry = value & 0xF == 0xF;
        new_value
    }
    fn dec(&mut self, value: u8) -> u8 {
        let new_value = value.wrapping_sub(1);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = value & 0xF == 0xF;
        new_value
    }
    fn sub(&mut self, value: u8, with_carry: bool) -> u8 {
        // Z1HC
        let (new_value, did_overflow) = self.registers.a.overflowing_sub(value);
        let carry = if with_carry && self.registers.f.carry {
            1
        } else {
            0
        };
        let (new_value, did_overflow2) = new_value.overflowing_sub(carry);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = (self.registers.a & 0xF) < (value & 0xF) + carry;
        self.registers.f.carry = did_overflow || did_overflow2;
        new_value
    }
    fn and(&mut self, value: u8) -> u8 {
        // Z010
        let new_value = self.registers.a & value;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = true;
        self.registers.f.carry = false;
        new_value
    }
    fn xor(&mut self, value: u8) -> u8 {
        // Z000
        let new_value = self.registers.a ^ value;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;
        new_value
    }
    fn or(&mut self, value: u8) -> u8 {
        // Z000
        let new_value = self.registers.a | value;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;
        new_value
    }
    fn cp(&mut self, value: u8) {
        // Z1HC
        self.registers.f.zero = self.registers.a == value;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = (0x0f & self.registers.a) > (0x0f & value);
        self.registers.f.carry = self.registers.a < value;
    }
}
