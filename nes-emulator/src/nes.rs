
pub struct Header {
    prom_size: u8,
    crom_size: u8,
    flag6: u8,
    flag7: u8,
    flag8: u8,
    flag9: u8,
    flag10: u8,
}
impl Default for Header {
    fn default() -> Self {
        Self {
            prom_size: 0,
            crom_size: 0,
            flag6: 0,
            flag7: 0,
            flag8: 0,
            flag9: 0,
            flag10: 0,
        }
    }
}

pub struct Nes {
    pub header: Header,
}

impl Default for Nes {
    fn default() -> Self {
        Self {
            header: Default::default(),
        }
    }
}

impl Nes {
    pub fn load(&mut self, buffer: Vec<u8>) -> bool {
        let mut result = false;

        if buffer[0] == b'N' && buffer[1] == b'E' && buffer[2] == b'S' {
            self.header.prom_size = buffer[3];
            self.header.crom_size = buffer[4];
            self.header.flag6 = buffer[5];
            self.header.flag7 = buffer[6];
            self.header.flag8 = buffer[7];
            self.header.flag9 = buffer[8];
            self.header.flag10 = buffer[9];

            result = true;
        }
        
        return result;
    }
}