#![allow(dead_code)]
use std::collections::HashMap;

use crate::consts::SCAN_BANKS;
use crate::rom::Rom;

struct BankRegion {
    start: u16,
    next: u16,
    end: u16,
}

pub struct FreeSpaceAllocator {
    banks: HashMap<usize, BankRegion>,
}

impl FreeSpaceAllocator {
    // Create a new allocator by scanning the ROM for trailing 0xFF bytes
    // in each bank listed in `SCAN_BANKS`.
    pub fn new(rom: &Rom) -> Self {
        let mut banks = HashMap::new();

        for &(bank, cpu_base, cpu_end) in SCAN_BANKS {
            let size = (cpu_end - cpu_base) as usize;
            let file_off = rom.cpu_to_file_offset(bank, cpu_base);
            let data = &rom.data()[file_off..file_off + size];

            // Walk backwards to find the last non-0xFF byte
            let mut last_used = None;
            for i in (0..size).rev() {
                if data[i] != 0xFF {
                    last_used = Some(i);
                    break;
                }
            }

            // Reserve one 0xFF byte as a guard — faxedit/faxiscripts data
            // may depend on a trailing 0xFF terminator.
            let free_start = match last_used {
                Some(i) => cpu_base + (i as u16) + 2, // +1 past data, +1 guard byte
                None => cpu_base, // entire bank is free
            };

            // If cpu_end doesn't land on a bank boundary, there's data above
            // the free region (e.g. title string at $FFE0 in bank 15).
            // Reserve a guard byte before that data too.
            let bank_ceiling = if cpu_base >= 0xC000 { 0x10000u32 } else { 0xC000u32 };
            let effective_end = if (cpu_end as u32) < bank_ceiling {
                cpu_end - 1 // guard byte before upper data
            } else {
                cpu_end
            };

            banks.insert(bank, BankRegion {
                start: free_start,
                next: free_start,
                end: effective_end,
            });
        }

        Self { banks }
    }

    // Allocate `size` bytes in the given bank.
    // Returns the CPU address of the allocated block.
    // Panics if the bank has no free space defined or not enough room.
    pub fn alloc(&mut self, bank: usize, size: usize) -> u16 {
        let region = self.banks.get_mut(&bank).unwrap_or_else(|| {
            panic!("No free space defined for bank {}", bank);
        });
        let addr = region.next;
        let new_next = region.next + size as u16;
        assert!(
            new_next <= region.end,
            "Bank {} free space exhausted: needed {} bytes at ${:04X}, ceiling ${:04X}",
            bank,
            size,
            addr,
            region.end,
        );
        region.next = new_next;
        addr
    }

    // Report remaining free space in a bank.
    pub fn remaining(&self, bank: usize) -> Option<u16> {
        self.banks.get(&bank).map(|r| r.end - r.next)
    }

    /// Print a summary of free space usage for all banks.
    pub fn report(&self) {
        println!();
        println!("╔═════════════════════════════════════════════════════════════╗");
        println!("║                  Free Space Allocation Report               ║");
        println!("╠═══════╦═══════════════════╦═══════════╦═══════════╦═════════╣");
        println!("║ Bank  ║ Range             ║ Available ║ Used      ║ % Used  ║");
        println!("╠═══════╬═══════════════════╬═══════════╬═══════════╬═════════╣");

        let mut sorted_banks: Vec<_> = self.banks.iter().collect();
        sorted_banks.sort_by_key(|(bank, _)| *bank);

        for (&bank, region) in &sorted_banks {
            let total = region.end.saturating_sub(region.start);
            let used = region.next.saturating_sub(region.start);
            let pct = if total > 0 {
                (used as f64 / total as f64) * 100.0
            } else {
                0.0
            };

            println!(
                "║  {:>2}   ║ ${:04X}..${:04X}      ║ {:>5}  B  ║ {:>5}  B  ║ {:>5.1}%  ║",
                bank,
                region.start,
                region.end - 1,
                total,
                used,
                pct,
            );
        }

        println!("╚═══════╩═══════════════════╩═══════════╩═══════════╩═════════╝");
    }
}
