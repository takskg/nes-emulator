pub struct Rom {
    rom: Vec<u8>,
}

impl Default for Rom {
    fn default() -> Self {
        Self {
            rom: Default::default(),
        }
    }
}

impl Rom {
    pub fn new(size: usize, data: &[u8]) -> Rom {
        let mut rom = Rom {
            rom: Vec::with_capacity(size),
        };
        rom.rom.resize(size, 0);
        rom.rom.copy_from_slice(&data);
        return rom;
    }

    /// 指定アドレスから1byte読み込む
    pub fn read_u8(self, addr: u16) -> u8 {
        return self.rom[addr as usize];
    }

    /// ROMのサイズを取得（byte）
    pub fn get_rom_size(self) -> usize {
        return self.rom.len();
    }
}
