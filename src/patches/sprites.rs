use crate::rom::Rom;

const SOURCE_ENTITY: u16 = 29;  // 0x1D, Skeleton Knight
const TARGET_ENTITY: u16 = 37;  // 0x25, Invisible Stationary #2

// Per-entity table base addresses
const SPRITE_UPDATE_HANDLERS: u16 = 0x8087;           // Bank 14, 2 bytes per entry
const BANK6_SPRITEADDRS_START: u16 = 0x8002;           // Bank 6, 2 bytes per entry
const SPRITE_APPEARANCE_PHASE_OFFSETS: u16 = 0x8C9F;   // Bank 14, 1 byte per entry
const SPRITES_PPU_TILE_COUNTS: u16 = 0xCE1B;           // Bank 15, 1 byte per entry
const SPRITE_RENDERING_FLAGS: u16 = 0xB672;            // Bank 14, 1 byte per entry

/*
 * Clone Entity 0x1D (Skeleton Knight) Sprite to Entity 0x25
 *
 * This allows two versions of unused enemy sprite 0x1D ("Skeleton Knight")
 * with differing behaviours.
 *
 * In faxedit, place entity #37 to place this version, and use entrypoint 37
 * in the bscript for behaviour.
 *
 * All values are read dynamically from the source entity so that faxedit
 * can reorganize sprite data without breaking this patch.
 *
 * This is compatible with sprite patching in faxedit beta 6+!
 */
pub fn clone_sprite_1d_to_25(rom: &mut Rom) {
    // 1. Copy sprite update handler pointer (Bank 14, 2 bytes per entry).
    let src_off = rom.cpu_to_file_offset(14, SPRITE_UPDATE_HANDLERS + SOURCE_ENTITY * 2);
    let handler = rom.read_word(src_off);
    let dst_off = rom.cpu_to_file_offset(14, SPRITE_UPDATE_HANDLERS + TARGET_ENTITY * 2);
    rom.write_bytes(dst_off, &handler.to_le_bytes());

    // 2. Copy CHR tile data pointer (Bank 6, 2 bytes per entry).
    let src_off = rom.cpu_to_file_offset(6, BANK6_SPRITEADDRS_START + SOURCE_ENTITY * 2);
    let chr_ptr = rom.read_word(src_off);
    let dst_off = rom.cpu_to_file_offset(6, BANK6_SPRITEADDRS_START + TARGET_ENTITY * 2);
    rom.write_bytes(dst_off, &chr_ptr.to_le_bytes());

    // 3. Copy sprite appearance phase offset (Bank 14, 1 byte per entry).
    let src_off = rom.cpu_to_file_offset(14, SPRITE_APPEARANCE_PHASE_OFFSETS + SOURCE_ENTITY);
    let phase = rom.read_byte(src_off);
    let dst_off = rom.cpu_to_file_offset(14, SPRITE_APPEARANCE_PHASE_OFFSETS + TARGET_ENTITY);
    rom.write_bytes(dst_off, &[phase]);

    // 4. Copy PPU tile count (Bank 15, 1 byte per entry).
    let src_off = rom.cpu_to_file_offset(15, SPRITES_PPU_TILE_COUNTS + SOURCE_ENTITY);
    let tile_count = rom.read_byte(src_off);
    let dst_off = rom.cpu_to_file_offset(15, SPRITES_PPU_TILE_COUNTS + TARGET_ENTITY);
    rom.write_bytes(dst_off, &[tile_count]);

    // 5. Copy rendering flag (Bank 14, 1 byte per entry).
    let src_off = rom.cpu_to_file_offset(14, SPRITE_RENDERING_FLAGS + SOURCE_ENTITY);
    let flag = rom.read_byte(src_off);
    let dst_off = rom.cpu_to_file_offset(14, SPRITE_RENDERING_FLAGS + TARGET_ENTITY);
    rom.write_bytes(dst_off, &[flag]);

    println!("patch: Entity 0x25 (Invisible Stationary #2) now uses Skeleton Knight sprite from entity 0x1D");
}
