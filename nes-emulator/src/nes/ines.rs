
pub struct Ines {
    pub prom_size: u8,
    pub crom_size: u8,
    flag6: u8,
    flag7: u8,
    flag8: u8,
    flag9: u8,
    flag10: u8,
}

impl Default for Ines {
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

impl Ines {
    pub fn load_ines(&mut self, buffer: &Vec<u8>) -> bool {
        let mut result = false;

        if buffer[0] == 0x4E && buffer[1] == 0x45 && buffer[2] == 0x53 && buffer[3] == 0x1A {//NES
            self.prom_size = buffer[4];
            self.crom_size = buffer[5];
            self.flag6 = buffer[6];
            self.flag7 = buffer[7];
            self.flag8 = buffer[8];
            self.flag9 = buffer[9];
            self.flag10 = buffer[10];

            result = true;
        }
        
        return result;
    }
}