const SCREEN_WIDTH: usize = 160;
const SCREEN_HEIGHT: usize = 144;
pub const RAM_SIZE: usize = 0x2000;
pub const SPRITE_SIZE: usize = 0xA0;

const TILE_SET0_FIRST: usize = 0x800;
const TILE_SET0_LAST: usize = 0x17FF;
const TILE_SET1_FIRST: usize = 0x0;
const TILE_SET1_LAST: usize = 0xFFF;
const TILE_MAP0_FIRST: usize = 0x1800;
const TILE_MAP0_LAST: usize = 0x1BFF;
const TILE_MAP1_FIRST: usize = 0x1C00;
const TILE_MAP1_LAST: usize = 0x1FFF;

const BYTES_PER_TILE: usize = 16;
const PIXELS_PER_TILE: usize = 64;
const TILE_COUNT: usize = 256;
const BACKGROUND_WIDTH: usize = 256;

#[derive(Clone, Copy)]
struct Color(u8, u8, u8);
const COLOR_WHITE: Color = Color(255, 255, 255);
const COLOR_LIGHTGREY: Color = Color(192, 192, 192);
const COLOR_DARKGREY: Color = Color(96, 96, 96);
const COLOR_BLACK: Color = Color(0, 0, 0);

impl From<u8> for Color {
    fn from(v: u8) -> Self {
        match v {
            0 => COLOR_WHITE,
            1 => COLOR_LIGHTGREY,
            2 => COLOR_DARKGREY,
            3 => COLOR_BLACK,
            _ => panic!("unknown color"),
        }
    }
}
type Tile = [Color; PIXELS_PER_TILE];

pub struct GPU {
    pub background: [u32; BACKGROUND_WIDTH * BACKGROUND_WIDTH],
    pub canvas: [u32; SCREEN_WIDTH * SCREEN_HEIGHT],
    pub ram: [u8; RAM_SIZE],
    pub sprite: [u8; SPRITE_SIZE],
    tile_set: [[Tile; TILE_COUNT]; 2],
    read_mode: ReadMode,
    clock: u32,
    line: u32,

    scroll_x: usize,
    scroll_y: usize,
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
            background: [0xFFFF; BACKGROUND_WIDTH * BACKGROUND_WIDTH],
            canvas: [0; SCREEN_WIDTH * SCREEN_HEIGHT],
            ram: [0; RAM_SIZE],
            sprite: [0; SPRITE_SIZE],
            tile_set: [[[COLOR_WHITE; PIXELS_PER_TILE]; TILE_COUNT]; 2],
            read_mode: ReadMode::HorizontalBlank,
            clock: 0,
            line: 0,
            scroll_x: 0,
            scroll_y: 0,
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
                    self.render_scan_line();
                }
            }
        }
    }

    fn read_tiles(&mut self, set_idx: usize) {
        let address = match set_idx {
            0 => TILE_SET0_FIRST..=TILE_SET0_LAST,
            1 => TILE_SET1_FIRST..=TILE_SET1_LAST,
            _ => panic!(),
        };
        self.ram[address]
            .chunks(BYTES_PER_TILE)
            .enumerate()
            .for_each(|(tile_pos, chunk)| {
                chunk
                    .chunks(2)
                    .flat_map(|b| GPU::tile_line_color(b[0], b[1]))
                    .enumerate()
                    .for_each(|(n, colors)| {
                        self.tile_set[set_idx][tile_pos][n] = colors;
                    });
            });
    }

    fn render_scan_line(&mut self) {
        let addr = self.line;
    }

    fn tile_line_color(b1: u8, b2: u8) -> [Color; 8] {
        [
            Color::from(b1 & 0b10000000 | b2 & 0b10000000),
            Color::from(b1 & 0b01000000 | b2 & 0b01000000),
            Color::from(b1 & 0b00100000 | b2 & 0b00100000),
            Color::from(b1 & 0b00010000 | b2 & 0b00010000),
            Color::from(b1 & 0b00001000 | b2 & 0b00001000),
            Color::from(b1 & 0b00000100 | b2 & 0b00000100),
            Color::from(b1 & 0b00000010 | b2 & 0b00000010),
            Color::from(b1 & 0b00000001 | b2 & 0b00000001),
        ]
    }
}
