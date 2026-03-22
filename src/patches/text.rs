use crate::allocator::FreeSpaceAllocator;
use crate::consts::jsr_bytes;
use crate::rom::Rom;

/*
 * Faster Text
 *
 * This speeds up the game text but also has a side-effect of the sound made
 * when text is scrolling being different.
 */
pub fn faster_text(rom: &mut Rom) {
    // Change the AND mask in Maybe_TextBox_ShowCurrentMessageID
    // from #$03 (every 4 frames) to #$01 (every 2 frames)
    // side effect: SFX changes
    let stub: [u8; 1] = [0x00];
    let off = rom.cpu_to_file_offset(15, 0xF49F);
    rom.write_bytes(off, &stub);

    println!("patch: Text rendering speed is doubled");
}

/*
 * Faster Text v2
 *
 * This speeds up the game text without any sfx changes, but does require new code to do so.
 */
pub fn faster_text_v2(rom: &mut Rom, alloc: &mut FreeSpaceAllocator) {
    // Change AND mask from #$03 to #$01 (display char every 2 frames) or #$00 for every frame
    let stub: [u8; 1] = [0x00];
    let off = rom.cpu_to_file_offset(15, 0xF49F);
    rom.write_bytes(off, &stub);

    // newcode: set TextBox_PlayTextSound to 1 when timer bit 1 is clear,
    // or 0 when set — keeping sound at original 4-frame cadence
    let code: [u8; 12] = [
        0xAD, 0x1D, 0x02,       // LDA $021D        ; a:TextBox_Timer
        0x29, 0x02,             // AND #$02         ; isolate bit 1
        0x4A,                   // LSR A            ; 0→0, 2→1
        0x49, 0x01,             // EOR #$01         ; 0→1, 1→0
        0x8D, 0x12, 0x02,       // STA $0212        ; a:TextBox_PlayTextSound
        0x60,                   // RTS
    ];
    let addr = alloc.alloc(15, code.len());
    let off = rom.cpu_to_file_offset(15, addr);
    rom.write_bytes(off, &code);

    // Replace "LDA #$01 / STA TextBox_PlayTextSound" with JSR to new code + NOPs
    let hook: [u8; 5] = [
        jsr_bytes(addr)[0], jsr_bytes(addr)[1], jsr_bytes(addr)[2],
        0xEA, 0xEA,             // NOP NOP
    ];
    let off = rom.cpu_to_file_offset(15, 0xF472);
    rom.write_bytes(off, &hook);

    println!("patch: Text rendering speed is doubled but with original sound cadence");
}
