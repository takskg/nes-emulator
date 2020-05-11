pub struct Ram {
    ram: Vec<u8>,
}

impl Ram {
    pub fn new(size: usize) -> Ram {
        let mut ram = Ram {
            ram: Vec::with_capacity(size),
        };
        ram.ram.resize(size, 0);
        return ram;
    }

    /// 読み込み
    /// 指定アドレスを1byte読み込む
    pub fn read_u8(self, addr: u16) -> u8 {
        return self.ram[addr as usize];
    }

    /// 書き込み
    /// 指定アドレスに対して1byte書き込む
    /// 範囲チェックはしてない
    pub fn write_u8(mut self, addr: u16, data: u8) {
        self.ram[addr as usize] = data;
    }
}
