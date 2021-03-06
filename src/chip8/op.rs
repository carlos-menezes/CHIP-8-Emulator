use std::fmt;

pub enum Op {
    SYS  { addr: u16 },                         // 0nnn
    CLS,                                        // 00E0
    RET,                                        // 00EE
    JP   { addr: u16 },                         // 1nnn
    CALL { addr: u16 },                         // 2nnn
    SE   { reg: u8, byte: u8 },                 // 3xkk
    SNE  { reg: u8, byte: u8 },                 // 4xkk
    SER  { reg_a: u8, reg_b: u8 },              // 5xy0
    LD   { reg: u8, byte: u8 },                 // 6xkk
    ADD  { reg: u8, byte: u8 },                 // 7xkk
    LDR  { reg_a: u8, reg_b: u8 },              // 8xy0
    OR   { reg_a: u8, reg_b: u8 },              // 8xy1
    AND  { reg_a: u8, reg_b: u8 },              // 8xy2
    XOR  { reg_a: u8, reg_b: u8 },              // 8xy3
    ADDR { reg_a: u8, reg_b: u8 },              // 8xy4
    SUB  { reg_a: u8, reg_b: u8 },              // 8xy5
    SHR  { reg_a: u8, reg_b: u8 },              // 8xy6
    SUBN { reg_a: u8, reg_b: u8 },              // 8xy7
    SHL  { reg_a: u8, reg_b: u8 },              // 8xyE
    SNER { reg_a: u8, reg_b: u8 },              // 9xy0
    LDA  { addr: u16 },                         // Annn
    JPO  { addr: u16 },                         // Bnnn
    RND  { reg: u8, byte: u8 },                 // Cxkk
    DRW  { reg_a: u8, reg_b: u8, nibble: u8 },  // Dxyn
    SKP  { reg: u8 },                           // Ex9E
    SKNP { reg: u8 },                           // ExA1
    LDDT { reg: u8 },                           // Fx07
    LDKP { reg: u8 },                           // Fx0A
    STDT { reg: u8 },                           // Fx15
    STST { reg: u8 },                           // Fx18
    ADDA { reg: u8 },                           // Fx1E
    LDSA { reg: u8 },                           // Fx29
    STDR { reg: u8 },                           // Fx33
    STRR { reg: u8 },                           // Fx55
    LDRR { reg: u8 },                           // Fx66
    INV  { opcode: u16 },                       // invalid
}

impl Op {
    pub fn decode(opcode: u16) -> Self {
        match opcode & 0xF000 {
            0x0000 => {
                let addr = opcode & 0x0FFF;
                match addr {
                    0x00E0 => Self::CLS,
                    0x00EE => Self::RET,
                    _ => Self::SYS { addr }
                }
            }
            0x1000 => {
                let addr = opcode & 0x0FFF;
                Self::JP { addr }
            }
            0x2000 => {
                let addr = opcode & 0x0FFF;
                Self::CALL { addr }
            }
            0x3000 => {
                let reg = ((opcode & 0x0F00) >> 8) as u8;
                let byte = (opcode & 0x00FF) as u8;
                Self::SE { reg, byte }
            }
            0x4000 => {
                let reg = ((opcode & 0x0F00) >> 8) as u8;
                let byte = (opcode & 0x00FF) as u8;
                Self::SNE { reg, byte }
            }
            0x5000 => {
                match opcode & 0x000F {
                    0x0 => {
                        let reg_a = ((opcode & 0x0F00) >> 8) as u8;
                        let reg_b = ((opcode & 0x00F0) >> 4) as u8;
                        Self::SER { reg_a, reg_b }
                    }
                    _ => Self::INV { opcode }
                }
            }
            0x6000 => {
                let reg = ((opcode & 0x0F00) >> 8) as u8;
                let byte = (opcode & 0x00FF) as u8;
                Self::LD { reg, byte }
            }
            0x7000 => {
                let reg = ((opcode & 0x0F00) >> 8) as u8;
                let byte = (opcode & 0x00FF) as u8;
                Self::ADD { reg, byte }
            }
            0x8000 => {
                let reg_a = ((opcode & 0x0F00) >> 8) as u8;
                let reg_b = ((opcode & 0x00F0) >> 4) as u8;
                match opcode & 0x000F {
                    0x0 => Self::LDR  { reg_a, reg_b },
                    0x1 => Self::OR   { reg_a, reg_b },
                    0x2 => Self::AND  { reg_a, reg_b },
                    0x3 => Self::XOR  { reg_a, reg_b },
                    0x4 => Self::ADDR { reg_a, reg_b },
                    0x5 => Self::SUB  { reg_a, reg_b },
                    0x6 => Self::SHR  { reg_a, reg_b },
                    0x7 => Self::SUBN { reg_a, reg_b },
                    0xE => Self::SHL  { reg_a, reg_b },
                    _ => Self::INV { opcode }
                }
            }
            0x9000 => {
                match opcode & 0x000F {
                    0x0 => {
                        let reg_a = ((opcode & 0x0F00) >> 8) as u8;
                        let reg_b = ((opcode & 0x00F0) >> 4) as u8;
                        Self::SNER { reg_a, reg_b }
                    }
                    _ => Self::INV { opcode }
                }
            }
            0xA000 => {
                let addr = opcode & 0x0FFF;
                Self::LDA { addr }
            }
            0xB000 => {
                let addr = opcode & 0x0FFF;
                Self::JPO { addr }
            }
            0xC000 => {
                let reg = ((opcode & 0x0F00) >> 8) as u8;
                let byte = (opcode & 0x00FF) as u8;
                Self::RND { reg, byte }
            }
            0xD000 => {
                let reg_a = ((opcode & 0x0F00) >> 8) as u8;
                let reg_b = ((opcode & 0x00F0) >> 4) as u8;
                let nibble = (opcode & 0x000F) as u8;
                Self::DRW { reg_a, reg_b, nibble }
            }
            0xE000 => {
                let reg = ((opcode & 0x0F00) >> 8) as u8;
                match opcode & 0x00FF {
                    0x9E => Self::SKP  { reg },
                    0xA1 => Self::SKNP { reg },
                    _ => Self::INV { opcode }
                }
            }
            0xF000 => {
                let reg = ((opcode & 0x0F00) >> 8) as u8;
                match opcode & 0x00FF {
                    0x07 => Self::LDDT { reg },
                    0x0A => Self::LDKP { reg },
                    0x15 => Self::STDT { reg },
                    0x18 => Self::STST { reg },
                    0x1E => Self::ADDA { reg },
                    0x29 => Self::LDSA { reg },
                    0x33 => Self::STDR { reg },
                    0x55 => Self::STRR { reg },
                    0x65 => Self::LDRR { reg },
                    _ => Self::INV { opcode }
                }
            }
            _ => Self::INV { opcode }
        }
    }

    pub fn name(&self) -> &'static str {
        match self{
            Self::SYS  {..} => "SYS",
            Self::CLS       => "CLS",
            Self::RET       => "RET",
            Self::JP   {..} => "JP",
            Self::CALL {..} => "CALL",
            Self::SE   {..} => "SE",
            Self::SNE  {..} => "SNE",
            Self::SER  {..} => "SER",
            Self::LD   {..} => "LD",
            Self::ADD  {..} => "ADD",
            Self::LDR  {..} => "LDR",
            Self::OR   {..} => "OR",
            Self::AND  {..} => "AND",
            Self::XOR  {..} => "XOR",
            Self::ADDR {..} => "ADDR",
            Self::SUB  {..} => "SUB",
            Self::SHR  {..} => "SHR",
            Self::SUBN {..} => "SUBN",
            Self::SHL  {..} => "SHL",
            Self::SNER {..} => "SNER",
            Self::LDA  {..} => "LDA",
            Self::JPO  {..} => "JPO",
            Self::RND  {..} => "RND",
            Self::DRW  {..} => "DRW",
            Self::SKP  {..} => "SKP",
            Self::SKNP {..} => "SKNP",
            Self::LDDT {..} => "LDDT",
            Self::LDKP {..} => "LDKP",
            Self::STDT {..} => "STDT",
            Self::STST {..} => "STST",
            Self::ADDA {..} => "ADDA",
            Self::LDSA {..} => "LDSA",
            Self::STDR {..} => "STDR",
            Self::STRR {..} => "STRR",
            Self::LDRR {..} => "LDRR",
            Self::INV  {..} => "INV",
        }
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())?;
        match self{
            Self::CLS                           => Ok(()),
            Self::RET                           => Ok(()),
            Self::SYS  { addr }                 => write!(f, " {:#05X}", addr),
            Self::JP   { addr }                 => write!(f, " {:#05X}", addr),
            Self::CALL { addr }                 => write!(f, " {:#05X}", addr),
            Self::LDA  { addr }                 => write!(f, " {:#05X}", addr),
            Self::JPO  { addr }                 => write!(f, " {:#05X}", addr),
            Self::SKP  { reg }                  => write!(f, " V{:X}", reg),
            Self::SKNP { reg }                  => write!(f, " V{:X}", reg),
            Self::LDDT { reg }                  => write!(f, " V{:X}", reg),
            Self::LDKP { reg }                  => write!(f, " V{:X}", reg),
            Self::STDT { reg }                  => write!(f, " V{:X}", reg),
            Self::STST { reg }                  => write!(f, " V{:X}", reg),
            Self::ADDA { reg }                  => write!(f, " V{:X}", reg),
            Self::LDSA { reg }                  => write!(f, " V{:X}", reg),
            Self::STDR { reg }                  => write!(f, " V{:X}", reg),
            Self::STRR { reg }                  => write!(f, " V{:X}", reg),
            Self::LDRR { reg }                  => write!(f, " V{:X}", reg),
            Self::SE   { reg, byte }            => write!(f, " V{:X} {:#04X}", reg, byte),
            Self::SNE  { reg, byte }            => write!(f, " V{:X} {:#04X}", reg, byte),
            Self::LD   { reg, byte }            => write!(f, " V{:X} {:#04X}", reg, byte),
            Self::ADD  { reg, byte }            => write!(f, " V{:X} {:#04X}", reg, byte),
            Self::RND  { reg, byte }            => write!(f, " V{:X} {:#04X}", reg, byte),
            Self::SER  { reg_a, reg_b }         => write!(f, " V{:X} V{:X}", reg_a, reg_b),
            Self::LDR  { reg_a, reg_b }         => write!(f, " V{:X} V{:X}", reg_a, reg_b),
            Self::OR   { reg_a, reg_b }         => write!(f, " V{:X} V{:X}", reg_a, reg_b),
            Self::AND  { reg_a, reg_b }         => write!(f, " V{:X} V{:X}", reg_a, reg_b),
            Self::XOR  { reg_a, reg_b }         => write!(f, " V{:X} V{:X}", reg_a, reg_b),
            Self::ADDR { reg_a, reg_b }         => write!(f, " V{:X} V{:X}", reg_a, reg_b),
            Self::SUB  { reg_a, reg_b }         => write!(f, " V{:X} V{:X}", reg_a, reg_b),
            Self::SHR  { reg_a, reg_b }         => write!(f, " V{:X} V{:X}", reg_a, reg_b),
            Self::SUBN { reg_a, reg_b }         => write!(f, " V{:X} V{:X}", reg_a, reg_b),
            Self::SHL  { reg_a, reg_b }         => write!(f, " V{:X} V{:X}", reg_a, reg_b),
            Self::SNER { reg_a, reg_b }         => write!(f, " V{:X} V{:X}", reg_a, reg_b),
            Self::DRW  { reg_a, reg_b, nibble } => write!(f, " V{:X} V{:X} {:X}", reg_a, reg_b, nibble),
            Self::INV  { opcode }               => write!(f, " {:#06X}", opcode),
        }
    }
}
