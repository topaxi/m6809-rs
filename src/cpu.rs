use mem::Mem;

const CARRY_FLAG: u8 = 1 << 0;
const OVERFLOW_FLAG: u8 = 1 << 1;
const ZERO_FLAG: u8 = 1 << 2;
const NEGATIVE_FLAG: u8 = 1 << 3;
const IRQ_MASK_FLAG: u8 = 1 << 4;
const HALF_HARRY_FLAG: u8 = 1 << 5;
const FIRQ_MASK_FLAG: u8 = 1 << 6;
const ENTIRE_SAVE_FLAG: u8 = 1 << 7;

const RESET_VECTOR: u16 = 0xFFFE;
const NMI_VECTOR: u16 = 0xFFFC;
const SWI_VECTOR: u16 = 0xFFFA;
const IRQ_VECTOR: u16 = 0xFFF8;
const FIRQ_VECTOR: u16 = 0xFFF6;
const SWI2_VECTOR: u16 = 0xFFF4;
const SWI3_VECTOR: u16 = 0xFFF2;

#[cfg_attr(rustfmt, rustfmt_skip)]
static CYCLE_TABLE: [u8; 256] = [
    /* 00-0F */ 6, 0, 0, 6, 6, 0, 6, 6, 6, 6, 6, 0, 6, 6, 3, 6,
    /* 10-1F */ 0, 0, 2, 4, 0, 0, 5, 9, 0, 2, 3, 0, 3, 2, 8, 6,
    /* 20-2F */ 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
    /* 30-3F */ 4, 4, 4, 4, 5, 5, 5, 5, 0, 5, 3, 6, 9,11, 0,19,
    /* 40-4F */ 2, 0, 0, 2, 2, 0, 2, 2, 2, 2, 2, 0, 2, 2, 0, 2,
    /* 50-5F */ 2, 0, 0, 2, 2, 0, 2, 2, 2, 2, 2, 0, 2, 2, 0, 2,
    /* 60-6F */ 6, 0, 0, 6, 6, 0, 6, 6, 6, 6, 6, 0, 6, 6, 3, 6,
    /* 70-7F */ 7, 0, 0, 7, 7, 0, 7, 7, 7, 7, 7, 0, 7, 7, 4, 7,
    /* 80-8F */ 2, 2, 2, 4, 2, 2, 2, 0, 2, 2, 2, 2, 4, 7, 3, 0,
    /* 90-9F */ 4, 4, 4, 6, 4, 4, 4, 4, 4, 4, 4, 4, 6, 7, 5, 5,
    /* A0-AF */ 4, 4, 4, 6, 4, 4, 4, 4, 4, 4, 4, 4, 6, 7, 5, 5,
    /* B0-BF */ 5, 5, 5, 7, 5, 5, 5, 5, 5, 5, 5, 5, 7, 8, 6, 6,
    /* C0-CF */ 2, 2, 2, 4, 2, 2, 2, 0, 2, 2, 2, 2, 3, 0, 3, 0,
    /* D0-DF */ 4, 4, 4, 6, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5,
    /* E0-EF */ 4, 4, 4, 6, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5,
    /* F0-FF */ 5, 5, 5, 7, 5, 5, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6,
];

#[cfg_attr(rustfmt, rustfmt_skip)]
static CYCLE_TABLE2: [u8; 256] = [
    /* 00-0F */ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    /* 10-1F */ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    /* 20-2F */ 0, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
    /* 30-3F */ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,20,
    /* 40-4F */ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    /* 50-5F */ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    /* 60-6F */ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    /* 70-7F */ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    /* 80-8F */ 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 4, 0,
    /* 90-9F */ 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 6, 6,
    /* A0-AF */ 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 6, 6,
    /* B0-BF */ 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 7, 7,
    /* C0-CF */ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0,
    /* D0-DF */ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 6,
    /* E0-EF */ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 6,
    /* F0-FF */ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 7
];

enum AddrType {
    None,
    Immediate,
    Relative,
    Extended,
    Direct,
    Indexed,
}

pub struct Cpu<M: Mem> {
    ix: u16,
    iy: u16,

    su: u16,
    ss: u16,

    pc: u16,

    aa: u8,
    ab: u8,

    dp: u8,
    cc: u8,

    mem: M,
    stk: [u16; 16],

    cy: u64,

    addr_type: AddrType,
}

impl<M: Mem> Mem for Cpu<M> {
    fn read(&self, addr: u16) -> u8 {
        self.mem.read(addr)
    }

    fn store(&mut self, addr: u16, val: u8) {
        self.mem.store(addr, val);
    }

    fn copy_into(&self, mut slice: &mut [u8]) {
        self.mem.copy_into(&mut slice);
    }
}

impl<M: Mem> Cpu<M> {
    fn fetch(&mut self) -> u8 {
        let val = self.read(self.pc);
        self.pc += 1;
        val
    }

    fn fetch_word(&mut self) -> u16 {
        let val = self.read_word(self.pc);
        self.pc += 2;
        val
    }

    #[inline(always)]
    fn fetch_operand(&mut self) -> u16 {
        match self.addr_type {
            AddrType::Immediate | AddrType::Relative => self.fetch() as u16,
            AddrType::Extended => self.fetch_word(),
            AddrType::Direct => self.dp_add(),
            AddrType::Indexed => panic!("Not implemented yet"),
            _ => panic!("Invalid addressing type"),
        }
    }

    pub fn reset(&mut self) {
        self.pc = self.read_word(RESET_VECTOR);
        self.dp = 0x00;
        self.cc = 0x00 | IRQ_MASK_FLAG | FIRQ_MASK_FLAG;
        self.cy = 0x00;
        self.ss = 0x27FF;
    }

    pub fn go(&mut self, addr: u16) {
        self.pc = addr;
    }

    #[inline(always)]
    pub fn step(&mut self) -> u64 {
        let opcode = self.fetch();

        self.process_opcode(opcode)
    }

    #[inline(always)]
    fn get_addr_type(&self, opcode: u8, opcode2: u8) -> AddrType {
        match opcode & 0xF0 {
            0x00 | 0x90 | 0xD0 => AddrType::Direct,
            0x20 => AddrType::Relative,
            0x30 | 0x40 | 0x50 if opcode < 0x34 => AddrType::Indexed,
            0x30 | 0x40 | 0x50 if opcode < 0x38 => AddrType::Immediate,
            0x30 | 0x40 | 0x50 => AddrType::None,
            0x60 | 0xA0 | 0xE0 => AddrType::Indexed,
            0x70 | 0xB0 | 0xF0 => AddrType::Extended,
            _ if opcode == 0x8D => AddrType::Relative,
            0x80 | 0xc0 => AddrType::Immediate,
            0x10 => match opcode & 0x0F {
                0x02 | 0x03 | 0x09 | 0x0D | 0x0E | 0x0F => AddrType::None,
                0x06 | 0x07 => AddrType::Relative,
                0x0A | 0x0C => AddrType::Immediate,
                0x00 | 0x01 => match opcode2 & 0xF0 {
                    0x20 => AddrType::Relative,
                    0x30 => AddrType::None,
                    0x80 | 0xC0 => AddrType::Immediate,
                    0x90 | 0xD0 => AddrType::Direct,
                    0xA0 | 0xE0 => AddrType::Indexed,
                    0xB0 | 0xF0 => AddrType::Extended,
                    _ => AddrType::None,
                },
                _ => AddrType::None,
            },
            _ => AddrType::None,
        }
    }

    fn process_opcode(&mut self, opcode: u8) -> u64 {
        let cy = self.cy;
        self.cy += CYCLE_TABLE[opcode as usize] as u64;

        // Opcodes of 0x10 and 0x11 are 16bit wide
        let opcode2 = if opcode == 0x10 || opcode == 0x11 {
            let opcode2 = self.fetch();
            self.cy += CYCLE_TABLE2[opcode2 as usize] as u64;
            opcode2
        } else {
            0
        };

        self.addr_type = self.get_addr_type(opcode, opcode2);

        match (opcode, opcode2) {
            /* NEG DP */ (0x00, _) |
            /* NEG extended */ (0x70, _) => {
                let addr = self.fetch_operand();
                let byte = self.neg(self.mem.read(addr));
                self.mem.store(addr, byte);
            }
            /* COM DP */ (0x03, _) |
            /* COM extended */ (0x73, _) => {
                let addr = self.fetch_operand();
                let byte = self.com(self.mem.read(addr));
                self.mem.store(addr, byte);
            }
            /* LSR DP */ (0x04, _) |
            /* LSR extended */ (0x74, _) => {
                let addr = self.fetch_operand();
                let byte = self.lsr(self.mem.read(addr));
                self.mem.store(addr, byte);
            }
            /* ROR DP */ (0x06, _) |
            /* ROR extended */ (0x76, _) => {
                let addr = self.fetch_operand();
                let byte = self.ror(self.mem.read(addr));
                self.mem.store(addr, byte);
            }
            (0x07, _) => (), //ASR DP
            (0x08, _) => (), //ASL DP
            (0x09, _) => (), //ROL DP
            (0x0A, _) => (), //DEC DP
            (0x0C, _) => (), //INC DP
            (0x0D, _) => (), //TST DP
            (0x0E, _) => (), //JMP DP
            (0x0F, _) => (), //CLR DP
            (0x12, _) => (), //NOP
            (0x13, _) => (), //SYNC
            (0x16, _) => (), //LBRA relative
            (0x17, _) => (), //LBSR relative
            (0x19, _) => (), //DAA
            (0x1A, _) => (), //ORCC
            (0x1C, _) => (), //ANDCC
            (0x1D, _) => (), //SEX
            (0x1E, _) => (), //EXG
            (0x1F, _) => (), //EXG
            (0x20, _) => (), //BRA
            (0x21, _) => (), //BRN
            (0x22, _) => (), //BHI
            (0x23, _) => (), //BLS
            (0x24, _) => (), //BCC
            (0x25, _) => (), //BCS
            (0x26, _) => (), //BNE
            (0x27, _) => (), //BEQ
            (0x28, _) => (), //BVC
            (0x29, _) => (), //BVS
            (0x2A, _) => (), //BPL
            (0x2B, _) => (), //BMI
            (0x2C, _) => (), //BGE
            (0x2D, _) => (), //BLT
            (0x2E, _) => (), //BGT
            (0x2F, _) => (), //BLE
            (0x30, _) => (), //LEAX
            (0x31, _) => (), //LEAY
            (0x32, _) => (), //LEAS
            (0x33, _) => (), //LEAU
            (0x34, _) => (), //PSHS
            (0x35, _) => (), //PULS
            (0x36, _) => (), //PSHU
            (0x37, _) => (), //PULU
            //RTS
            (0x39, _) => {
                self.pc = self.pull_word_s();
            },
            (0x3A, _) => (), //ABX
            (0x3B, _) => (), //RTI
            (0x3C, _) => (), //CWAI
            (0x3D, _) => (), //MUL
            (0x3F, _) => (), //SWI
            //NEGA
            (0x40, _) => self.aa = self.neg(self.aa),
            //COMA
            (0x43, _) => self.aa = self.com(self.aa),
            //LSRA
            (0x44, _) => self.aa = self.lsr(self.aa),
            //RORA
            (0x46, _) => self.aa = self.ror(self.aa),
            (0x47, _) => (),
            (0x48, _) => (),
            (0x49, _) => (),
            (0x4A, _) => (),
            (0x4C, _) => (),
            (0x4D, _) => (),
            (0x4F, _) => (),
            //NEGB
            (0x50, _) => self.ab = self.neg(self.ab),
            //COMB
            (0x53, _) => self.ab = self.com(self.ab),
            //LSRB
            (0x54, _) => self.ab = self.lsr(self.ab),
            //RORB
            (0x56, _) => self.ab = self.ror(self.ab),
            (0x57, _) => (),
            (0x58, _) => (),
            (0x59, _) => (),
            (0x5A, _) => (),
            (0x5C, _) => (),
            (0x5D, _) => (),
            (0x5F, _) => (),
            (0x60, _) => (), //NEG indexed
            (0x63, _) => (), //COM indexed
            (0x64, _) => (), //LSR indexed
            (0x66, _) => (), //ROR indexed
            (0x67, _) => (), //ASR indexed
            (0x68, _) => (), //ASL indexed
            (0x69, _) => (), //ROL indexed
            (0x6A, _) => (), //DEC indexed
            (0x6C, _) => (), //INC indexed
            (0x6D, _) => (), //TST indexed
            (0x6E, _) => (), //JMP indexed
            (0x6F, _) => (), //CLR indexed
            (0x77, _) => (),    //ASR extended
            (0x78, _) => (),    //ASL extended
            (0x79, _) => (),    //ROL extended
            (0x7A, _) => (),    //DEC extended
            (0x7C, _) => (),    //INC extended
            (0x7D, _) => (),    //TST extended
            //JMP extended
            (0x7E, _) => {
                self.pc = self.fetch_operand();
            },
            (0x7F, _) => (),    //CLR extended
            (0x80, _) => (),    //SUBA imm
            (0x81, _) => (),    //CMPA imm
            (0x82, _) => (),    //SBCA imm
            (0x83, _) => (),    //SUBD imm
            //ANDA imm
            (0x84, _) => {
                let byte = self.fetch_operand() as u8;
                self.aa = self.and(byte, self.aa);
            },
            (0x85, _) => (),    //BITA imm
            (0x86, _) => (),    //LDA imm
            //EORA imm
            (0x88, _) => {
                let byte = self.fetch_operand() as u8;
                self.aa = self.eor(byte, self.aa);
            },
            (0x89, _) => (),    //ADCA imm
            //ORA imm
            (0x8A, _) => {
                let byte = self.fetch_operand() as u8;
                self.aa = self.or(byte, self.aa);
            },
            (0x8B, _) => (),    //ADDA imm
            (0x8C, _) => (),    //CMPX imm
            (0x8D, _) => (),    //JSR imm
            (0x8E, _) => (),    //LDX imm
            (0x90, _) => (),    //SUBA direct
            (0x91, _) => (),    //CMPA direct
            (0x92, _) => (),    //SBCA direct
            (0x93, _) => (),    //SUBD direct
            (0x94, _) => (),    //ANDA direct
            (0x95, _) => (),    //BITA direct
            (0x96, _) => (),    //LDA direct
            (0x97, _) => (),    //STA direct
            (0x98, _) => (),    //EORA direct
            (0x99, _) => (),    //ADCA direct
            (0x9A, _) => (),    //ORA direct
            (0x9B, _) => (),    //ADDA direct
            (0x9C, _) => (),    //CMPX direct
            (0x9D, _) => (),    //JSR direct
            (0x9E, _) => (),    //LDX direct
            (0x9F, _) => (),    //STX direct
            (0xA0, _) => (),    //SUBA indexed
            (0xA1, _) => (),    //CMPA indexed
            (0xA2, _) => (),    //SBCA indexed
            (0xA3, _) => (),    //SUBD indexed
            (0xA4, _) => (),    //ANDA indexed
            (0xA5, _) => (),    //BITA indexed
            (0xA6, _) => (),    //LDA indexed
            (0xA7, _) => (),    //STA indexed
            (0xA8, _) => (),    //EORA indexed
            (0xA9, _) => (),    //ADCA indexed
            (0xAA, _) => (),    //ORA indexed
            (0xAB, _) => (),    //ADDA indexed
            (0xAC, _) => (),    //CMPX indexed
            (0xAD, _) => (),    //JSR indexed
            (0xAE, _) => (),    //LDX indexed
            (0xAF, _) => (),    //STX indexed
            (0xB0, _) => (),    //SUBA extended
            (0xB1, _) => (),    //CMPA extended
            (0xB2, _) => (),    //SBCA extended
            (0xB3, _) => (),    //SUBD extended
            (0xB4, _) => (),    //ANDA extended
            (0xB5, _) => (),    //BITA extended
            //LDA extended
            (0xB6, _) => {
                let addr = self.fetch_operand();
                self.lda(self.mem.read(addr));
            },
            //STA extended
            (0xB7, _) => {
                let addr = self.fetch_operand();
                self.sta(addr);
            },
            (0xB8, _) => (),    //EORA extended
            (0xB9, _) => (),    //ADCA extended
            (0xBA, _) => (),    //ORA extended
            (0xBB, _) => (),    //ADDA extended
            (0xBC, _) => (),    //CMPX extended
            //JSR extended
            (0xBD, _) => {
                let addr = self.fetch_operand();
                self.push_word_s(self.pc);
                self.pc = addr;
            },
            (0xBE, _) => (),    //LDX extended
            (0xBF, _) => (),    //STX extended
            (0xC0, _) => (),    //SUBB imm
            (0xC1, _) => (),    //CMPB imm
            (0xC2, _) => (),    //SBCB imm
            (0xC3, _) => (),    //ADDD imm
            (0xC4, _) => (),    //ANDB imm
            (0xC5, _) => (),    //BITB imm
            (0xC6, _) => (),    //LDB imm
            (0xC8, _) => (),    //EORB imm
            (0xC9, _) => (),    //ADCB imm
            (0xCA, _) => (),    //ORB imm
            (0xCB, _) => (),    //ADDB imm
            (0xCC, _) => (),    //LDD imm
            (0xCE, _) => (),    //LDU imm
            (0xD0, _) => (),    //SUBB direct
            (0xD1, _) => (),    //CMPB direct
            (0xD2, _) => (),    //SBCB direct
            (0xD3, _) => (),    //ADDD direct
            (0xD4, _) => (),    //ANDB direct
            (0xD5, _) => (),    //BITB direct
            (0xD6, _) => (),    //LDB direct
            (0xD7, _) => (),    //STB direct
            (0xD8, _) => (),    //EORB direct
            (0xD9, _) => (),    //ADCB direct
            (0xDA, _) => (),    //ORB direct
            (0xDB, _) => (),    //ADDB direct
            (0xDC, _) => (),    //LDD direct
            (0xDD, _) => (),    //STD direct
            (0xDE, _) => (),    //LDU direct
            (0xDF, _) => (),    //STU direct
            (0xE0, _) => (),    //SUBB indexed
            (0xE1, _) => (),    //CMPB indexed
            (0xE2, _) => (),    //SBCB indexed
            (0xE3, _) => (),    //ADDD indexed
            (0xE4, _) => (),    //ANDB indexed
            (0xE5, _) => (),    //BITB indexed
            (0xE6, _) => (),    //LDB indexed
            (0xE7, _) => (),    //STB indexed
            (0xE8, _) => (),    //EORB indexed
            (0xE9, _) => (),    //ADCB indexed
            (0xEA, _) => (),    //ORB indexed
            (0xEB, _) => (),    //ADDB indexed
            (0xEC, _) => (),    //LDD indexed
            (0xED, _) => (),    //STD indexed
            (0xEE, _) => (),    //LDU indexed
            (0xEF, _) => (),    //STU indexed
            (0xF0, _) => (),    //SUBB extended
            (0xF1, _) => (),    //CMPB extended
            (0xF2, _) => (),    //SBCB extended
            (0xF3, _) => (),    //ADDD extended
            (0xF4, _) => (),    //ANDB extended
            (0xF5, _) => (),    //BITB extended
            (0xF6, _) => (),    //LDB extended
            (0xF7, _) => (),    //STB extended
            (0xF8, _) => (),    //EORB extended
            (0xF9, _) => (),    //ADCB extended
            (0xFA, _) => (),    //ORB extended
            (0xFB, _) => (),    //ADDB extended
            (0xFC, _) => (),    //LDD extended
            (0xFD, _) => (),    //STD extended
            (0xFE, _) => (),    //LDU extended
            (0xFF, _) => (),    //STU extended
            (0x10, 0x21) => (), //BRN
            (0x10, 0x22) => (), //BHI
            (0x10, 0x23) => (), //BLS
            (0x10, 0x24) => (), //BCC
            (0x10, 0x25) => (), //BCS
            (0x10, 0x26) => (), //BNE
            (0x10, 0x27) => (), //BEQ
            (0x10, 0x28) => (), //BVC
            (0x10, 0x29) => (), //BVS
            (0x10, 0x2A) => (), //BPL
            (0x10, 0x2B) => (), //BMI
            (0x10, 0x2C) => (), //BGE
            (0x10, 0x2D) => (), //BLT
            (0x10, 0x2E) => (), //BGT
            (0x10, 0x2F) => (), //BLE
            (0x10, 0x3f) => (), //SWI2
            (0x10, 0x83) => (), //CMPD imm
            (0x10, 0x8C) => (), //CMPY imm
            (0x10, 0x8E) => (), //LDY imm
            (0x10, 0x93) => (), //CMPD direct
            (0x10, 0x9C) => (), //CMPY direct
            (0x10, 0x9E) => (), //LDY direct
            (0x10, 0x9F) => (), //STY direct
            (0x10, 0xA3) => (), //CMPD indexed
            (0x10, 0xAC) => (), //CMPY indexed
            (0x10, 0xAE) => (), //LDY indexed
            (0x10, 0xAF) => (), //STY indexed
            (0x10, 0xB3) => (), //CMPD extended
            (0x10, 0xBC) => (), //CMPY extended
            (0x10, 0xBE) => (), //LDY extended
            (0x10, 0xBF) => (), //STY extended
            (0x10, 0xCE) => (), //LDS imm
            (0x10, 0xDE) => (), //LDS direct
            (0x10, 0xDF) => (), //STS direct
            (0x10, 0xEE) => (), //LDS indexed
            (0x10, 0xEF) => (), //STS indexed
            (0x10, 0xFE) => (), //LDS extended
            (0x10, 0xFF) => (), //STS extended
            (0x11, 0x3F) => (), //SWI3
            (0x11, 0x83) => (), //CMPU imm
            (0x11, 0x8C) => (), //CMPS imm
            (0x11, 0x93) => (), //CMPU imm
            (0x11, 0x9C) => (), //CMPS imm
            (0x11, 0xA3) => (), //CMPU imm
            (0x11, 0xAC) => (), //CMPS imm
            (0x11, 0xB3) => (), //CMPU imm
            (0x11, 0xBC) => (), //CMPS imm
            (_, _) => panic!("Unknown opcode!"),
        }

        self.cy - cy
    }

    pub fn run_cycles(&mut self, mut cycles: u64) {
        while cycles != 0 {
            let cy = self.step();

            if cy >= cycles {
                break;
            }

            cycles -= cy;
        }
    }

    pub fn new(mem: M) -> Cpu<M> {
        Cpu {
            ix: 0,
            iy: 0,
            su: 0,
            ss: 0,
            pc: 0,
            aa: 0,
            ab: 0,
            dp: 0,
            cc: 0,
            mem,
            stk: [0; 16],
            cy: 0,
            addr_type: AddrType::None,
        }
    }

    #[inline(always)]
    fn dp_add(&mut self) -> u16 {
        ((self.dp as u16) << 8) + self.fetch() as u16
    }

    fn neg(&mut self, byte: u8) -> u8 {
        self.cc &= !(CARRY_FLAG | ZERO_FLAG | OVERFLOW_FLAG | NEGATIVE_FLAG);

        let mut byte = byte;

        if byte == 0x80 {
            byte |= OVERFLOW_FLAG;
        }

        byte = 0 - byte;

        if byte == 0 {
            self.cc |= ZERO_FLAG
        }

        if byte & 0x80 != 0 {
            self.cc |= NEGATIVE_FLAG | CARRY_FLAG;
        }

        byte
    }

    fn com(&mut self, byte: u8) -> u8 {
        self.cc &= !(ZERO_FLAG | NEGATIVE_FLAG | OVERFLOW_FLAG);
        let byte = !byte;
        self.flag_nz(byte);
        self.cc |= CARRY_FLAG;
        byte
    }

    fn lsr(&mut self, byte: u8) -> u8 {
        self.cc &= !(ZERO_FLAG | CARRY_FLAG | NEGATIVE_FLAG);

        if byte & 0x01 != 0 {
            self.cc |= CARRY_FLAG
        }

        let byte = byte >> 1;

        if byte == 0 {
            self.cc |= ZERO_FLAG
        }

        byte
    }

    fn ror(&mut self, byte: u8) -> u8 {
        let cc = self.cc;
        self.cc &= !(ZERO_FLAG | CARRY_FLAG | NEGATIVE_FLAG);
        if byte & 0x01 != 0 {
            self.cc |= CARRY_FLAG;
        }

        let byte = byte >> 1 | cc << 7;
        self.flag_nz(byte);
        byte
    }

    fn or(&mut self, byte1: u8, byte2: u8) -> u8 {
        self.cc &= !(ZERO_FLAG | NEGATIVE_FLAG | OVERFLOW_FLAG);
        let byte = byte1 | byte2;
        self.flag_nz(byte);
        self.cc |= CARRY_FLAG;
        byte
    }

    fn eor(&mut self, byte1: u8, byte2: u8) -> u8 {
        self.cc &= !(ZERO_FLAG | NEGATIVE_FLAG | OVERFLOW_FLAG);
        let byte = byte1 ^ byte2;
        self.flag_nz(byte);
        self.cc |= CARRY_FLAG;
        byte
    }

    fn and(&mut self, byte1: u8, byte2: u8) -> u8 {
        self.cc &= !(ZERO_FLAG | NEGATIVE_FLAG | OVERFLOW_FLAG);
        let byte = byte1 & byte2;
        self.flag_nz(byte);
        self.cc |= CARRY_FLAG;
        byte
    }

    fn push_s(&mut self, byte: u8) {
        self.ss -= 1;
        self.mem.store(self.ss, byte);
    }

    fn push_word_s(&mut self, word: u16) {
        self.push_s(((word >> 0) & 0xFF) as u8);
        self.push_s(((word >> 8) & 0xFF) as u8);
    }

    fn pull_s(&mut self) -> u8 {
        let byte = self.mem.read(self.ss);
        self.ss += 1;
        byte
    }

    fn pull_word_s(&mut self) -> u16 {
        ((self.pull_s() as u16) << 8) | (self.pull_s() as u16)
    }
    fn lda(&mut self, byte: u8) {
        self.aa = byte;
        self.cc = !(ZERO_FLAG | NEGATIVE_FLAG | OVERFLOW_FLAG);
        self.flag_nz(byte);
    }

    fn sta(&mut self, addr: u16) {
        self.mem.store(addr, self.aa);
        self.cc = !(ZERO_FLAG | NEGATIVE_FLAG | OVERFLOW_FLAG);
        self.flag_nz(self.aa);
    }

    #[inline(always)]
    fn flag_nz(&mut self, byte: u8) {
        self.cc |= match byte {
            0 => ZERO_FLAG,
            _ if (byte > 0x7F) => NEGATIVE_FLAG,
            _ => 0,
        };
    }
}
