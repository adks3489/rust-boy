const SCREEN_WIDTH: usize = 160;
const SCREEN_HEIGHT: usize = 144;
pub const RAM_SIZE: usize = 0x2000;
pub const SPRITE_SIZE: usize = 0xA0;

pub struct GPU {
    pub canvas: [u32; SCREEN_WIDTH * SCREEN_HEIGHT],
    pub ram: [u8; RAM_SIZE],
    pub sprite: [u8; SPRITE_SIZE],
    read_mode: ReadMode,
    clock: u32,
    line: u32,
}

enum ReadMode {
    HorizontalBlank,
    VerticalBlank,
    ScanlineOAM,
    ScanlineVRAM,
}

impl GPU {
    pub fn new() -> Self {
        GPU {
            canvas: [0; SCREEN_WIDTH * SCREEN_HEIGHT],
            ram: [0; RAM_SIZE],
            sprite: [0; SPRITE_SIZE],
            read_mode: ReadMode::HorizontalBlank,
            clock: 0,
            line: 0,
        }
    }

    fn step(&mut self) {
        match self.read_mode {
            ReadMode::HorizontalBlank => {
                if self.clock >= 204 {
                    self.clock = 0;
                    self.line += 1;
                    if self.line == SCREEN_HEIGHT as u32 - 1 {
                        self.read_mode = ReadMode::VerticalBlank;
                        // todo: draw buffer
                    } else {
                        self.read_mode = ReadMode::ScanlineOAM;
                    }
                }
            }
            ReadMode::VerticalBlank => {
                if self.clock >= 456 {
                    self.clock = 0;
                    self.line += 1;
                    if self.line > 153 {
                        self.read_mode = ReadMode::ScanlineOAM;
                        self.line = 0;
                    }
                }
            }
            ReadMode::ScanlineOAM => {
                if self.clock >= 80 {
                    self.clock = 0;
                    self.read_mode = ReadMode::ScanlineVRAM;
                }
            }
            ReadMode::ScanlineVRAM => {
                if self.clock >= 172 {
                    self.clock = 0;
                    self.read_mode = ReadMode::HorizontalBlank;
                    // todo: render_scan
                }
            }
        }
    }
}
