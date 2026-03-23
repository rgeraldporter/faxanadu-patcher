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

    pub fn cpu_to_file_offset(&self, bank: usize, cpu_addr: u16) -> usize {
        let bank_base_addr = if cpu_addr >= 0xC000 { 0xC000u16 } else { 0x8000u16 };
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

    /// Search for a byte pattern within a given bank.
    /// Returns the file offset of the first match, or None.
    pub fn find_in_bank(&self, bank: usize, pattern: &[u8]) -> Option<usize> {
        let bank_start = self.cpu_to_file_offset(bank, if bank == 15 { 0xC000 } else { 0x8000 });
        let bank_end = bank_start + 0x4000; // 16KB per bank
        let search = &self.data[bank_start..bank_end];
        search.windows(pattern.len())
            .position(|w| w == pattern)
            .map(|pos| bank_start + pos)
    }

    /// Search for all occurrences of a byte pattern within a given bank.
    /// Returns a vec of file offsets.
    pub fn find_all_in_bank(&self, bank: usize, pattern: &[u8]) -> Vec<usize> {
        let bank_start = self.cpu_to_file_offset(bank, if bank == 15 { 0xC000 } else { 0x8000 });
        let bank_end = bank_start + 0x4000;
        let search = &self.data[bank_start..bank_end];
        search.windows(pattern.len())
            .enumerate()
            .filter(|(_, w)| *w == pattern)
            .map(|(pos, _)| bank_start + pos)
            .collect()
    }

    /// Search for a masked byte pattern within a given bank.
    /// `mask` has the same length as `pattern`. A mask byte of $FF means
    /// the corresponding pattern byte must match exactly; $00 means any
    /// byte is accepted (wildcard).
    /// Returns the file offset of the first match, or None.
    pub fn find_masked_in_bank(&self, bank: usize, pattern: &[u8], mask: &[u8]) -> Option<usize> {
        assert_eq!(pattern.len(), mask.len());
        let bank_start = self.cpu_to_file_offset(bank, if bank == 15 { 0xC000 } else { 0x8000 });
        let bank_end = bank_start + 0x4000;
        let search = &self.data[bank_start..bank_end];
        search.windows(pattern.len())
            .position(|w| {
                w.iter().zip(pattern.iter()).zip(mask.iter())
                    .all(|((b, p), m)| b & m == p & m)
            })
            .map(|pos| bank_start + pos)
    }

    /// Search for all occurrences of a masked byte pattern within a given bank.
    pub fn find_all_masked_in_bank(&self, bank: usize, pattern: &[u8], mask: &[u8]) -> Vec<usize> {
        assert_eq!(pattern.len(), mask.len());
        let bank_start = self.cpu_to_file_offset(bank, if bank == 15 { 0xC000 } else { 0x8000 });
        let bank_end = bank_start + 0x4000;
        let search = &self.data[bank_start..bank_end];
        search.windows(pattern.len())
            .enumerate()
            .filter(|(_, w)| {
                w.iter().zip(pattern.iter()).zip(mask.iter())
                    .all(|((b, p), m)| b & m == p & m)
            })
            .map(|(pos, _)| bank_start + pos)
            .collect()
    }

}
