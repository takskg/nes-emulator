use std::fs::File;
use std::io::prelude::*;

#[macro_use]
extern crate log;

mod nes;
use nes::cassette::Cassette;
use nes::nes_machine::NesMachine;

fn logger_initialize() {
    //とりあえず全部出す
    std::env::set_var("RUST_LOG", "trace");
    env_logger::init();
}

fn main() {
    //ロガーの設定
    logger_initialize();

    //NESファイルからROMデータを読み込み
    let mut file = File::open("rom/hello-world.nes").unwrap();
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let mut cst: Cassette = Default::default();
    if cst.load_from_buffer(buffer) {
        info!("NESファイルの読み込みに成功！！");
    } else {
        info!("NESファイルの読み込みに失敗しました");
    }

    let mut machine: NesMachine = NesMachine::new();
    machine.insert_cassette(cst);
    if machine.boot() {
        loop {
            if machine.update() {
                //ゲーム実行中...
            } else {
                //マシン終了
                info!("ゲームを終了させます");
                break;
            }
        }
    } else {
        error!(
            "NESマシンの起動に失敗しました。.nesファイルが正しく読み込めてない可能性が高いです。"
        );
    }
}
