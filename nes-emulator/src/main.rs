mod nes;
use log::*;
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
    let rom_file_path = "rom/hello-world.nes".to_string();
    let mut cst: Cassette = Default::default();
    if cst.load_from_file(rom_file_path) {
        info!("NESファイルの読み込みに成功！！");
    } else {
        info!("NESファイルの読み込みに失敗しました");
    }

    let mut machine: NesMachine = NesMachine::new(&cst);
    machine.reset();
    if machine.boot() {
        debug!("ゲーム実行開始");
        machine.run();
        debug!("ゲーム終了");
    } else {
        error!(
            "NESマシンの起動に失敗しました。.nesファイルが正しく読み込めてない可能性が高いです。"
        );
    }
}
