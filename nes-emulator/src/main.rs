use std::fs::File;
use std::io::prelude::*;

mod nes;


fn main() {
    //NESファイルからROMデータを読み込み
    let mut file = File::open("rom/hello-world.nes").unwrap();
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let mut nes_rom: nes::Nes = Default::default();
    nes_rom.load(buffer);
}
