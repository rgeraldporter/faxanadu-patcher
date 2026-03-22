use crate::rom::Rom;

/*
 * Equip Items indoors
 *
 * In the original game, no items could be equipped while indoors. This removes The
 * restriction.
 */
pub fn equip_items_indoors(rom: &mut Rom) {
    let stub: [u8; 1] = [ 0xFF ];
    let off = rom.cpu_to_file_offset(12, 0x8B87);
    rom.write_bytes(off, &stub);
    println!("patch: Items can be equipped while inside buildings (world 4)");
}

/*
 * Allow Items indoors
 *
 * In the original games, no items could be used indoors. This removes the restriction.
 */
pub fn allow_items_indoors(rom: &mut Rom) {
    // NOP out "STA Player_CurWeapon" in Game_EnterBuilding (Bank 15 $DE06)
    let stub: [u8; 1] = [
        0xAD       // LDA ; essentially a no-op
    ];

    let off = rom.cpu_to_file_offset(15, 0xDE08);
    rom.write_bytes(off, &stub);

    println!("patch: Player character will keep weapon wielded when entering buildings (world 4)");
}

/*
 * Draw Weapon indoors
 *
 * In the original game, weapons were not shown when in a building. This
 * removes that restriction.
 */
pub fn draw_weapon_indoors(rom: &mut Rom) {
    let stub: [u8; 1] = [ 0xFF ];
    let off = rom.cpu_to_file_offset(15, 0xEDF0);
    rom.write_bytes(off, &stub);

    println!("patch: Player character weapon will be wielded immediately upon equipping when in buildings (world 4)");
}
