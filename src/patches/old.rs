/*
 *
 * OLDER STUFF I NEED TO MIGRATE TO NEW METHODS STILL
 */

use crate::allocator::FreeSpaceAllocator;
use crate::rom::Rom;

/*
 * Item: CRYSTAL
 * Implementation: Player equips item and presses Down + B. The will advance one
 *  world forward, but only to worlds that they have acquired the main quest item from.
 *  Once the highest world possible for them is reached, it will send them back to Eolis.
 *  This item is non-consumable but need to be equiped to use.
 *
 * Quest items required:
 *  Eolis (0): None
 *  Trunk (1): Ring of Ruby
 *  Mist (2): Black Onyx
 *  Branch (3): Ring of Dworf
 *  Dartmoor (4): Demon's Ring
 *
 * Currently you cannot warp into the Evil One's lair with this.
 */

// Patch the Crystal (0x8B) so it's usable with Down+B.
// `warp_stub_addr` is the CPU address returned by `install_crystal_warp_stub`.
pub fn patch_crystal_warp(rom: &mut Rom, warp_stub_addr: u16) {
    let bank = 14;
    let jump_table_cpu = 0xC49D;
    let base = rom.bank_base(bank);

    let table_off = base + (jump_table_cpu - 0x8000) as usize;

    // Crystal is item ID 0x8B
    // Table starts at 0x80, 2 bytes each.
    let index = 0x8B - 0x80;
    let entry_off = table_off + index as usize * 2;

    // The bank switch process works by jumping to one byte before the instruction
    // then naturally advancing, so we subtract 1 to reach our target.
    let target = warp_stub_addr - 1;
    let lo = (target & 0xFF) as u8;
    let hi = (target >> 8) as u8;

    rom.write_byte(entry_off, lo);
    rom.write_byte(entry_off + 1, hi);

    println!(
        "Patched Crystal (0x8B) use-table entry at 0x{:X} -> ${:04X} (warp up)",
        entry_off, target
    );
}

pub fn install_crystal_warp_stub(rom: &mut Rom, alloc: &mut FreeSpaceAllocator) -> u16 {
    // Internal jump targets (offsets from start of stub):
    //   LoopStart = +20  (was $FE7C when base was $FE68)
    //   SaveArea  = +105 (was $FED1 when base was $FE68)
    const LOOP_START_OFF: u16 = 20;
    const SAVE_AREA_OFF: u16 = 105;

    let mut stub: [u8; 111] = [
        0xA9, 0x0A,                     // LDA #$0A         ; SFX Ref
        0x20, 0xE4, 0xD0,               // JSR $D0E4        ; SFX Execute
        0xAD, 0x2C, 0x04,               // LDA $042C        ; Get the special items
        0x29, 0x71,                     // AND #%01110001   ; Check Ruby, Onyx, Dworf, Demon
        0xD0, 0x08,                     // BNE              ; If we have none of these, go to Eolis (0)
        0xA9, 0x00,                     // LDA #$00
        0x8D, 0x35, 0x04,               // STA $0435        ; Save the area
        0x4C, 0x00, 0x00,               // JMP SaveArea     ; (patched below)
                            // --- LoopStart (offset 20)
        0xEE, 0x35, 0x04,               // INC $0435        ; increment area by 1
        0xAD, 0x35, 0x04,               // LDA $0435
        0xC9, 0x05,                     // CMP #$05
        0x90, 0x08,                     // BCC              ; NotWrap
        0xA9, 0x00,                     // LDA #$00         ; Go to Eolis
        0x8D, 0x35, 0x04,               // STA $0435        ; Save the area
        0x4C, 0x00, 0x00,               // JMP              ; SaveArea (patched below)
        0xA8,                           // TAY              ; Load Area_Region into Y
                            // --- AREA 1 Trunk
        0xC0, 0x01,                     // CPY #$01         ; Is this area 1
        0xD0, 0x0B,                     // BNE              ; goto next (2)
        0xAD, 0x2C, 0x04,               // LDA $042C        ; Get the special items
        0x29, 0x40,                     // AND #$40         ; Do we have Ring of Ruby?
        0xD0, 0x03,                     // BNE
        0x4C, 0x00, 0x00,               // JMP LoopStart    ; (patched below)
        0x4C, 0x00, 0x00,               // JMP SaveArea     ; (patched below)
                            // --- AREA 2 Mist
        0xC0, 0x02,                     // CPY #$02         ; Is this area 2
        0xD0, 0x0B,                     // BNE              ; goto next (3)
        0xAD, 0x2C, 0x04,               // LDA $042C        ; Get the special items
        0x29, 0x01,                     // AND #$01         ; Do we have Black Onyx?
        0xD0, 0x03,                     // BNE
        0x4C, 0x00, 0x00,               // JMP              ; LoopStart (patched below)
        0x4C, 0x00, 0x00,               // JMP SaveArea     ; (patched below)
                            // --- AREA 3 Branch
        0xC0, 0x03,                     // CPY #$03         ; Is this area 3
        0xD0, 0x0B,                     // BNE              ; goto next (4)
        0xAD, 0x2C, 0x04,               // LDA $042C        ; Get the special items
        0x29, 0x20,                     // AND #$20         ; Do we have Ring of Dworf?
        0xD0, 0x03,                     // BNE
        0x4C, 0x00, 0x00,               // JMP LoopStart    ; (patched below)
        0x4C, 0x00, 0x00,               // JMP SaveArea     ; (patched below)
                            // --- AREA 4 Dartmoor
        0xC0, 0x04,                     // CPY #$04         ; Is this area 4
        0xD0, 0x0B,                     // BNE              ; goto next (0)
        0xAD, 0x2C, 0x04,               // LDA $042C        ; Get the special items
        0x29, 0x10,                     // AND #$10         ; Do we have Demon's Ring?
        0xD0, 0x03,                     // BNE
        0x4C, 0x00, 0x00,               // JMP LoopStart    ; (patched below)
                            // --- Save area and trigger warp (offset 104/105)
        0x8C, 0x35, 0x04,               // STY $0435        ; Save the area
                            // --- JSR to area load subroutine
        0x20, 0xDC, 0xDA,               // JSR $DADC
        0x60,                           // RTS
    ];

    let addr = alloc.alloc(15, stub.len());
    let loop_start = addr + LOOP_START_OFF;
    let save_area = addr + SAVE_AREA_OFF;

    // Helper to write a 16-bit LE address into the stub at a given offset
    let patch_addr = |stub: &mut [u8], off: usize, target: u16| {
        stub[off] = (target & 0xFF) as u8;
        stub[off + 1] = (target >> 8) as u8;
    };

    // Patch all JMP SaveArea targets (byte after JMP opcode)
    patch_addr(&mut stub, 18, save_area);
    patch_addr(&mut stub, 36, save_area);
    patch_addr(&mut stub, 54, save_area);
    patch_addr(&mut stub, 71, save_area);
    patch_addr(&mut stub, 88, save_area);

    // Patch all JMP LoopStart targets
    patch_addr(&mut stub, 51, loop_start);
    patch_addr(&mut stub, 68, loop_start);
    patch_addr(&mut stub, 85, loop_start);
    patch_addr(&mut stub, 102, loop_start);

    let off = rom.cpu_to_file_offset(15, addr);
    rom.write_slice(off, &stub);

    println!("Installed Crystal warp stub at ${:04X}", addr);
    addr
}

