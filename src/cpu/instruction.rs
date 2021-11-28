pub type InstructionSize = u8;
pub enum Instruction {
    LD(LoadType),
    ADD(ArithmeticTarget),
    ADDHL(Source),
    ADDSP,
    ADC(Source),
    SUB(Source),
    SBC(Source),
    AND(Source),
    XOR(Source),
    OR(Source),
    CP(Source),
    DAA,
    SCF,
    CPL,
    CCF,
    INC(Target),
    DEC(Target),
    JP(JumpTest),
    PUSH(StackTarget),
    POP(StackTarget),
    CALL(JumpTest),
    RET(JumpTest),
    NOP,
    STOP,
    HALT,
}
impl Instruction {
    pub fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction> {
        if prefixed {
            Instruction::from_byte_prefixed(byte)
        } else {
            Instruction::from_byte_not_prefixed(byte)
        }
    }
    fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x0 => todo!(),
            0x01 => todo!(),
            0x02 => todo!(),
            0x03 => todo!(),
            0x04 => todo!(),
            0x05 => todo!(),
            0x06 => todo!(),
            0x07 => todo!(),
            0x08 => todo!(),
            0x09 => todo!(),
            0x0A => todo!(),
            0x0B => todo!(),
            0x0C => todo!(),
            0x0D => todo!(),
            0x0E => todo!(),
            0x0F => todo!(),
            0x10 => todo!(),
            0x11 => todo!(),
            0x12 => todo!(),
            0x13 => todo!(),
            0x14 => todo!(),
            0x15 => todo!(),
            0x16 => todo!(),
            0x17 => todo!(),
            0x18 => todo!(),
            0x19 => todo!(),
            0x1A => todo!(),
            0x1B => todo!(),
            0x1C => todo!(),
            0x1D => todo!(),
            0x1E => todo!(),
            0x1F => todo!(),
            0x20 => todo!(),
            0x21 => todo!(),
            0x22 => todo!(),
            0x23 => todo!(),
            0x24 => todo!(),
            0x25 => todo!(),
            0x26 => todo!(),
            0x27 => todo!(),
            0x28 => todo!(),
            0x29 => todo!(),
            0x2A => todo!(),
            0x2B => todo!(),
            0x2C => todo!(),
            0x2D => todo!(),
            0x2E => todo!(),
            0x2F => todo!(),
            0x30 => todo!(),
            0x31 => todo!(),
            0x32 => todo!(),
            0x33 => todo!(),
            0x34 => todo!(),
            0x35 => todo!(),
            0x36 => todo!(),
            0x37 => todo!(),
            0x38 => todo!(),
            0x39 => todo!(),
            0x3A => todo!(),
            0x3B => todo!(),
            0x3C => todo!(),
            0x3D => todo!(),
            0x3E => todo!(),
            0x3F => todo!(),
            0x40 => todo!(),
            0x41 => todo!(),
            0x42 => todo!(),
            0x43 => todo!(),
            0x44 => todo!(),
            0x45 => todo!(),
            0x46 => todo!(),
            0x47 => todo!(),
            0x48 => todo!(),
            0x49 => todo!(),
            0x4A => todo!(),
            0x4B => todo!(),
            0x4C => todo!(),
            0x4D => todo!(),
            0x4E => todo!(),
            0x4F => todo!(),
            0x50 => todo!(),
            0x51 => todo!(),
            0x52 => todo!(),
            0x53 => todo!(),
            0x54 => todo!(),
            0x55 => todo!(),
            0x56 => todo!(),
            0x57 => todo!(),
            0x58 => todo!(),
            0x59 => todo!(),
            0x5A => todo!(),
            0x5B => todo!(),
            0x5C => todo!(),
            0x5D => todo!(),
            0x5E => todo!(),
            0x5F => todo!(),
            0x60 => todo!(),
            0x61 => todo!(),
            0x62 => todo!(),
            0x63 => todo!(),
            0x64 => todo!(),
            0x65 => todo!(),
            0x66 => todo!(),
            0x67 => todo!(),
            0x68 => todo!(),
            0x69 => todo!(),
            0x6A => todo!(),
            0x6B => todo!(),
            0x6C => todo!(),
            0x6D => todo!(),
            0x6E => todo!(),
            0x6F => todo!(),
            0x70 => todo!(),
            0x71 => todo!(),
            0x72 => todo!(),
            0x73 => todo!(),
            0x74 => todo!(),
            0x75 => todo!(),
            0x76 => todo!(),
            0x77 => todo!(),
            0x78 => todo!(),
            0x79 => todo!(),
            0x7A => todo!(),
            0x7B => todo!(),
            0x7C => todo!(),
            0x7D => todo!(),
            0x7E => todo!(),
            0x7F => todo!(),
            0x80 => todo!(),
            0x81 => todo!(),
            0x82 => todo!(),
            0x83 => todo!(),
            0x84 => todo!(),
            0x85 => todo!(),
            0x86 => todo!(),
            0x87 => todo!(),
            0x88 => todo!(),
            0x89 => todo!(),
            0x8A => todo!(),
            0x8B => todo!(),
            0x8C => todo!(),
            0x8D => todo!(),
            0x8E => todo!(),
            0x8F => todo!(),
            0x90 => todo!(),
            0x91 => todo!(),
            0x92 => todo!(),
            0x93 => todo!(),
            0x94 => todo!(),
            0x95 => todo!(),
            0x96 => todo!(),
            0x97 => todo!(),
            0x98 => todo!(),
            0x99 => todo!(),
            0x9A => todo!(),
            0x9B => todo!(),
            0x9C => todo!(),
            0x9D => todo!(),
            0x9E => todo!(),
            0x9F => todo!(),
            0xA0 => todo!(),
            0xA1 => todo!(),
            0xA2 => todo!(),
            0xA3 => todo!(),
            0xA4 => todo!(),
            0xA5 => todo!(),
            0xA6 => todo!(),
            0xA7 => todo!(),
            0xA8 => todo!(),
            0xA9 => todo!(),
            0xAA => todo!(),
            0xAB => todo!(),
            0xAC => todo!(),
            0xAD => todo!(),
            0xAE => todo!(),
            0xAF => todo!(),
            0xB0 => todo!(),
            0xB1 => todo!(),
            0xB2 => todo!(),
            0xB3 => todo!(),
            0xB4 => todo!(),
            0xB5 => todo!(),
            0xB6 => todo!(),
            0xB7 => todo!(),
            0xB8 => todo!(),
            0xB9 => todo!(),
            0xBA => todo!(),
            0xBB => todo!(),
            0xBC => todo!(),
            0xBD => todo!(),
            0xBE => todo!(),
            0xBF => todo!(),
            0xC0 => todo!(),
            0xC1 => todo!(),
            0xC2 => todo!(),
            0xC3 => todo!(),
            0xC4 => todo!(),
            0xC5 => todo!(),
            0xC6 => todo!(),
            0xC7 => todo!(),
            0xC8 => todo!(),
            0xC9 => todo!(),
            0xCA => todo!(),
            0xCB => todo!(),
            0xCC => todo!(),
            0xCD => todo!(),
            0xCE => todo!(),
            0xCF => todo!(),
            0xD0 => todo!(),
            0xD1 => todo!(),
            0xD2 => todo!(),
            0xD3 => todo!(),
            0xD4 => todo!(),
            0xD5 => todo!(),
            0xD6 => todo!(),
            0xD7 => todo!(),
            0xD8 => todo!(),
            0xD9 => todo!(),
            0xDA => todo!(),
            0xDB => todo!(),
            0xDC => todo!(),
            0xDD => todo!(),
            0xDE => todo!(),
            0xDF => todo!(),
            0xE0 => todo!(),
            0xE1 => todo!(),
            0xE2 => todo!(),
            0xE3 => todo!(),
            0xE4 => todo!(),
            0xE5 => todo!(),
            0xE6 => todo!(),
            0xE7 => todo!(),
            0xE8 => todo!(),
            0xE9 => todo!(),
            0xEA => todo!(),
            0xEB => todo!(),
            0xEC => todo!(),
            0xED => todo!(),
            0xEE => todo!(),
            0xEF => todo!(),
            0xF0 => todo!(),
            0xF1 => todo!(),
            0xF2 => todo!(),
            0xF3 => todo!(),
            0xF4 => todo!(),
            0xF5 => todo!(),
            0xF6 => todo!(),
            0xF7 => todo!(),
            0xF8 => todo!(),
            0xF9 => todo!(),
            0xFA => todo!(),
            0xFB => todo!(),
            0xFC => todo!(),
            0xFD => todo!(),
            0xFE => todo!(),
            0xFF => todo!(),
            _ => None,
        }
    }
    fn from_byte_not_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x0 => Some(Instruction::NOP),
            0x1 => Some(Instruction::LD(LoadType::Word(
                LoadWordTarget::BC,
                LoadWordSource::D16,
            ))),
            0x2 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::BCI,
                LoadByteSource::A,
            ))),
            0x3 => Some(Instruction::INC(Target::BC)),
            0x4 => Some(Instruction::INC(Target::B)),
            0x5 => Some(Instruction::DEC(Target::B)),
            0x6 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::D8,
            ))),
            0x7 => todo!(),
            0x8 => Some(Instruction::LD(LoadType::Word(
                LoadWordTarget::N16I,
                LoadWordSource::SP,
            ))),
            0x9 => Some(Instruction::ADDHL(Source::BC)),
            0xA => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::BCI,
            ))),
            0xB => Some(Instruction::DEC(Target::BC)),
            0xC => Some(Instruction::INC(Target::C)),
            0xD => Some(Instruction::DEC(Target::C)),
            0xE => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::C,
                LoadByteSource::D8,
            ))),
            0xF => todo!(),
            0x10 => Some(Instruction::STOP),
            0x11 => Some(Instruction::LD(LoadType::Word(
                LoadWordTarget::DE,
                LoadWordSource::D16,
            ))),
            0x12 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::DEI,
                LoadByteSource::A,
            ))),
            0x13 => Some(Instruction::INC(Target::DE)),
            0x14 => Some(Instruction::INC(Target::D)),
            0x15 => Some(Instruction::DEC(Target::D)),
            0x16 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::D,
                LoadByteSource::D8,
            ))),
            0x17 => todo!(),
            0x18 => todo!(),
            0x19 => Some(Instruction::ADDHL(Source::DE)),
            0x1A => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::DEI,
            ))),
            0x1B => Some(Instruction::DEC(Target::DE)),
            0x1C => Some(Instruction::INC(Target::E)),
            0x1D => Some(Instruction::DEC(Target::E)),
            0x1E => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::E,
                LoadByteSource::D8,
            ))),
            0x1F => todo!(),
            0x20 => todo!(),
            0x21 => Some(Instruction::LD(LoadType::Word(
                LoadWordTarget::HL,
                LoadWordSource::D16,
            ))),
            0x22 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::HLINCR,
                LoadByteSource::A,
            ))),
            0x23 => Some(Instruction::INC(Target::HL)),
            0x24 => Some(Instruction::INC(Target::H)),
            0x25 => Some(Instruction::DEC(Target::H)),
            0x26 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::H,
                LoadByteSource::D8,
            ))),
            0x27 => Some(Instruction::DAA),
            0x28 => todo!(),
            0x29 => Some(Instruction::ADDHL(Source::HL)),
            0x2A => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::HLINCR,
            ))),
            0x2B => Some(Instruction::DEC(Target::HL)),
            0x2C => Some(Instruction::INC(Target::L)),
            0x2D => Some(Instruction::DEC(Target::L)),
            0x2E => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::L,
                LoadByteSource::D8,
            ))),
            0x2F => Some(Instruction::CPL),
            0x30 => todo!(),
            0x31 => Some(Instruction::LD(LoadType::Word(
                LoadWordTarget::SP,
                LoadWordSource::D16,
            ))),
            0x32 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::HLDECR,
                LoadByteSource::A,
            ))),
            0x33 => Some(Instruction::INC(Target::SP)),
            0x34 => Some(Instruction::INC(Target::IndirectHL)),
            0x35 => Some(Instruction::DEC(Target::IndirectHL)),
            0x36 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::HLI,
                LoadByteSource::D8,
            ))),
            0x37 => Some(Instruction::SCF),
            0x38 => todo!(),
            0x39 => Some(Instruction::ADDHL(Source::SP)),
            0x3A => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::HLDECR,
            ))),
            0x3B => Some(Instruction::DEC(Target::SP)),
            0x3C => Some(Instruction::INC(Target::A)),
            0x3D => Some(Instruction::DEC(Target::A)),
            0x3E => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::D8,
            ))),
            0x3F => Some(Instruction::CCF),
            0x40 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::B,
            ))),
            0x41 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::C,
            ))),
            0x42 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::D,
            ))),
            0x43 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::E,
            ))),
            0x44 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::H,
            ))),
            0x45 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::L,
            ))),
            0x46 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::HLI,
            ))),
            0x47 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::A,
            ))),
            0x48 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::C,
                LoadByteSource::B,
            ))),
            0x49 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::C,
                LoadByteSource::C,
            ))),
            0x4A => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::C,
                LoadByteSource::D,
            ))),
            0x4B => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::C,
                LoadByteSource::E,
            ))),
            0x4C => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::C,
                LoadByteSource::H,
            ))),
            0x4D => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::C,
                LoadByteSource::L,
            ))),
            0x4E => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::C,
                LoadByteSource::HLI,
            ))),
            0x4F => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::C,
                LoadByteSource::A,
            ))),
            0x50 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::D,
                LoadByteSource::B,
            ))),
            0x51 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::D,
                LoadByteSource::C,
            ))),
            0x52 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::D,
                LoadByteSource::D,
            ))),
            0x53 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::D,
                LoadByteSource::E,
            ))),
            0x54 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::D,
                LoadByteSource::H,
            ))),
            0x55 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::D,
                LoadByteSource::L,
            ))),
            0x56 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::D,
                LoadByteSource::HLI,
            ))),
            0x57 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::D,
                LoadByteSource::A,
            ))),
            0x58 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::E,
                LoadByteSource::B,
            ))),
            0x59 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::E,
                LoadByteSource::C,
            ))),
            0x5A => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::E,
                LoadByteSource::D,
            ))),
            0x5B => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::E,
                LoadByteSource::E,
            ))),
            0x5C => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::E,
                LoadByteSource::H,
            ))),
            0x5D => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::E,
                LoadByteSource::L,
            ))),
            0x5E => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::E,
                LoadByteSource::HLI,
            ))),
            0x5F => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::E,
                LoadByteSource::A,
            ))),
            0x60 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::H,
                LoadByteSource::B,
            ))),
            0x61 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::H,
                LoadByteSource::C,
            ))),
            0x62 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::H,
                LoadByteSource::D,
            ))),
            0x63 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::H,
                LoadByteSource::E,
            ))),
            0x64 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::H,
                LoadByteSource::H,
            ))),
            0x65 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::H,
                LoadByteSource::L,
            ))),
            0x66 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::H,
                LoadByteSource::HLI,
            ))),
            0x67 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::H,
                LoadByteSource::A,
            ))),
            0x68 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::L,
                LoadByteSource::B,
            ))),
            0x69 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::L,
                LoadByteSource::C,
            ))),
            0x6A => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::L,
                LoadByteSource::D,
            ))),
            0x6B => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::L,
                LoadByteSource::E,
            ))),
            0x6C => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::L,
                LoadByteSource::H,
            ))),
            0x6D => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::L,
                LoadByteSource::L,
            ))),
            0x6E => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::L,
                LoadByteSource::HLI,
            ))),
            0x6F => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::L,
                LoadByteSource::A,
            ))),
            0x70 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::HLI,
                LoadByteSource::B,
            ))),
            0x71 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::HLI,
                LoadByteSource::C,
            ))),
            0x72 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::HLI,
                LoadByteSource::D,
            ))),
            0x73 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::HLI,
                LoadByteSource::E,
            ))),
            0x74 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::HLI,
                LoadByteSource::H,
            ))),
            0x75 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::HLI,
                LoadByteSource::L,
            ))),
            0x76 => Some(Instruction::HALT),
            0x77 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::HLI,
                LoadByteSource::A,
            ))),
            0x78 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::B,
            ))),
            0x79 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::C,
            ))),
            0x7A => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::D,
            ))),
            0x7B => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::E,
            ))),
            0x7C => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::H,
            ))),
            0x7D => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::L,
            ))),
            0x7E => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::HLI,
            ))),
            0x7F => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::A,
            ))),
            0x80 => Some(Instruction::ADD(ArithmeticTarget::B)),
            0x81 => Some(Instruction::ADD(ArithmeticTarget::C)),
            0x82 => Some(Instruction::ADD(ArithmeticTarget::D)),
            0x83 => Some(Instruction::ADD(ArithmeticTarget::E)),
            0x84 => Some(Instruction::ADD(ArithmeticTarget::H)),
            0x85 => Some(Instruction::ADD(ArithmeticTarget::L)),
            0x86 => Some(Instruction::ADD(ArithmeticTarget::HLI)),
            0x87 => Some(Instruction::ADD(ArithmeticTarget::A)),
            0x88 => Some(Instruction::ADC(Source::B)),
            0x89 => Some(Instruction::ADC(Source::C)),
            0x8A => Some(Instruction::ADC(Source::D)),
            0x8B => Some(Instruction::ADC(Source::E)),
            0x8C => Some(Instruction::ADC(Source::H)),
            0x8D => Some(Instruction::ADC(Source::L)),
            0x8E => Some(Instruction::ADC(Source::IndirectHL)),
            0x8F => Some(Instruction::ADC(Source::A)),
            0x90 => Some(Instruction::SUB(Source::B)),
            0x91 => Some(Instruction::SUB(Source::C)),
            0x92 => Some(Instruction::SUB(Source::D)),
            0x93 => Some(Instruction::SUB(Source::E)),
            0x94 => Some(Instruction::SUB(Source::H)),
            0x95 => Some(Instruction::SUB(Source::L)),
            0x96 => Some(Instruction::SUB(Source::IndirectHL)),
            0x97 => Some(Instruction::SUB(Source::A)),
            0x98 => Some(Instruction::SBC(Source::B)),
            0x99 => Some(Instruction::SBC(Source::C)),
            0x9A => Some(Instruction::SBC(Source::D)),
            0x9B => Some(Instruction::SBC(Source::E)),
            0x9C => Some(Instruction::SBC(Source::H)),
            0x9D => Some(Instruction::SBC(Source::L)),
            0x9E => Some(Instruction::SBC(Source::IndirectHL)),
            0x9F => Some(Instruction::SBC(Source::A)),
            0xA0 => Some(Instruction::AND(Source::B)),
            0xA1 => Some(Instruction::AND(Source::C)),
            0xA2 => Some(Instruction::AND(Source::D)),
            0xA3 => Some(Instruction::AND(Source::E)),
            0xA4 => Some(Instruction::AND(Source::H)),
            0xA5 => Some(Instruction::AND(Source::L)),
            0xA6 => Some(Instruction::AND(Source::IndirectHL)),
            0xA7 => Some(Instruction::AND(Source::A)),
            0xA8 => Some(Instruction::XOR(Source::B)),
            0xA9 => Some(Instruction::XOR(Source::C)),
            0xAA => Some(Instruction::XOR(Source::D)),
            0xAB => Some(Instruction::XOR(Source::E)),
            0xAC => Some(Instruction::XOR(Source::H)),
            0xAD => Some(Instruction::XOR(Source::L)),
            0xAE => Some(Instruction::XOR(Source::IndirectHL)),
            0xAF => Some(Instruction::XOR(Source::A)),
            0xB0 => Some(Instruction::OR(Source::B)),
            0xB1 => Some(Instruction::OR(Source::C)),
            0xB2 => Some(Instruction::OR(Source::D)),
            0xB3 => Some(Instruction::OR(Source::E)),
            0xB4 => Some(Instruction::OR(Source::H)),
            0xB5 => Some(Instruction::OR(Source::L)),
            0xB6 => Some(Instruction::OR(Source::IndirectHL)),
            0xB7 => Some(Instruction::OR(Source::A)),
            0xB8 => Some(Instruction::CP(Source::B)),
            0xB9 => Some(Instruction::CP(Source::C)),
            0xBA => Some(Instruction::CP(Source::D)),
            0xBB => Some(Instruction::CP(Source::E)),
            0xBC => Some(Instruction::CP(Source::H)),
            0xBD => Some(Instruction::CP(Source::L)),
            0xBE => Some(Instruction::CP(Source::IndirectHL)),
            0xBF => Some(Instruction::CP(Source::A)),
            0xC0 => Some(Instruction::RET(JumpTest::NotZero)),
            0xC1 => Some(Instruction::POP(StackTarget::BC)),
            0xC2 => todo!(),
            0xC3 => todo!(),
            0xC4 => Some(Instruction::CALL(JumpTest::NotZero)),
            0xC5 => Some(Instruction::PUSH(StackTarget::BC)),
            0xC6 => Some(Instruction::ADD(ArithmeticTarget::D8)),
            0xC7 => todo!(),
            0xC8 => Some(Instruction::RET(JumpTest::Zero)),
            0xC9 => Some(Instruction::RET(JumpTest::Always)),
            0xCA => todo!(),
            0xCB => todo!(),
            0xCC => Some(Instruction::CALL(JumpTest::Zero)),
            0xCD => Some(Instruction::CALL(JumpTest::Always)),
            0xCE => Some(Instruction::ADC(Source::D8)),
            0xCF => todo!(),
            0xD0 => Some(Instruction::RET(JumpTest::NotCarry)),
            0xD1 => Some(Instruction::POP(StackTarget::DE)),
            0xD2 => todo!(),
            0xD3 => todo!(),
            0xD4 => Some(Instruction::CALL(JumpTest::NotCarry)),
            0xD5 => Some(Instruction::PUSH(StackTarget::DE)),
            0xD6 => Some(Instruction::SUB(Source::D8)),
            0xD7 => todo!(),
            0xD8 => Some(Instruction::RET(JumpTest::Carry)),
            0xD9 => todo!(),
            0xDA => todo!(),
            0xDB => todo!(),
            0xDC => Some(Instruction::CALL(JumpTest::Carry)),
            0xDD => todo!(),
            0xDE => Some(Instruction::SBC(Source::D8)),
            0xDF => todo!(),
            0xE0 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::N8I,
                LoadByteSource::A,
            ))),
            0xE1 => Some(Instruction::POP(StackTarget::HL)),
            0xE2 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::CI,
                LoadByteSource::A,
            ))),
            0xE3 => todo!(),
            0xE4 => todo!(),
            0xE5 => Some(Instruction::PUSH(StackTarget::HL)),
            0xE6 => Some(Instruction::AND(Source::D8)),
            0xE7 => todo!(),
            0xE8 => Some(Instruction::ADDSP),
            0xE9 => todo!(),
            0xEA => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::N16I,
                LoadByteSource::A,
            ))),
            0xEB => todo!(),
            0xEC => todo!(),
            0xED => todo!(),
            0xEE => Some(Instruction::XOR(Source::D8)),
            0xEF => todo!(),
            0xF0 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::N8I,
            ))),
            0xF1 => Some(Instruction::POP(StackTarget::AF)),
            0xF2 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::CI,
            ))),
            0xF3 => todo!(),
            0xF4 => todo!(),
            0xF5 => Some(Instruction::PUSH(StackTarget::AF)),
            0xF6 => Some(Instruction::OR(Source::D8)),
            0xF7 => todo!(),
            0xF8 => Some(Instruction::LD(LoadType::HLFromSPN)),
            0xF9 => Some(Instruction::LD(LoadType::Word(
                LoadWordTarget::SP,
                LoadWordSource::HL,
            ))),
            0xFA => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::N16I,
            ))),
            0xFB => todo!(),
            0xFC => todo!(),
            0xFD => todo!(),
            0xFE => todo!(),
            0xFF => todo!(),
            _ => None,
        }
    }
}
pub type Source = Target;
pub enum Target {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    AF,
    BC,
    DE,
    HL,
    SP,
    IndirectHL,
    D8,
}

pub enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HLI,
    D8,
}

pub enum JumpTest {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always,
}

pub enum LoadByteTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    N16I,
    HLI,
    BCI,
    DEI,
    N8I,
    CI,
    HLINCR,
    HLDECR,
}
pub enum LoadByteSource {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    D8,
    N16I,
    N8I,
    HLI,
    BCI,
    DEI,
    CI,
    HLINCR,
    HLDECR,
}
pub enum LoadWordTarget {
    BC,
    DE,
    HL,
    N16I,
    SP,
}
pub enum LoadWordSource {
    D16,
    SP,
    HL,
}
pub enum LoadType {
    Byte(LoadByteTarget, LoadByteSource),
    Word(LoadWordTarget, LoadWordSource),
    HLFromSPN,
}

pub enum StackTarget {
    BC,
    DE,
    HL,
    AF,
}
