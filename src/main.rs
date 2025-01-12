mod chip8;
mod reader;
mod runner;
mod bench;

use macroquad::{prelude::Conf, miniquad::conf::Platform};
use runner::*;
use chip8::VM;
use reader::*;
use std::env;

static ARGUMENT_PARSE_ERROR: &str = "Could not parse argument";
static NO_INPUT_FILE_ERROR: &str = "No input file provided";

fn create_conf() -> Conf {
    Conf {
        window_title: String::from("Chipr"),
        window_resizable: false,
        window_height: 256,
        window_width: 512,
        high_dpi: false,
        fullscreen: false,
        sample_count: 1,
        icon: None,
        platform: Platform::default()
    }
}

#[macroquad::main(create_conf)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let mut filename = String::new();
    let mut rate: u32 = 450;
    let mut dump = false;
    let mut benchmark = false;

    for arg in args.iter().skip(1) {
        if arg.starts_with("rom=") {
            filename = arg[4..arg.len()].to_string();
        }
        else if arg.starts_with("rate="){
            let n = arg[5..arg.len()]
            .parse::<usize>()
            .expect(&format!("{} {}", ARGUMENT_PARSE_ERROR, arg));
            rate = n as u32;
        }
        else if arg.eq("--dump"){
            dump = true;
        }
        else if arg.eq("--benchmark"){
            benchmark = true;
        }
        else {
            panic!("{} {}", ARGUMENT_PARSE_ERROR, arg);
        }
    }

    if filename == "" {
        panic!("{}", NO_INPUT_FILE_ERROR);
    }

    if dump {
        let mut vm = VM::new();
        vm.load_rom(read_rom(filename));
        vm.init_font();

        vm.dump_memory();
    }
    else if benchmark {
        let mut vm = VM::new_with_freq(100_000_000);
        vm.load_rom(read_rom(filename));
        vm.init_font();

        let mut bench = bench::Bench::new(vm);
        bench.test();
        bench.print_results();
    }
    else {
        create_vm_and_start(filename, rate).await;
    }

    Ok(())
}
