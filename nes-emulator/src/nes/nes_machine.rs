use super::cassette;
use super::cpu;

/*
NES実機の表現用
いわゆるManager的な物

1. カセット挿入
2. 電源ON
3. カセットのプログラム実行
4. 電源OFF
5. 終了
*/
pub struct NesMachine {
    cassette: cassette::Cassette, //実行するカセット
    cpu: cpu::Cpu,                //CPU
}

impl NesMachine {
    pub fn new() -> NesMachine {
        NesMachine {
            cassette: Default::default(),
            cpu: cpu::Cpu::new(),
        }
    }

    /// 有効なカセットを挿入します。
    /// 必ずBoot前に実行してください。
    pub fn insert_cassette(&mut self, insert_cassette: cassette::Cassette) {
        self.cassette = insert_cassette;
    }

    /// ブートします
    /// 必ずカセット挿入後に呼び出してください。
    /// ブートに成功したら、Updateを呼び続けてください。
    pub fn boot(&self) -> bool {
        return true;
    }
    /// ゲームの更新を行う
    /// Bootを呼び出した後に呼び続けてください
    /// falseが返ってくるとシャットダウン処理が行われたので、
    /// 速やかにUpdateを呼ぶのを停止してください。
    pub fn update(&self) -> bool {
        //今のところ出来ることがないので即終了
        return false;
    }
}
