static FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80, 0xF0, 0xF0,
    0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0, 0x10, 0xF0, 0xF0, 0x80,
    0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90, 0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0,
    0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0, 0x90, 0xE0, 0x90, 0xE0, 0xF0, 0x80, 0x80, 0x80,
    0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0, 0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80,
];

fn get_bools_of_byte(byte: u8) -> [bool; 8] {
    [
        (byte & 0b10000000) >> 7 == 1,
        (byte & 0b01000000) >> 6 == 1,
        (byte & 0b00100000) >> 5 == 1,
        (byte & 0b00010000) >> 4 == 1,
        (byte & 0b00001000) >> 3 == 1,
        (byte & 0b00000100) >> 2 == 1,
        (byte & 0b00000010) >> 1 == 1,
        (byte & 0b00000001) == 1,
    ]
}

fn merge_bytes(b1: u8, b2: u8) -> u16 {
    (b1 as u16) << 8 | b2 as u16
}

#[allow(dead_code)]
fn merge_hex2(h1: u8, h2: u8) -> u8 {
    h1 << 4 | h2
}

#[allow(dead_code)]
fn merge_hex3(h1: u8, h2: u8, h3: u8) -> u16 {
    ((h1 as u16) << 8) | ((h2 as u16) << 4) | h3 as u16
}

#[allow(dead_code)]
fn bitmask1(instruction: u16) -> u16 {
    return instruction & 0xF000;
}

#[allow(dead_code)]
fn bitmask2(instruction: u16) -> u16 {
    return instruction & 0x0F00;
}

#[allow(dead_code)]
fn bitmask3(instruction: u16) -> u16 {
    return instruction & 0x00F0;
}

#[allow(dead_code)]
fn bitmask4(instruction: u16) -> u16 {
    return instruction & 0x000F;
}

fn s_bitmask1(instruction: u16) -> u8 {
    return ((instruction & 0xF000) >> 12) as u8;
}

fn s_bitmask2(instruction: u16) -> u8 {
    return ((instruction & 0x0F00) >> 8) as u8;
}

fn s_bitmask3(instruction: u16) -> u8 {
    return ((instruction & 0x00F0) >> 4) as u8;
}

fn s_bitmask4(instruction: u16) -> u8 {
    return (instruction & 0x000F) as u8;
}

fn s_bitmask24(instruction: u16) -> u16 {
    return instruction & 0x0FFF;
}

fn s_bitmask34(instruction: u16) -> u8 {
    return (instruction & 0x00FF) as u8;
}

fn most_significant_bit(byte: u8) -> u8 {
    return (byte & 0b10000000) >> 7;
}

fn least_significant_bit(byte: u8) -> u8 {
    return byte & 0b00000001;
}

pub struct Screen {
    pub pixels: [[bool; 32]; 64],
}

impl Screen {
    fn new() -> Screen {
        Screen {
            pixels: [[false; 32]; 64],
        }
    }

    fn clear(&mut self) {
        self.pixels = [[false; 32]; 64];
    }

    fn xor(&mut self, x: usize, y: usize, b: bool) -> bool {
        let pixel_value = self.pixels[x][y];
        self.pixels[x][y] = pixel_value ^ b;

        return pixel_value && b;
    }

    pub fn draw(&mut self, x: usize, y: usize, sprite: Vec<u8>) -> bool {
        let mut row_counter: usize = 0;
        let mut col_counter: usize = 0;
        let mut exact_x: usize;
        let mut exact_y: usize;

        let mut collision = false;

        for byte in sprite {
            let bools = get_bools_of_byte(byte);
            for pixel in bools {
                exact_x = (x + col_counter) % 64;
                exact_y = (y + row_counter) % 32;

                collision |= self.xor(exact_x, exact_y, pixel);

                col_counter += 1;
            }

            col_counter = 0;
            row_counter += 1;
        }

        return collision;
    }
}

pub struct VM {
    pub memory: [u8; 4096],
    pub registers: [u8; 16],
    pub pc: u16,
    pub i: u16,
    pub stack: Vec<u16>,
    pub delay_timer: u8,
    pub sound_timer: u8,
    timer_delay: u32,
    timer_counter: u32,
    pub screen: Screen,
    pub will_draw: bool,
    pub keys_pressed: Vec<u8>,
    pub custom_info: Vec<String>
}

impl VM {
    pub fn new_with_freq(freq: u32) -> VM {
        VM {
            memory: [0; 4096],
            registers: [0; 16],
            pc: 512,
            i: 0,
            stack: Vec::new(),
            delay_timer: 0,
            sound_timer: 0,
            screen: Screen::new(),
            will_draw: true,
            keys_pressed: Vec::new(),
            custom_info: Vec::new(),
            timer_delay: freq / 60,
            timer_counter: 0,
        }
    }

    pub fn new() -> VM {
        VM::new_with_freq(500)
    }

    #[allow(dead_code)]
    pub fn set_frequency(&mut self, freq: u32) {
        self.timer_delay = freq / 60;
    }

    pub fn load_rom(&mut self, rom: [u8; 4096]) {
        self.memory = rom;
    }

    #[allow(dead_code)]
    pub fn dump_memory(&self) {
        for (i, byte) in self.memory.iter().enumerate() {
            println!("{} -> {:#04X}", i, byte);
        }
    }

    pub fn get_instruction(&self) -> u16 {
        return merge_bytes(
            self.memory[self.pc as usize],
            self.memory[self.pc as usize + 1],
        );
    }

    fn push_to_stack(&mut self, value: u16) {
        self.stack.push(value);
    }

    fn pop_from_stack(&mut self) -> u16 {
        return self.stack.pop().unwrap_or(0);
    }

    pub fn init_font(&mut self) {
        for i in 0x50..0xA0 {
            self.memory[i] = FONT[i - 80];
        }
    }

    #[allow(dead_code)]
    fn get_font_sprite_of_byte(&self, byte: u8) -> Vec<u8> {
        let mut sprite: Vec<u8> = Vec::new();

        for pixels in &self.memory[(80 + byte * 5) as usize..(85 + byte * 5) as usize] {
            sprite.push(pixels.clone());
        }

        return sprite;
    }

    fn execute_instruction(&mut self, instruction: u16) -> u16 {
        match s_bitmask1(instruction) {
            0x0 => match instruction {
                0x00E0 => {
                    self.screen.clear();
                    return self.pc + 2;
                }
                0x00EE => self.pop_from_stack() + 2,
                _ => self.pc + 2,
            },
            0x1 => s_bitmask24(instruction),
            0x2 => {
                self.push_to_stack(self.pc);
                return s_bitmask24(instruction);
            }
            0x3 => {
                if self.registers[s_bitmask2(instruction) as usize] == s_bitmask34(instruction) {
                    self.pc + 4
                } else {
                    self.pc + 2
                }
            }
            0x4 => {
                if self.registers[s_bitmask2(instruction) as usize] == s_bitmask34(instruction) {
                    self.pc + 2
                } else {
                    self.pc + 4
                }
            }
            0x5 => match bitmask4(instruction) {
                0x0 => {
                    if self.registers[s_bitmask2(instruction) as usize]
                        == self.registers[s_bitmask3(instruction) as usize]
                    {
                        self.pc + 4
                    } else {
                        self.pc + 2
                    }
                }
                _ => self.pc + 2,
            },
            0x6 => {
                self.registers[s_bitmask2(instruction) as usize] = s_bitmask34(instruction);
                return self.pc + 2;
            }
            0x7 => {
                self.registers[s_bitmask2(instruction) as usize] = self.registers
                    [s_bitmask2(instruction) as usize]
                    .overflowing_add(s_bitmask34(instruction))
                    .0;
                return self.pc + 2;
            }
            0x8 => match s_bitmask4(instruction) {
                0x0 => {
                    self.registers[s_bitmask2(instruction) as usize] =
                        self.registers[s_bitmask3(instruction) as usize];
                    return self.pc + 2;
                }
                0x1 => {
                    self.registers[s_bitmask2(instruction) as usize] |=
                        self.registers[s_bitmask3(instruction) as usize];
                    return self.pc + 2;
                }
                0x2 => {
                    self.registers[s_bitmask2(instruction) as usize] &=
                        self.registers[s_bitmask3(instruction) as usize];
                    return self.pc + 2;
                }
                0x3 => {
                    self.registers[s_bitmask2(instruction) as usize] ^=
                        self.registers[s_bitmask3(instruction) as usize];
                    return self.pc + 2;
                }
                0x4 => {
                    let bitmask = s_bitmask2(instruction);
                    let (result, overflow) = self.registers[bitmask as usize]
                        .overflowing_add(self.registers[s_bitmask3(instruction) as usize]);
                    self.registers[0xF] = if overflow { 1 } else { 0 };
                    self.registers[bitmask as usize] = result;
                    return self.pc + 2;
                }
                0x5 => {
                    let bitmask = s_bitmask2(instruction);
                    let (result, overflow) = self.registers[bitmask as usize]
                        .overflowing_sub(self.registers[s_bitmask3(instruction) as usize]);
                    self.registers[0xF] = if overflow { 0 } else { 1 };
                    self.registers[bitmask as usize] = result;
                    return self.pc + 2;
                }
                0x6 => {
                    let bitmask = s_bitmask2(instruction);
                    self.registers[0xF] = least_significant_bit(self.registers[bitmask as usize]);
                    self.registers[bitmask as usize] >>= 1;
                    return self.pc + 2;
                }
                0x7 => {
                    let bitmask = s_bitmask2(instruction);
                    let (result, overflow) = self.registers[s_bitmask3(instruction) as usize]
                        .overflowing_sub(self.registers[bitmask as usize]);
                    self.registers[0xF] = if overflow { 0 } else { 1 };
                    self.registers[bitmask as usize] = result;
                    return self.pc + 2;
                }
                0xE => {
                    let bitmask = s_bitmask2(instruction);
                    self.registers[0xF] = most_significant_bit(self.registers[bitmask as usize]);
                    self.registers[bitmask as usize] <<= 1;
                    return self.pc + 2;
                }
                _ => self.pc + 2,
            },
            0x9 => match s_bitmask4(instruction) {
                0x0 => {
                    if self.registers[s_bitmask2(instruction) as usize]
                        == self.registers[s_bitmask3(instruction) as usize]
                    {
                        self.pc + 2
                    } else {
                        self.pc + 4
                    }
                }
                _ => self.pc + 2,
            },
            0xA => {
                self.i = s_bitmask24(instruction);
                return self.pc + 2;
            }
            0xB => s_bitmask24(instruction) + self.registers[0x0] as u16,
            0xC => {
                self.registers[s_bitmask2(instruction) as usize] =
                    fastrand::u8(..) & s_bitmask34(instruction);
                return self.pc + 2;
            }
            0xD => {
                let mut sprite: Vec<u8> = Vec::new();

                for i in 0..s_bitmask4(instruction) {
                    sprite.push(self.memory[(self.i + (i as u16)) as usize]);
                }

                self.registers[0xF] = if self.screen.draw(
                    self.registers[s_bitmask2(instruction) as usize] as usize,
                    self.registers[s_bitmask3(instruction) as usize] as usize,
                    sprite,
                ) {
                    1
                } else {
                    0
                };

                self.will_draw = true;

                return self.pc + 2;
            }
            0xE => match s_bitmask34(instruction) {
                0x9E => {
                    if self.keys_pressed.contains(&s_bitmask2(instruction)) {
                        self.pc + 4
                    } else {
                        self.pc + 2
                    }
                }
                0xA1 => {
                    if self.keys_pressed.contains(&s_bitmask2(instruction)) {
                        self.pc + 2
                    } else {
                        self.pc + 4
                    }
                }
                _ => self.pc + 2,
            },
            0xF => match s_bitmask34(instruction) {
                0x07 => {
                    self.registers[s_bitmask2(instruction) as usize] = self.delay_timer;
                    return self.pc + 2;
                }
                0x0A => {
                    if self.keys_pressed.is_empty() {
                        return self.pc;
                    } else {
                        self.registers[s_bitmask2(instruction) as usize] =
                            self.keys_pressed.pop().unwrap();
                        return self.pc + 2;
                    }
                }
                0x15 => {
                    self.delay_timer = self.registers[s_bitmask2(instruction) as usize];
                    return self.pc + 2;
                }
                0x18 => {
                    self.sound_timer = self.registers[s_bitmask2(instruction) as usize];
                    return self.pc + 2;
                }
                0x1E => {
                    self.i = self
                        .i
                        .overflowing_add(self.registers[s_bitmask2(instruction) as usize] as u16)
                        .0;
                    return self.pc + 2;
                }
                0x29 => {
                    self.i = 80 + 5 * (self.registers[s_bitmask2(instruction) as usize] as u16);
                    return self.pc + 2;
                }
                0x33 => {
                    let value = self.registers[s_bitmask2(instruction) as usize];
                    self.memory[self.i as usize] = value / 100;
                    self.memory[self.i as usize + 1] = value / 10 % 10;
                    self.memory[self.i as usize + 2] = value % 10;
                    return self.pc + 2;
                }
                0x55 => {
                    for i in 0..s_bitmask2(instruction) + 1 {
                        self.memory[(self.i + i as u16) as usize] = self.registers[i as usize];
                    }
                    return self.pc + 2;
                }
                0x65 => {
                    for i in 0..s_bitmask2(instruction) + 1 {
                        self.registers[i as usize] = self.memory[(self.i + i as u16) as usize];
                    }
                    return self.pc + 2;
                }
                _ => self.pc + 2,
            },
            _ => self.pc + 2,
        }
    }

    #[allow(dead_code)]
    fn test_keys_and_screen(&mut self) {
        for key in self.keys_pressed.iter() {
            let mut sprite: Vec<u8> = Vec::new();

            for i in 80 + key * 5..85 + key * 5 {
                sprite.push(self.memory[i as usize]);
            }

            self.screen.draw(1, 1, sprite);
        }
    }

    pub fn next(&mut self) -> u8 {
        if self.timer_counter == self.timer_delay {
            self.delay_timer = self.delay_timer.saturating_sub(1);
            self.sound_timer = self.sound_timer.saturating_sub(1);

            self.timer_counter = 0;
        }

        self.timer_counter += 1;

        let instruction = self.get_instruction();
        let new_pc = self.execute_instruction(instruction);

        if new_pc > 2047 {
            return 0;
        }

        self.pc = new_pc;

        return 1;
    }
}
