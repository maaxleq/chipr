use std::fs;
use std::io;
use std::io::Read;

static FILE_READ_ERROR: &str = "Could not read file";
static FILE_READING_ERROR: &str = "Error while reading file";

pub fn read_rom(filename: String) -> [u8; 4096] {
    let f = fs::File::open(filename).expect(FILE_READ_ERROR);
    let mut reader = io::BufReader::new(f);
    let mut rom_vec: Vec<u8> = vec![];
    let mut rom: [u8; 4096] = [0; 4096];

    reader.read_to_end(&mut rom_vec).expect(FILE_READING_ERROR);

    for (i, byte) in rom_vec.iter().enumerate() {
        rom[i + 512] = byte.clone();
    }

    return rom;
}
