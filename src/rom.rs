#![allow(dead_code)]
use std::{fs, io, path::Path};

use crate::consts::{BANK_SIZE, NES_HEADER_LEN};

pub struct Rom {
    data: Vec<u8>,
}

impl Rom {
    pub fn from_file<P: AsRef<Path>>(p: P) -> io::Result<Self> {
        Ok(Self { data: fs::read(p)? })
    }

    pub fn save<P: AsRef<Path>>(&self, p: P) -> io::Result<()> {
        fs::write(p, &self.data)
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    pub fn bank_base(&self, bank: usize) -> usize {
        let header = if self.data.len() >= NES_HEADER_LEN && &self.data[0..4] == b"NES\x1A" {
            NES_HEADER_LEN
        } else {
            0
        };
        header + bank * BANK_SIZE
    }

    pub fn cpu_to_file_offset(&self, bank: usize, cpu_addr: u16, bank_base_addr: u16) -> usize {
        let base = self.bank_base(bank);
        let within = (cpu_addr - bank_base_addr) as usize;
        base + within
    }

    pub fn read_byte(&self, offset: usize) -> u8 {
        self.data[offset]
    }

    pub fn write_byte(&mut self, offset: usize, val: u8) {
        self.data[offset] = val;
    }

    pub fn write_slice(&mut self, offset: usize, bytes: &[u8]) {
        self.data[offset..offset + bytes.len()].copy_from_slice(bytes);
    }

    /// Read a little-endian 16-bit word from file offset.
    pub fn read_word(&self, off: usize) -> u16 {
        let lo = self.read_byte(off) as u16;
        let hi = self.read_byte(off + 1) as u16;
        lo | (hi << 8)
    }

    /// Write a little-endian 16-bit word to file offset.
    pub fn write_word(&mut self, off: usize, val: u16) {
        self.write_byte(off, (val & 0xFF) as u8);
        self.write_byte(off + 1, (val >> 8) as u8);
    }

    pub fn write_bytes(&mut self, offset: usize, bytes: &[u8]) {
        self.data[offset..offset + bytes.len()].copy_from_slice(bytes);
    }

    /// Write a slice of bytes directly at a given file offset.
    pub fn write_slice_at(&mut self, offset: usize, bytes: &[u8]) {
        let data = self.data_mut();
        let end = offset + bytes.len();
        data[offset..end].copy_from_slice(bytes);
    }

    pub fn find_free_block(&self, bank: usize, size: usize) -> Option<(usize, u16)> {
        let base = self.bank_base(bank);
        let bank_data = &self.data[base..base + crate::consts::BANK_SIZE];

        for i in 0..=crate::consts::BANK_SIZE - size {
            if bank_data[i..i + size].iter().all(|&b| b == 0xFF) {
                let cpu_addr = crate::consts::BANK_BASE_ADDR as u16 + i as u16;
                return Some((base + i, cpu_addr));
            }
        }

        None
    }
}
