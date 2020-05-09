use super::ines;

const INES_HEADER_SIZE: usize = 16; //カセットのヘッダーサイズ(byte)
const PROM_MAX_SIZE: usize = 32768; //プログラムROMの最大サイズ(32KiB)
const CROM_MAX_SIZE: usize = 8192; //キャラクタROMの最大サイズ(8KiB)
const PROM_BLOCK_SIZE: usize = 16384; //プログラムROMのブロックサイズ(byte)
const CROM_BLOCK_SIZE: usize = 8192; //キャラクタROMのブロックサイズ(byte)

/*
カセットの内容
*/
pub struct Cassette {
    pub ines: ines::Ines,
    pub prom: [u8; PROM_MAX_SIZE],
    pub crom: [u8; CROM_MAX_SIZE],
}

impl Default for Cassette {
    fn default() -> Self {
        Self {
            ines: Default::default(),
            prom: [0; PROM_MAX_SIZE],
            crom: [0; CROM_MAX_SIZE],
        }
    }
}

impl Cassette {
    pub fn load_from_buffer(&mut self, buffer: Vec<u8>) -> bool {
        let mut result = false;

        //ヘッダーの読み込み
        if self.ines.load_ines(&buffer) {
            //PROM/CROMの読み込み
            let prom_byte = self.ines.prom_size as usize * PROM_BLOCK_SIZE;
            let crom_byte = self.ines.crom_size as usize * CROM_BLOCK_SIZE;
            let prom_start = INES_HEADER_SIZE;
            let prom_end = prom_start + prom_byte - 1;
            let crom_start = prom_end;
            let crom_end = crom_start + crom_byte - 1;
            //PROM
            self.prom[..prom_byte].copy_from_slice(&buffer[prom_start..prom_end]);
            //CROM
            self.crom[..crom_byte].copy_from_slice(&buffer[crom_start..crom_end]);

            result = true;
        } else {
            //ヘッダーが不正
        }

        return result;
    }
}
