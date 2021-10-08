mod chip8;
mod reader;
mod runner;

use runner::*;
use chip8::VM;
use reader::*;
use std::env;

static ARGUMENT_PARSE_ERROR: &str = "Could not parse argument";
static NO_INPUT_FILE_ERROR: &str = "No input file provided";

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut filename = String::new();
    let mut rate: u32 = 450;
    let mut debug = false;
    let mut dump = false;

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
        else if arg.eq("--debug"){
            debug = true;
        }
        else if arg.eq("--dump"){
            dump = true;
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
    else {
        let mut runner = Runner::new(filename, rate, debug);
        runner.run();
    }
}
