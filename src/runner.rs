use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::iter;
use std::time::Duration;

use macroquad::prelude::*;

use crate::chip8::*;
use crate::reader::*;

static KEYMAP: &'static [(KeyCode, u8)] = &[
    (KeyCode::A, 0x0),
    (KeyCode::Z, 0x1),
    (KeyCode::E, 0x2),
    (KeyCode::R, 0x3),
    (KeyCode::T, 0x4),
    (KeyCode::Y, 0x5),
    (KeyCode::U, 0x6),
    (KeyCode::I, 0x7),
    (KeyCode::O, 0x8),
    (KeyCode::P, 0x9),
    (KeyCode::Q, 0xA),
    (KeyCode::S, 0xB),
    (KeyCode::D, 0xC),
    (KeyCode::F, 0xD),
    (KeyCode::G, 0xE),
    (KeyCode::H, 0xF),
];

// Launch a thread responsible for the VM backend
fn launch_vm_thread(vm_shared: Arc<Mutex<VM>>, target_freq: u32){
    thread::spawn(move || {
        // Ticker with infinite iterator
        for _ in ticker::Ticker::new(iter::repeat(()), Duration::from_nanos(1_000_000_000 / (target_freq as u64))){
            if vm_shared.lock().unwrap().next() == 0 {
                break;
            };
        }
    });
}

pub async fn create_vm_and_start(rom_path: String, target_freq: u32) {
    let vm_shared = Arc::new(Mutex::new(VM::new_with_freq(target_freq)));

    // Specific scope so that the mutex is unlocked after vm init
    {
        let mut vm = vm_shared.lock().unwrap();
        vm.load_rom(read_rom(rom_path));
        vm.init_font();
    }

    // Launch the VM backend
    launch_vm_thread(Arc::clone(&vm_shared), target_freq);

    // Launch VM frontend
    launch_vm_frontend(vm_shared).await;
}

async fn launch_vm_frontend(vm_shared: Arc<Mutex<VM>>) {
    loop {
        clear_background(BLACK);

        if is_key_down(KeyCode::Escape){
            break;
        }

        {
            let mut vm = vm_shared.lock().unwrap();

            vm.keys_pressed.clear();

            for (key, byte) in KEYMAP.iter() {
                if is_key_down(*key) {
                    vm.keys_pressed.push(*byte);
                }
            }

        }

        {
            let vm = vm_shared.lock().unwrap();

            for (x, col) in vm.screen.pixels.iter().enumerate() {
                for (y, pixel) in col.iter().enumerate() {
                    if *pixel {
                        draw_rectangle((x * 8) as f32, (y * 8) as f32, 8f32, 8f32, WHITE);
                    }
                }
            }
        }

        next_frame().await;
    }
}
