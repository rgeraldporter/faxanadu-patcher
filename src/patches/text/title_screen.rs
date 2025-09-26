use crate::patches::text::TitleDict;
use crate::rom::Rom;

/// Patch the 3 title lines stored in PRG bank 12 between $9DCC–$9E0D.
/// Each line is a 0x00-terminated string; lines are packed back-to-back.
pub fn patch_title_lines(rom: &mut Rom, line0: &str, line1: &str, line2: &str) {
    let bank = 12;
    let bank_base_cpu: u16 = 0x8000; // PRG bank base for bank 12
    let start_cpu: u16 = 0x9DCC;
    let end_cpu: u16 = 0x9E0D; // inclusive

    let start_off = rom.bank_base(bank) + (start_cpu - bank_base_cpu) as usize;
    let end_off = rom.bank_base(bank) + (end_cpu - bank_base_cpu) as usize;

    // Discover the three slots (start, capacity_including_terminator)
    let mut slots: Vec<(usize, usize)> = Vec::with_capacity(3);
    let mut cur = start_off;

    while cur <= end_off && slots.len() < 3 {
        // Scan until 0x00 or end
        let mut p = cur;
        while p <= end_off && rom.read_byte(p) != 0x00 {
            p += 1;
        }
        // Capacity is bytes [cur..=p] (including the 0x00 terminator)
        let cap = (p - cur) + 1;
        slots.push((cur, cap));

        // Next slot starts after the terminator
        if p < end_off {
            cur = p + 1;
        } else {
            break;
        }
    }

    // Basic safety: must have at least 3 slots
    if slots.len() < 3 {
        eprintln!(
            "Title region scan found only {} slot(s) – aborting title patch.",
            slots.len()
        );
        return;
    }

    let dict = TitleDict::new();

    // Write each line respecting its slot capacity
    write_title_line(rom, &dict, slots[0], line0);
    write_title_line(rom, &dict, slots[1], line1);
    write_title_line(rom, &dict, slots[2], line2);

    println!(
        "Patched title lines at bank {}: slots caps = [{}, {}, {}] (incl. terminators)",
        bank, slots[0].1, slots[1].1, slots[2].1
    );
}

/// Encode and write one title line into (start_off, cap_incl_term),
/// padding with spaces to (cap-1) and writing a final 0x00.
fn write_title_line(rom: &mut Rom, dict: &TitleDict, slot: (usize, usize), text: &str) {
    let (start, cap_incl_term) = slot;
    // Maximum visible chars we can store in this slot
    let max_chars = cap_incl_term.saturating_sub(1);

    // Prepare string: uppercase, trimmed to max_chars, then pad with spaces to fill the slot
    let mut s = text.to_uppercase();
    if s.len() > max_chars {
        s.truncate(max_chars);
    }
    if s.len() < max_chars {
        s = format!("{:width$}", s, width = max_chars);
    }

    // Encode and write characters
    for (i, ch) in s.chars().enumerate() {
        let b = dict.encode(ch);
        rom.write_byte(start + i, b);
    }
    // Write terminator
    rom.write_byte(start + max_chars, 0x00);
}
