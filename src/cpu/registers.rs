pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: FlagsRegister,
    pub h: u8,
    pub l: u8,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: FlagsRegister::new(),
            h: 0,
            l: 0,
        }
    }
    // af, bc, de, hl
    pub fn get_af(&self) -> u16 {
        (self.a as u16) << 8 | u8::from(self.f) as u16
    }
    pub fn set_af(&mut self, value: u16) {
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.f = ((value & 0xFF) as u8).into();
    }
    pub fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }
    pub fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }
    pub fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }
    pub fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }
    pub fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.h as u16
    }
    pub fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }
}
#[derive(Clone, Copy)]
pub struct FlagsRegister {
    pub zero: bool,
    pub subtract: bool,
    pub half_carry: bool,
    pub carry: bool,
}
impl FlagsRegister {
    fn new() -> Self {
        FlagsRegister {
            zero: false,
            subtract: false,
            half_carry: false,
            carry: false,
        }
    }
}
const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;
impl std::convert::From<FlagsRegister> for u8 {
    fn from(flag: FlagsRegister) -> Self {
        (if flag.zero { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION
            | (if flag.subtract { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSITION
            | (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION
            | (if flag.carry { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
    }
}
impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        FlagsRegister {
            zero: ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0,
            subtract: ((byte >> SUBTRACT_FLAG_BYTE_POSITION) & 0b1) != 0,
            half_carry: ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0b1) != 0,
            carry: ((byte >> CARRY_FLAG_BYTE_POSITION) & 0b1) != 0,
        }
    }
}
