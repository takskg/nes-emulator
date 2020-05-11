use super::cassette::Cassette;
use super::cpu::Cpu;
use super::ram::Ram;

const WRAM_SIZE: usize = 1024 * 2; //WRAMのサイズ(2KiB)

/*
NES実機の表現用
いわゆるManager的な物

1. カセット挿入
2. 電源ON
3. カセットのプログラム実行
4. 電源OFF
5. 終了
*/
pub struct NesMachine<'a> {
    cassette: &'a Cassette, //実行するカセット
    cpu: Cpu,               //CPU
    wram: Ram,              //WRAM
}

impl<'a> NesMachine<'a> {
    pub fn new(insert_cassette: &'a Cassette) -> NesMachine<'a> {
        NesMachine {
            cassette: insert_cassette,
            cpu: Cpu::new(),
            wram: Ram::new(WRAM_SIZE),
        }
    }

    /// ブートします
    /// 必ずカセット挿入後に呼び出してください。
    /// ブートに成功したら、Updateを呼び続けてください。
    pub fn boot(&mut self) -> bool {
        return true;
    }

    /// リセット
    /// リセットボタンが押された時の挙動をエミュレートします
    pub fn reset(&mut self) {
        self.cpu.reset();
    }

    /// ゲームの更新を行う
    /// Bootを呼び出した後に呼び出してください
    pub fn run(&mut self) {
        //今のところ出来ることがないので即終了
    }
}
