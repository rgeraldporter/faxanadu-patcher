use crate::rom::Rom;

const DIALOG_START_CPU: u16 = 0x8300; // PRG13 message block start
const NEW_DIALOG_CPU: u16 = 0xB3BA; // first free space (extra glyphs area)

pub fn add_dialog_message(rom: &mut Rom, text: &str) -> u8 {
    let bank_base = rom.bank_base(13);
    let start_off = bank_base + usize::from(DIALOG_START_CPU - 0x8000);
    let insert_off = bank_base + usize::from(NEW_DIALOG_CPU - 0x8000);

    // Count existing messages (0xFF terminators) from $8300 up to $B3B9
    let mut term_count: u16 = 0;
    let mut p = start_off;
    while p < insert_off {
        if rom.read_byte(p) == 0xFF {
            term_count += 1;
        }
        p += 1;
    }

    // Write new message at $B3BA
    let mut w = insert_off;
    for ch in text.chars() {
        let b = match ch {
            ' ' => 0xFD,
            '\n' => 0xFE,
            _ => ch as u8,
        };
        rom.write_byte(w, b);
        w += 1;
    }
    rom.write_byte(w, 0xFF);

    // Next available message index = number of existing terminators + 1
    // (disasm’s “Message N” is one higher than the 0-based count we scanned)
    let new_index = (term_count as u8).wrapping_add(1);

    println!(
        "Inserted dialog at ${:04X} (file 0x{:X}), old_count={}, returned index {}",
        NEW_DIALOG_CPU, insert_off, term_count, new_index
    );

    new_index
}
