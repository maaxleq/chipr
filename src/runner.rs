use crate::chip8::VM;
use crate::reader::*;
use console_engine::*;
use std::time::Instant;

static PIXEL_1: pixel::Pixel = pixel::Pixel {
    bg: Color::White,
    fg: Color::DarkGrey,
    chr: ' ',
};

static PIXEL_0: pixel::Pixel = pixel::Pixel {
    bg: Color::DarkGrey,
    fg: Color::White,
    chr: ' ',
};

static DEBUG_BG: Color = Color::DarkRed;

static DEBUG_FG: Color = Color::White;

static DEBUG_BG_PIXEL: pixel::Pixel = pixel::Pixel {
    bg: DEBUG_BG,
    fg: DEBUG_FG,
    chr: ' ',
};

static KEYBINDS: &'static [(char, u8)] = &[
    ('à', 0x0),
    ('&', 0x1),
    ('é', 0x2),
    ('"', 0x3),
    ('\'', 0x4),
    ('(', 0x5),
    ('-', 0x6),
    ('è', 0x7),
    ('_', 0x8),
    ('ç', 0x9),
    ('a', 0xA),
    ('b', 0xB),
    ('c', 0xC),
    ('d', 0xD),
    ('e', 0xE),
    ('f', 0xF),
];

pub struct DebugInfo {
    ips: u128,
    pc: u16,
    i: u16,
    stack_size: usize,
    delay_timer: u8,
    sound_timer: u8,
    registers: [u8; 16],
    current_instruction: u16,
    additional_info: Vec<String>,
    keys_pressed: Vec<u8>
}

impl DebugInfo {
    fn new() -> DebugInfo {
        DebugInfo {
            ips: 0,
            pc: 0,
            i: 0,
            stack_size: 0,
            delay_timer: 0,
            sound_timer: 0,
            registers: [0; 16],
            current_instruction: 0,
            additional_info: Vec::new(),
            keys_pressed: Vec::new()
        }
    }

    fn get_keys_pressed_bin(&self) -> [u8; 16] {
        let mut bools = [0; 16];

        for i in 0..16 {
            if self.keys_pressed.contains(&i) {
                bools[i as usize] = 1;
            }
        }

        return bools;
    }
}

pub struct Runner {
    vm: VM,
    ui_engine: ConsoleEngine,
    main_screen: screen::Screen,
    target_rate: u32,
    ips_timer: Instant,
    debug: bool,
    debug_screen: Option<screen::Screen>,
    debug_info: Option<DebugInfo>,
}

impl Runner {
    pub fn new(rom_path: String, rate: u32, debug: bool) -> Runner {
        let mut vm = VM::new_with_freq(rate);
        vm.load_rom(read_rom(rom_path));
        vm.init_font();

        Runner {
            vm: vm,
            main_screen: screen::Screen::new(64, 32),
            ui_engine: ConsoleEngine::init(if debug { 80 } else { 64 }, if debug { 40 } else { 32 }, rate).unwrap(),
            ips_timer: Instant::now(),
            debug: debug,
            target_rate: rate,
            debug_screen: if debug {
                Some(screen::Screen::new(16, 40))
            } else {
                None
            },
            debug_info: if debug { Some(DebugInfo::new()) } else { None },
        }
    }

    fn print(&mut self) {
        self.main_screen.clear();

        for (x, col) in self.vm.screen.pixels.iter().enumerate() {
            for (y, pixel) in col.iter().enumerate() {
                self.main_screen.set_pxl(
                    x as i32,
                    y as i32,
                    if pixel.clone() { PIXEL_1 } else { PIXEL_0 },
                );
            }
        }
    }

    fn print_debug(&mut self) {
        let screen = self.debug_screen.as_mut().unwrap();
        let info = self.debug_info.as_mut().unwrap();

        screen.clear();

        for x in 0..16 {
            for y in 0..40 {
                screen.set_pxl(x, y, DEBUG_BG_PIXEL);
            }
        }

        screen.print_fbg(0, 0, "instr / sec", DEBUG_FG, DEBUG_BG);
        screen.print_fbg(0, 1, format!("{} / {}", info.ips, self.target_rate).as_str(), DEBUG_FG, DEBUG_BG);

        screen.print_fbg(0, 3, "pc", DEBUG_FG, DEBUG_BG);
        screen.print_fbg(0, 4, format!("{}", info.pc).as_str(), DEBUG_FG, DEBUG_BG);

        screen.print_fbg(0, 6, "i", DEBUG_FG, DEBUG_BG);
        screen.print_fbg(0, 7, format!("{}", info.i).as_str(), DEBUG_FG, DEBUG_BG);

        screen.print_fbg(0, 9, "stack size", DEBUG_FG, DEBUG_BG);
        screen.print_fbg(0, 10, format!("{}", info.stack_size).as_str(), DEBUG_FG, DEBUG_BG);

        screen.print_fbg(0, 12, "delay timer", DEBUG_FG, DEBUG_BG);
        screen.print_fbg(0, 13, format!("{}", info.delay_timer).as_str(), DEBUG_FG, DEBUG_BG);

        screen.print_fbg(0, 15, "sound timer", DEBUG_FG, DEBUG_BG);
        screen.print_fbg(0, 16, format!("{}", info.sound_timer).as_str(), DEBUG_FG, DEBUG_BG);

        screen.print_fbg(0, 18, "instr", DEBUG_FG, DEBUG_BG);
        screen.print_fbg(0, 19, format!("{:04X?}", info.current_instruction).as_str(), DEBUG_FG, DEBUG_BG);

        screen.print_fbg(0, 21, "registers", DEBUG_FG, DEBUG_BG);

        for i in (0..16).step_by(4) {
            screen.print_fbg(0, i / 4 + 22, format!("{:02X?}", info.registers[i as usize]).as_str(), DEBUG_FG, DEBUG_BG);
            screen.print_fbg(3, i / 4 + 22, format!("{:02X?}", info.registers[i as usize + 1]).as_str(), DEBUG_FG, DEBUG_BG);
            screen.print_fbg(6, i / 4 + 22, format!("{:02X?}", info.registers[i as usize + 2]).as_str(), DEBUG_FG, DEBUG_BG);
            screen.print_fbg(9, i / 4 + 22, format!("{:02X?}", info.registers[i as usize + 3]).as_str(), DEBUG_FG, DEBUG_BG);
        }

        let keys_pressed_bools = info.get_keys_pressed_bin();
        screen.print_fbg(0, 27, "keys", DEBUG_FG, DEBUG_BG);

        for i in (0..16).step_by(4) {
            screen.print_fbg(0, i / 4 + 28, format!("{:02X?}", keys_pressed_bools[i as usize]).as_str(), DEBUG_FG, DEBUG_BG);
            screen.print_fbg(3, i / 4 + 28, format!("{:02X?}", keys_pressed_bools[i as usize + 1]).as_str(), DEBUG_FG, DEBUG_BG);
            screen.print_fbg(6, i / 4 + 28, format!("{:02X?}", keys_pressed_bools[i as usize + 2]).as_str(), DEBUG_FG, DEBUG_BG);
            screen.print_fbg(9, i / 4 + 28, format!("{:02X?}", keys_pressed_bools[i as usize + 3]).as_str(), DEBUG_FG, DEBUG_BG);
        }

        for (i, line) in info.additional_info.iter().enumerate() {
            screen.print_fbg(0, 27 + (i as i32), line.as_str(), DEBUG_FG, DEBUG_BG);
        }
    }

    fn update_debug_info(&mut self, ips: u128) {
        let info = self.debug_info.as_mut().unwrap();

        info.ips = ips;
        info.pc = self.vm.pc;
        info.i = self.vm.i;
        info.stack_size = self.vm.stack.len();
        info.delay_timer = self.vm.delay_timer.time;
        info.sound_timer = self.vm.sound_timer.time;
        info.registers = self.vm.registers.clone();
        info.current_instruction = self.vm.get_instruction();
        info.additional_info = self.vm.custom_info.clone();
        info.keys_pressed = self.vm.keys_pressed.clone();
    }

    pub fn run(&mut self) {
        loop {
            self.ui_engine.wait_frame();

            for (chr, byte) in KEYBINDS.iter() {
                if self.ui_engine.is_key_pressed(KeyCode::Char(chr.clone())) {
                    self.vm.keys_pressed.push(byte.clone());
                }
            }

            if self.ui_engine.is_key_pressed(KeyCode::Esc) {
                break;
            }

            self.ui_engine.clear_screen();

            if self.vm.will_draw {
                self.print();
                self.vm.will_draw = false;
            }

            if self.debug {
                self.update_debug_info(1000000000 / self.ips_timer.elapsed().as_nanos());
                self.ips_timer = Instant::now();
                self.print_debug();
                self.ui_engine
                    .print_screen(64, 0, &self.debug_screen.as_ref().unwrap());
            }

            self.ui_engine.print_screen(0, 0, &self.main_screen);
            self.ui_engine.draw();

            if self.vm.next() == 0 {
                break;
            };
        }
    }
}
