#![allow(dead_code)]
use crate::consts::{mem, op, subaddr, BANK_BASE_ADDR};
use crate::rom::Rom;

pub struct Subroutine {
    bytes: Vec<u8>,
}

impl Subroutine {
    pub fn new() -> Self {
        Self {
            bytes: Vec::with_capacity(256),
        }
    }

    #[inline]
    fn le16(v: u16) -> [u8; 2] {
        [(v & 0xFF) as u8, (v >> 8) as u8]
    }

    pub fn add(&mut self, opcode: u8) {
        self.bytes.push(opcode);
    }

    pub fn add_imm(&mut self, opcode: u8, val: u8) {
        self.bytes.push(opcode);
        self.bytes.push(val);
    }

    pub fn add_abs(&mut self, opcode: u8, addr: u16) {
        self.bytes.push(opcode);
        self.bytes.extend_from_slice(&Self::le16(addr));
    }

    pub fn jsr(&mut self, target: u16) {
        self.add_abs(op::JSR, target);
    }

    pub fn jmp(&mut self, target: u16) {
        self.add_abs(op::JMP_ABS, target);
    }

    pub fn rts(&mut self) {
        self.add(op::RTS);
    }

    pub fn set_ascii_value(&mut self, ptr: u16) {
        self.add_abs(op::LDA_ABS, ptr);
        self.add_abs(op::STA_ABS, mem::BYTE_EC);
    }

    // Clear ED/EE and set row base to $20
    pub fn prepare_ascii(&mut self) {
        // wipe ED/EE
        self.add_imm(op::LDA_IMM, 0x00);
        self.add_abs(op::STA_ABS, mem::BYTE_ED);
        self.add_abs(op::STA_ABS, mem::BYTE_EE);

        // base index row to $20
        self.add_imm(op::LDA_IMM, 0x20);
        self.add_abs(op::STA_ABS, mem::BYTE_E9);
    }

    pub fn set_ascii_length(&mut self, len: u8) {
        self.add_imm(op::LDY_IMM, len);
    }

    pub fn set_ascii_position(&mut self, pos: u8) {
        self.add_imm(op::LDA_IMM, pos);
        self.add_abs(op::STA_ABS, mem::BYTE_E8);
    }

    pub fn show_ascii(&mut self) {
        self.jsr(subaddr::SHOW_ASCII);
    }

    pub fn add_to_rom(self, dest_cpu: u16, bank: usize, rom: &mut Rom) {
        let offset = rom.cpu_to_file_offset(bank, dest_cpu, BANK_BASE_ADDR);
        rom.write_slice(offset, &self.bytes);
    }

    /// Return the raw bytes of the subroutine
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }
}
