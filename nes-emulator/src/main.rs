use std::fs::File;
use std::io::prelude::*;

mod nes;

fn main() {
    //NESファイルからROMデータを読み込み
    let mut file = File::open("rom/hello-world.nes").unwrap();
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let mut cst: nes::cassette::Cassette = Default::default();
    if cst.load_from_buffer(buffer) {
        println!("NESファイルの読み込みに成功！！");
    }else{
        println!("NESファイルの読み込みに失敗しました");
    }
}
