use super::ines;
use super::rom;
use std::fs::File;
use std::io::prelude::*;

const INES_HEADER_SIZE: usize = 16; //カセットのヘッダーサイズ(byte)
const PROM_BLOCK_SIZE: usize = 16384; //プログラムROMのブロックサイズ(byte)
const CROM_BLOCK_SIZE: usize = 8192; //キャラクタROMのブロックサイズ(byte)

/*
カセットの内容
*/
pub struct Cassette {
    pub ines: ines::Ines,
    pub prom: rom::Rom,
    pub crom: rom::Rom,
}

impl Default for Cassette {
    fn default() -> Self {
        Self {
            ines: Default::default(),
            prom: Default::default(),
            crom: Default::default(),
        }
    }
}

impl Cassette {
    /// ファイルからROMをロードする
    pub fn load_from_file(&mut self, file_path: String) -> bool {
        let mut file = File::open(file_path).unwrap();
        let mut buffer: Vec<u8> = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        return self.load_from_buffer(buffer);
    }
    /// バッファからROMをロードする
    fn load_from_buffer(&mut self, buffer: Vec<u8>) -> bool {
        let mut result = false;

        //ヘッダーの読み込み
        if self.ines.load_ines(&buffer) {
            //PROM/CROMの読み込み
            let prom_byte = self.ines.prom_size as usize * PROM_BLOCK_SIZE;
            let crom_byte = self.ines.crom_size as usize * CROM_BLOCK_SIZE;
            let prom_start = INES_HEADER_SIZE;
            let prom_end = prom_start + prom_byte;
            let crom_start = prom_end;
            let crom_end = crom_start + crom_byte;
            //PROM
            self.prom = rom::Rom::new(prom_byte, &buffer[prom_start..prom_end]);
            //CROM
            self.crom = rom::Rom::new(crom_byte, &buffer[crom_start..crom_end]);

            result = true;
        } else {
            //ヘッダーが不正
        }

        return result;
    }
}
