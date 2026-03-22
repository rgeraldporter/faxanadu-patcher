use crate::rom::Rom;

/*
 * Clone Entity 0x1D (Skeleton Knight) Sprite to Entity 0x25
 *
 * This allows two versions of unused enemy sprite 0x1D ("Skeleton Knight")
 * with differing behaviours.
 *
 * In fax-edit, place entity #37 to place this version, and use entrypoint 37
 * in the bscript for behaviour.
 */
pub fn clone_sprite_1d_to_25(rom: &mut Rom) {
    // Copy the sprite update handler pointer from entity 29 to entity 37.
    // SPRITE_UPDATE_HANDLERS at $8087 in Bank 14, 2 bytes per entry (address - 1).
    // SpriteUpdateHandler_Enemy_Unused29 = $9453, stored as $9452 (little-endian, minus 1).
    let off = rom.cpu_to_file_offset(14, 0x80D1); // SPRITE_UPDATE_HANDLERS[37]
    rom.write_bytes(off, &[0x52, 0x94]);

    // Point entity 37 to the same CHR tile data as entity 29 in Bank 6.
    // BANK6_SPRITEADDRS_START at $8002, 2 bytes per entry.
    let off = rom.cpu_to_file_offset(6, 0x804C);  // BANK6_SPRITEADDRS_START[37]
    rom.write_bytes(off, &[0x32, 0x14]);           // same pointer as [29]: $1432

    // Copy the sprite appearance phase offset from entity 29 to entity 37.
    // SPRITE_APPEARANCE_PHASE_OFFSETS at $8C9F in Bank 14, 1 byte per entry.
    let off = rom.cpu_to_file_offset(14, 0x8CC4);  // [37]
    rom.write_bytes(off, &[0x35]);                  // same as [29]

    // Copy the PPU tile count from entity 29 to entity 37.
    // SPRITES_PPU_TILE_COUNTS at $CE1B in Bank 15, 1 byte per entry.
    let off = rom.cpu_to_file_offset(15, 0xCE40);  // [37]
    rom.write_bytes(off, &[0x10]);                  // same as [29]

    // Copy unknown rendering flag from entity 29 to entity 37.
    // Table at $B672 in Bank 14, 1 byte per entry. Entity 29 = $FF, entity 37 = $18.
    let off = rom.cpu_to_file_offset(14, 0xB697);  // [37]
    rom.write_bytes(off, &[0xFF]);                  // same as [29]

    println!("patch: Entity 0x25 (Invisible Stationary #2) now uses Skeleton Knight sprite from entity 0x1D");
}
