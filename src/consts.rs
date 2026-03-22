#![allow(dead_code)]

pub const NES_HEADER_LEN: usize = 16;
pub const BANK_SIZE: usize = 0x4000; // PRG bank size
pub const TARGET_BANK: usize = 15;
pub const BANK_BASE_ADDR: u16 = 0xC000;

pub mod op {
    pub const JSR: u8 = 0x20;
    pub const RTS: u8 = 0x60;
    pub const JMP_ABS: u8 = 0x4C;

    pub const LDA_ABS: u8 = 0xAD;
    pub const STA_ABS: u8 = 0x8D;
    pub const STY_ABS: u8 = 0x8C;
    pub const LDY_IMM: u8 = 0xA0;
    pub const LDA_IMM: u8 = 0xA9;

    pub const DEC_ABS: u8 = 0xCE;
}

pub mod mem {
    pub const BYTE_E8: u16 = 0x00E8;
    pub const BYTE_E9: u16 = 0x00E9;
    pub const BYTE_EA: u16 = 0x00EA;
    pub const BYTE_EB: u16 = 0x00EB;
    pub const BYTE_EC: u16 = 0x00EC;
    pub const BYTE_ED: u16 = 0x00ED;
    pub const BYTE_EE: u16 = 0x00EE;
    pub const BYTE_EF: u16 = 0x00EF;

    pub const DURATION_OINTMENT: u16 = 0x0427;
    pub const DURATION_GLOVE: u16 = 0x0428;
    pub const DURATION_WINGBOOTS: u16 = 0x0429;
    pub const DURATION_HOURGLASS: u16 = 0x042A;
    pub const INTERRUPT_COUNTER: u16 = 0x001A; // essentially a frame counter
    pub const CURRENT_SCREEN: u16 = 0x0063;
    pub const CURRENT_LEVEL: u16 = 0x0024;
    pub const HEALTH_FULL: u16 = 0x0431;
    pub const HEALTH_FRAC: u16 = 0x0432;
    pub const MANA_POINTS: u16 = 0x039A;
}

// Banks whose free space is discovered dynamically by scanning the ROM
// for trailing 0xFF bytes. Stored as (bank, cpu_base, cpu_end_exclusive).
// The scan walks backwards from cpu_end to find the first non-0xFF byte.
pub const SCAN_BANKS: &[(usize, u16, u16)] = &[
    (9,  0x8000, 0xC000), // bank 9  (overflow; fax-edit fills bottom-up with level data)
    (12, 0x8000, 0xC000), // bank 12 (scripts, shops, passwords)
    (14, 0x8000, 0xC000), // bank 14 (sprite/player interaction)
    (15, 0xC000, 0xFFE0), // bank 15 (main logic, fixed bank; stop before vectors)
];

// Build a JSR instruction targeting the given CPU address.
pub fn jsr_bytes(target: u16) -> [u8; 3] {
    [op::JSR, (target & 0xFF) as u8, (target >> 8) as u8]
}

// Build a JMP (absolute) instruction targeting the given CPU address.
pub fn jmp_bytes(target: u16) -> [u8; 3] {
    [op::JMP_ABS, (target & 0xFF) as u8, (target >> 8) as u8]
}

// Known subroutines (labels from faxdump)
pub mod subaddr {
    pub const SHOW_ASCII: u16 = 0xFA06;
    pub const UI_DRAW_TIME_VALUE: u16 = 0xF990;
    pub const PPU_BUFFER_HEADER: u16 = 0xCFDC;
    pub const PPU_BUFFER_TILE: u16 = 0xF845;
}

