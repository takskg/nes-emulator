struct Status {
    pub negative: bool,
    pub overflow: bool,
    pub reserved: bool,
    pub break_mode: bool,
    pub decimal: bool,
    pub interrupt: bool,
    pub zero: bool,
    pub carry: bool,
}
impl Status {
    fn new() -> Status {
        Status {
            negative: false,
            overflow: false,
            reserved: true,
            break_mode: true,
            decimal: false,
            interrupt: true,
            zero: false,
            carry: false,
        }
    }
}
//レジスタ
struct Registor {
    pub a: u8,     //アキュムレータ
    pub x: u8,     //インデックスレジスタ
    pub y: u8,     //インデックスレジスタ
    pub s: u8,     //スタックポインタ
    pub p: Status, //ステータスレジスタ
    pub pc: u16,   //プログラムカウンタ
}
impl Default for Registor {
    fn default() -> Self {
        Self {
            a: 0x0,
            x: 0x0,
            y: 0x0,
            s: 0x0,
            p: Status::new(),
            pc: 0x0,
        }
    }
}

//CPU
pub struct Cpu {
    register: Registor,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            register: Default::default(),
        }
    }

    /// リセット
    pub fn reset(&mut self) {
        //今のところすることなし
    }
}
