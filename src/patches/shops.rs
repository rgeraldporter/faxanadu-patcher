use crate::allocator::FreeSpaceAllocator;
use crate::consts::jmp_bytes;
use crate::rom::Rom;

/*
 * Allow All Items to be Sold
 *
 * In the original game a vendor would have to be selling an item themselves to want
 * to buy any back from the player. This makes it so that all vendors will buy back
 * anything, but if that vendor does not sell the given item, they will only give the
 * player 100 golds.
 *
 * Original code by Notlob for Randumizer
 */
pub fn allow_all_items_to_be_sold(rom: &mut Rom, alloc: &mut FreeSpaceAllocator) {
    let code: [u8; 30] = [
        0x8A,                   // TXA              ; A = item ID (original X from $8690)
        0x48,                   // PHA              ; save item ID on stack
        0x20, 0x04, 0x87,       // JSR $8704        ; IScripts_SellMenu_Something8704
        0xC9, 0xFF,             // CMP #$FF
        0xF0, 0x04,             // BEQ +4           ; @skip
        0x68,                   // PLA              ; discard saved ID (normal path)
        0x4C, 0x96, 0x86,       // JMP $8696        ; a:[Within @LAB_PRG12__8689]
                                // @skip:
        0x68,                   // PLA              ; A = original item ID
        0xAE, 0x1F, 0x02,       // LDX $021F        ; Arg_StringsCount
        0x9D, 0x20, 0x02,       // STA $0220,X      ; DataArray,X = item ID
        0xA9, 0x64,             // LDA #$64         ; 100 golds
        0x9D, 0x28, 0x02,       // STA $0228,X      ; ShopItemCostsL,X
        0xA9, 0x00,             // LDA #$00
        0x4C, 0xA9, 0x86,       // JMP $86A9        ; ShopItemCostsU + increment count
    ];
    let addr = alloc.alloc(12, code.len());
    let off = rom.cpu_to_file_offset(12, addr);
    rom.write_bytes(off, &code);

    // overwrite part of `@LAB_PRG12__8689` with JMP to newcode
    let off = rom.cpu_to_file_offset(12, 0x8691);
    rom.write_bytes(off, &jmp_bytes(addr));

    println!("patch: All items that normally cannot be sold to a shop are sellable for 100 golds")
}
