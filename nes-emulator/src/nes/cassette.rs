use super::ines;


const INES_SIZE: usize = 16;//カセットのヘッダーサイズ(byte)
const PROM_MAX_SIZE: usize = 128;//プログラムROMの最大サイズ(byte)
const CROM_MAX_SIZE: usize = 128;//キャラクタROMの最大サイズ(byte)

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
        if self.ines.load_ines(buffer) {
            //PROM/CROMの読み込み

            result = true;
        }else{
            //ヘッダーが不正
        }

        return result;
    }
}