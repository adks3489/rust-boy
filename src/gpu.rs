const SCREEN_WIDTH: usize = 160;
const SCREEN_HEIGHT: usize = 144;

pub struct GPU {
    pub canvas: [u32; SCREEN_WIDTH * SCREEN_HEIGHT],
    // VideoMemory, Sprite Attribute Memory
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
            ReadMode::VerticalBlank => todo!(),
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
