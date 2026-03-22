use crate::allocator::FreeSpaceAllocator;
use crate::consts::{jsr_bytes, jmp_bytes};
use crate::rom::Rom;

/*
 * Add Mana Potion
 *
 * Originally in the unused game code a Black Potion was present. This potion
 * was part of other Dragon Slayer games and used as a means of restoring Karma,
 * which is a player stat that is not part of Faxanadu.
 *
 * This patch uses this potion as refill for the "Magic meter", which is essentially
 * Mana Points, to use Final Fantasy terms.
 *
 * In this current iteration no textbox message appears, unlike the Red Potion.
 *
 * Originally written by Notlob for Randumizer.
 *
 */
/// Returns the CPU address of the handler so other patches can chain to it.
pub fn black_potion_to_mana_potion(rom: &mut Rom, alloc: &mut FreeSpaceAllocator) -> u16 {
    // newcode: consume Mana Potion, show dialog, and fill MP
    let code: [u8; 25] = [
        0xAD, 0xC1, 0x03,           // LDA $03C1        ; a:SelectedItem
        0xC9, 0x11,                 // CMP #$11         ; Black Potion (Mana Potion)?
        0xD0, 0x0E,                 // BNE +14          ; @skip
        0xA9, 0x58,                 // LDA #$58         ; IScript entrypoint 88 (decimal) for dialog
        0x20, 0x59, 0xF8,           // JSR $F859        ; MMC1_LoadBankAndJump
        0x0C,                       //   bank 12
        0x41, 0x82,                 //   IScripts_Begin - 1
        0x20, 0x06, 0xC5,           // JSR $C506        ; @_fillMPLoop (MP fill + sfx)
        0x20, 0xBF, 0xC4,           // JSR $C4BF        ; Player_ClearSelectedItem
                                    // @skip:
        0xAD, 0xC1, 0x03,           // LDA $03C1        ; a:SelectedItem
        0x60,                       // RTS
    ];
    let addr = alloc.alloc(15, code.len());
    let off = rom.cpu_to_file_offset(15, addr);
    rom.write_bytes(off, &code);

    // Hook $C48B — but fire_crystal_screen_damage will overwrite this if it runs later.
    // That's OK as long as fire_crystal_screen_damage chains back to us.
    let off = rom.cpu_to_file_offset(15, 0xC48B);
    rom.write_bytes(off, &jsr_bytes(addr));

    println!("patch: Unimplemented Black Potion can be used as a Mana Potion");
    addr
}

/*
 * Poison to Black Potion
 *
 * In the original game, potion-like icons were used to represent poisons in the overworld.
 * There was also an unimplemented item called "Black Potion". This patch allows you to
 * repurpose poison "pickups" for Black Potion instead.
 *
 * Original code by Notlob for Randumizer
 */
pub fn poison_to_black_potion(rom: &mut Rom) {

    // @todo: just write to C844 and include LDA to be cleaner
    let stub: [u8; 10] = [
        /* 0xA9, */ 0x08,           // /* LDA */ #$08   ; (LDA fallthrough from @_afterFarJump)
        0x20, 0xE4, 0xD0,           // JSR $D0E4        ; Sound_PlayEffect
        0xAE, 0x03, 0x78,           // LDX $7803        ; a:????
        0x60,                       // RTS
        0xEA, 0xEA,                 // NOP NOP          ; probably unnecessary
    ];
    let off = rom.cpu_to_file_offset(15, 0xC845);
    rom.write_bytes(off, &stub);

    println!("patch: Items in game world that were poisons are now Black Potion");
}

/*
 * Fire Crystal as a Screen-Wide Damage Item
 *
 * This adds the previously unused Fire Crystal as a "screenwide damage"
 * item, giving the player an option that is much like Sugata's attack.
 *
 * Presently it does 50 damage, this is adjustable as noted in the code
 * comments.
 *
 * Bank 15: dispatch hook + handler
 * Bank 14: damage loop + death handling
 */
pub fn fire_crystal_screen_damage(rom: &mut Rom, alloc: &mut FreeSpaceAllocator, mana_handler: u16) {
    let b14_code: [u8; 96] = [
        // FireCrystal_DamageAllEnemies:

        // Flash the screen (greyscale on)
        0xA5, 0x0B,                      // LDA $0B          ; PPU_Mask
        0x09, 0x01,                      // ORA #$01         ; set greyscale bit
        0x85, 0x0B,                      // STA $0B          ; PPU_Mask

        // Damage loop
        0xA2, 0x07,                      // LDX #$07         ; 8 sprite slots
                                         // @loop:
        0x8E, 0x78, 0x03,                // STX $0378        ; a:CurrentSpriteIndex
        0xBD, 0xCC, 0x02,                // LDA $02CC,X      ; a:CurrentSprites_Entities
        0x30, 0x37,                      // BMI +55          ; @next (empty slot)
        0xC9, 0xFF,                      // CMP #$FF         ; invalid?
        0xF0, 0x33,                      // BEQ +51          ; @next
        0xA8,                            // TAY              ; Y = entity type
        0xB9, 0x44, 0xB5,                // LDA $B544,Y      ; a:SPRITE_CATEGORIES_BY_ENTITY
        0xF0, 0x04,                      // BEQ +4           ; @isEnemy (category 0 = standard)
        0xC9, 0x07,                      // CMP #$07         ; big enemy?
        0xD0, 0x29,                      // BNE +41          ; @next (not an enemy)
                                         // @isEnemy:
        0xBD, 0x44, 0x03,                // LDA $0344,X      ; a:CurrentSprites_HP
        0x38,                            // SEC
        0xE9, 0x32,                      // SBC #$32         ; -50 damage
        0x9D, 0x44, 0x03,                // STA $0344,X      ; store new HP
        0xB0, 0x1E,                      // BCS +30          ; @next (survived)
                                         // @died:
        0x20, 0x87, 0x8B,                // JSR $8B87        ; Player_AddExperienceFromSprite
        0xA9, 0x03,                      // LDA #$03         ; death sound
        0x20, 0xE4, 0xD0,                // JSR $D0E4        ; Sound_PlayEffect
        0xAE, 0x78, 0x03,                // LDX $0378        ; reload X
        0xBC, 0xCC, 0x02,                // LDY $02CC,X      ; reload entity type
        0xB9, 0x44, 0xB5,                // LDA $B544,Y      ; a:SPRITE_CATEGORIES_BY_ENTITY
        0xC9, 0x07,                      // CMP #$07         ; big enemy?
        0xD0, 0x06,                      // BNE +6           ; @smallDeath
        0x20, 0xEC, 0xAB,                // JSR $ABEC        ; Sprite_ShowBigEnemyDeathByMagicOrWeapon
        0x4C, 0x00, 0x00,                // JMP @next        ; (placeholder, patched below)
                                         // @smallDeath:
        0x20, 0xF6, 0xAB,                // JSR $ABF6        ; Sprite_ShowNormalEnemyDeathByMagic
                                         // @next:
        0xAE, 0x78, 0x03,                // LDX $0378        ; reload X from CurrentSpriteIndex
        0xCA,                            // DEX
        0x10, 0xBB,                      // BPL -69          ; @loop

        // Hold the flash for 2 frames so it's visible
        0x20, 0x25, 0xCA,               // JSR $CA25        ; WaitForNextFrame
        0x20, 0xA8, 0xCB,               // JSR $CBA8        ; Sprites_FlipRanges
        0x20, 0x25, 0xCA,               // JSR $CA25        ; WaitForNextFrame
        0x20, 0xA8, 0xCB,               // JSR $CBA8        ; Sprites_FlipRanges

        // Clear the flash (greyscale off)
        0xA5, 0x0B,                      // LDA $0B          ; PPU_Mask
        0x29, 0xFE,                      // AND #$FE         ; clear greyscale bit
        0x85, 0x0B,                      // STA $0B          ; PPU_Mask

        0x60,                            // RTS
    ];

    let b14_addr = alloc.alloc(14, b14_code.len());
    let off = rom.cpu_to_file_offset(14, b14_addr);
    rom.write_bytes(off, &b14_code);

    // Patch the JMP @next placeholder (offset 65-67 within the block)
    // @next is at offset 71 from the start of the block
    let next_addr = b14_addr + 71;
    let jmp_next = jmp_bytes(next_addr);
    let off = rom.cpu_to_file_offset(14, b14_addr + 65);
    rom.write_bytes(off, &jmp_next);

    let b14_entry_minus1 = b14_addr - 1;
    let b15_code: [u8; 35] = [
        // FireCrystal_DispatchCheck:
        0xAD, 0xC1, 0x03,                // LDA $03C1        ; a:SelectedItem
        0xC9, 0x15,                      // CMP #$15         ; Fire Crystal?
        0xF0, 0x03,                      // BEQ +3           ; @isFireCrystal
        0x4C,                            // JMP mana_handler  ; chain to Mana Potion check
        (mana_handler & 0xFF) as u8,
        (mana_handler >> 8) as u8,
                                         // @isFireCrystal:
        0x68,                            // PLA              ; discard JSR return (low)
        0x68,                            // PLA              ; discard JSR return (high)

        // Player_UseFireCrystal:

        // Show textbox dialog via IScript.
        0xA9, 0x76,                      // LDA #$76         ; IScript ID 118 (change this if you need to!)
        0x20, 0x59, 0xF8,                // JSR $F859        ; MMC1_LoadBankAndJump
        0x0C,                            //   .byte $0C      ;   BANK_12_LOGIC
        0x41, 0x82,                      //   .word $8241    ;   IScripts_Begin-1

        0x20, 0xBF, 0xC4,                // JSR $C4BF        ; Player_ClearSelectedItem
        0xA9, 0x1A,                      // LDA #$1A         ; item use sound
        0x20, 0xE4, 0xD0,                // JSR $D0E4        ; Sound_PlayEffect
        0x20, 0x59, 0xF8,                // JSR $F859        ; MMC1_LoadBankAndJump
        0x0E,                            //   .byte $0E      ;   BANK_14_LOGIC
        (b14_entry_minus1 & 0xFF) as u8, //   .word lo       ;   FireCrystal_DamageAllEnemies-1
        (b14_entry_minus1 >> 8) as u8,   //   .word hi
        0x60,                            // RTS
    ];

    let b15_addr = alloc.alloc(15, b15_code.len());
    let off = rom.cpu_to_file_offset(15, b15_addr);
    rom.write_bytes(off, &b15_code);

    // Patch: replace "LDA a:SelectedItem" at $C48B with JSR to our dispatch check
    let off = rom.cpu_to_file_offset(15, 0xC48B);
    rom.write_bytes(off, &jsr_bytes(b15_addr));

    println!("patch: Fire Crystal can be used (Down+B) to deal 50 damage to all on-screen enemies");
}

/*
 * Crystal as a Warp to Spawn Item
 *
 * This implements the unused Crystal as a "return to spawn" item, much like how it's used
 * in Legacy of the Wizard (Dragon Slayer IV).
 *
 * The player selects the item, then uses it with Down+B. They will appear at their spawn
 * point, just as if they had died except without losing XP or Golds, but also not recovering
 * mana or health. Also adds a sound effect.
 */
pub fn crystal_warp_to_spawn(rom: &mut Rom, alloc: &mut FreeSpaceAllocator) {

    let code: [u8; 31] = [
        // Player_UseCrystal:

        // Show textbox dialog via IScript.
        0xA9, 0x77,                              // LDA #$77         ; IScript ID 119 (change this if you need to!)
        0x20, 0x59, 0xF8,                        // JSR $F859        ; MMC1_LoadBankAndJump
        0x0C,                                    //   .byte $0C      ;   BANK_12_LOGIC
        0x41, 0x82,                              //   .word $8241    ;   IScripts_Begin-1
        0xA9, 0x0A,                              // LDA #$0A         ; SFX Ref
        0x20, 0xE4, 0xD0,                        // JSR $D0E4        ; SFX Execute

        0x20, 0xBF, 0xC4,                        // JSR $C4BF        ; Player_ClearSelectedItem
        0x20, 0x2F, 0xDA,                        // JSR $DA2F        ; Screen_FadeToBlack
        0x20, 0x7D, 0xDA,                        // JSR $DA7D        ; Game_InitStateForSpawn
        0x20, 0x61, 0xDD,                        // JSR $DD61        ; Game_SpawnInTemple
        0x20, 0xAF, 0xDA,                        // JSR $DAAF        ; Game_SetupEnterBuilding
        0x4C, 0x45, 0xDB,                        // JMP $DB45        ; Game_MainLoop
    ];
    let addr = alloc.alloc(15, code.len());
    let off = rom.cpu_to_file_offset(15, addr);
    rom.write_bytes(off, &code);

    // Patch USE_ITEM_JUMP_TABLE[11] (Crystal) to point to our handler.
    // Table entry is at $C49D + (11 * 2) = $C4B3.
    let target = addr - 1;
    let stub: [u8; 2] = [
        (target & 0xFF) as u8,
        (target >> 8) as u8,
    ];
    let off = rom.cpu_to_file_offset(15, 0xC4B3);
    rom.write_bytes(off, &stub);

    println!("patch: Crystal can be used (Down+B) to warp to last Guru spawn point");
}

/*
 * Crystal as Overworld Pickup Item
 *
 * Reuses one of the poison sprite pickups as the Crystal instead.
 *
 * In fax-edit, place entity 0x4C (Poison, index 76) on screens where
 * you want Crystal pickups to appear.
 *
 * Side effects: In order to find space for this sprite, we replace some of the
 * unused Eyeball enemy sprites as we need space in the same bank.
 *
 * Note: The Crystal's use behavior (Down+B to warp) is handled separately by
 * crystal_warp_to_spawn.
 */
pub fn crystal_overworld_pickup(rom: &mut Rom, alloc: &mut FreeSpaceAllocator) {

    let code: [u8; 43] = [
        // CrystalOrPoisonDispatch:
        0xAE, 0x78, 0x03,       // LDX $0378        ; CurrentSpriteIndex
        0xBD, 0xCC, 0x02,       // LDA $02CC,X      ; CurrentSprites_Entities[X]
        0xC9, 0x4C,             // CMP #$4C         ; Poison_2 = Crystal?
        0xF0, 0x0B,             // BEQ +11          ; @crystal

        // --- Poison path ---
        0xA9, 0x91,             // LDA #$91         ; Poison IScript
        0x20, 0x59, 0xF8,       // JSR $F859        ; MMC1_LoadBankAndJump
        0x0C,                   //   .byte $0C      ;   BANK_12_LOGIC
        0x41, 0x82,             //   .word $8241    ;   IScripts_Begin-1
        0x4C, 0x44, 0xC8,       // JMP $C844        ; after IScript (poison_to_black_potion)

        // @crystal:
        0xA9, 0x6C,             // LDA #$6C         ; IScript 108 — Crystal pickup dialog
        0x20, 0x59, 0xF8,       // JSR $F859        ; MMC1_LoadBankAndJump
        0x0C,                   //   .byte $0C      ;   BANK_12_LOGIC
        0x41, 0x82,             //   .word $8241    ;   IScripts_Begin-1
        0xA9, 0x08,             // LDA #$08         ; item pickup SFX
        0x20, 0xE4, 0xD0,       // JSR $D0E4        ; Sound_PlayEffect
        0xA9, 0x0B,             // LDA #$0B         ; ITEMINVENTORY_CRYSTAL
        0x20, 0xCD, 0xC8,       // JSR $C8CD        ; Player_PickUpItem
        0xAE, 0x78, 0x03,       // LDX $0378        ; restore CurrentSpriteIndex
        0x60,                   // RTS
    ];

    let addr = alloc.alloc(15, code.len());
    let off = rom.cpu_to_file_offset(15, addr);
    rom.write_bytes(off, &code);

    // Hook: replace Player_PickUpPoison entry at $C83C with JMP + NOPs.
    // Original bytes at $C83C are the IScript far-call (8 bytes):
    //   LDA #$91 / JSR $F859 / .byte $0C / .word $8241
    // We replace all 8 bytes with JMP <dispatch> + 5× NOP.
    let hook: [u8; 8] = [
        jmp_bytes(addr)[0], jmp_bytes(addr)[1], jmp_bytes(addr)[2],
        0xEA, 0xEA, 0xEA, 0xEA, 0xEA,
    ];
    let off = rom.cpu_to_file_offset(15, 0xC83C);
    rom.write_bytes(off, &hook);

    // ===================================================================
    // Sprite appearance: replace Poison tiles with Crystal icon
    //
    // Entity 0x4C uses BANK7_SPRITEADDRS_START[21] for CHR tile data and
    // SPRITE_APPEARANCE_PHASE_OFFSETS[76] = $E0 for the TILEINFO layout.
    // Entity 0x5E (the other Poison) has separate table entries so changes
    // here only affect 0x4C.
    //
    // We overwrite the unused Eyeball enemy's CHR tiles ($8C96 in bank 7)
    // and TILEINFO ($9AB6) with Crystal graphics, then point 0x4C there.
    // ===================================================================

    // 1. Write Crystal CHR tile data (4 tiles × 16 bytes = 64 bytes)
    //    from the shop icon in PRG10 $8620-$865F (tile indexes $12-$15),
    //    overwriting the unused Eyeball tiles at bank 7 $8C96.
    let crystal_tiles: [u8; 64] = [
        // Tile $12 (top-left)
        0x03, 0x04, 0x08, 0x16, 0x1C, 0x50, 0x40, 0x62,
        0x00, 0x00, 0x00, 0x06, 0x04, 0x40, 0x50, 0x6D,
        // Tile $13 (top-right)
        0xC0, 0x20, 0x10, 0x08, 0x08, 0x0A, 0x1A, 0xB6,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x02, 0x46,
        // Tile $14 (bottom-left)
        0x30, 0x1F, 0x06, 0x01, 0x07, 0x0F, 0x07, 0x00,
        0x72, 0x3F, 0x0E, 0x01, 0x01, 0x08, 0x07, 0x00,
        // Tile $15 (bottom-right)
        0x0C, 0xF8, 0x60, 0x00, 0x20, 0xF0, 0xE0, 0x00,
        0x9C, 0xF8, 0x70, 0x00, 0x00, 0x10, 0xE0, 0x00,
    ];
    let off = rom.cpu_to_file_offset(7, 0x8C96);
    rom.write_bytes(off, &crystal_tiles);

    // 2. Overwrite Eyeball TILEINFO at $9AB6 with 2×2 Crystal layout.
    //    Format: $11 = 2 rows × 2 columns, palette 1 for each tile.
    let crystal_tileinfo: [u8; 12] = [
        0x11,                   // 2 rows, 2 columns
        0x00,                   // X offset = 0
        0x00,                   // Y offset = 0
        0x08,                   // Flipped front-of-entity X = 8
        0x00, 0x01,             // Tile 0, palette 1
        0x01, 0x01,             // Tile 1, palette 1
        0x02, 0x01,             // Tile 2, palette 1
        0x03, 0x01,             // Tile 3, palette 1
    ];
    let off = rom.cpu_to_file_offset(7, 0x9AB6);
    rom.write_bytes(off, &crystal_tileinfo);

    // 3. BANK7_SPRITEADDRS_START[21] → $0C96 (Crystal tiles at $8C96)
    //    Entry at $800C + 21*2 = $8036.
    let off = rom.cpu_to_file_offset(7, 0x8036);
    rom.write_bytes(off, &[0x96, 0x0C]);

    // 4. SPRITE_APPEARANCE_PHASE_OFFSETS[76] → $D7 (Eyeball TILEINFO index)
    //    Entry at $8C9F + 76 = $8CEB.
    let off = rom.cpu_to_file_offset(14, 0x8CEB);
    rom.write_bytes(off, &[0xD7]);

    // 5. SPRITES_PPU_TILE_COUNTS[76] → $04 (4 tiles for 2×2 Crystal)
    //    Entry at $CE1B + 76 = $CE67.
    let off = rom.cpu_to_file_offset(15, 0xCE67);
    rom.write_bytes(off, &[0x04]);

    println!("patch: Entity 0x4C (Poison_2) repurposed as Crystal overworld pickup");
}

/*
 * Fire Crystal as Overworld Pickup Item
 *
 * This repurposes the other Red Potion pickup as a Fire Crystal instead.
 *
 * In fax-edit, place entity 0x4B (Red Potion 2, decimal 75) on screens
 * where you want Fire Crystal pickups to appear. Entity 0x5D remains a
 * normal Red Potion.
 */
pub fn fire_crystal_overworld_pickup(rom: &mut Rom, alloc: &mut FreeSpaceAllocator) {

    let code: [u8; 43] = [
        // FireCrystalOrRedPotionDispatch:
        0xAE, 0x78, 0x03,       // LDX $0378        ; CurrentSpriteIndex
        0xBD, 0xCC, 0x02,       // LDA $02CC,X      ; CurrentSprites_Entities[X]
        0xC9, 0x4B,             // CMP #$4B         ; Red Potion 2 = Fire Crystal?
        0xF0, 0x0B,             // BEQ +11          ; @fireCrystal

        // --- Red Potion path ---
        0xA9, 0x87,             // LDA #$87         ; Red Potion IScript
        0x20, 0x59, 0xF8,       // JSR $F859        ; MMC1_LoadBankAndJump
        0x0C,                   //   .byte $0C      ;   BANK_12_LOGIC
        0x41, 0x82,             //   .word $8241    ;   IScripts_Begin-1
        0x4C, 0x2E, 0xC8,       // JMP $C82E        ; after IScript (original handler)

        // @fireCrystal:
        0xA9, 0x6B,             // LDA #$6B         ; IScript 107 — Fire Crystal dialog
        0x20, 0x59, 0xF8,       // JSR $F859        ; MMC1_LoadBankAndJump
        0x0C,                   //   .byte $0C      ;   BANK_12_LOGIC
        0x41, 0x82,             //   .word $8241    ;   IScripts_Begin-1
        0xA9, 0x08,             // LDA #$08         ; item pickup SFX
        0x20, 0xE4, 0xD0,       // JSR $D0E4        ; Sound_PlayEffect
        0xA9, 0x15,             // LDA #$15         ; ITEMINVENTORY_FIRE_CRYSTAL
        0x20, 0xCD, 0xC8,       // JSR $C8CD        ; Player_PickUpItem
        0xAE, 0x78, 0x03,       // LDX $0378        ; restore CurrentSpriteIndex
        0x60,                   // RTS
    ];

    let addr = alloc.alloc(15, code.len());
    let off = rom.cpu_to_file_offset(15, addr);
    rom.write_bytes(off, &code);

    // Hook: replace Player_PickUpRedPotion entry at $C826 with JMP + NOPs.
    // Original bytes at $C826 are the IScript far-call (8 bytes):
    //   LDA #$87 / JSR $F859 / .byte $0C / .word $8241
    // We replace all 8 bytes with JMP <dispatch> + 5× NOP.
    let hook: [u8; 8] = [
        jmp_bytes(addr)[0], jmp_bytes(addr)[1], jmp_bytes(addr)[2],
        0xEA, 0xEA, 0xEA, 0xEA, 0xEA,
    ];
    let off = rom.cpu_to_file_offset(15, 0xC826);
    rom.write_bytes(off, &hook);

    // ===================================================================
    // Sprite appearance: replace Red Potion 2 tiles with Fire Crystal icon
    //
    // Entity 0x4B (75) uses bank 7 (entity >= 0x37).
    // BANK7_SPRITEADDRS_START[75-55] = BANK7_SPRITEADDRS_START[20].
    //
    // Write Fire Crystal CHR tiles at bank 7 $8CD6 (after Crystal tiles
    // in the unused Eyeball space — Eyeball has 128 bytes, Crystal uses
    // 64, leaving 64 for Fire Crystal).
    //
    // Reuse the same 2×2 TILEINFO at $9AB6 (Eyeball overwrite, shared
    // with Crystal — palette 1, tiles 0-3).
    // ===================================================================

    // 1. Write Fire Crystal CHR tile data (4 tiles × 16 bytes = 64 bytes)
    //    from PRG10 $8900-$893F.
    let fire_crystal_tiles: [u8; 64] = [
        // Tile 1
        0x01, 0x03, 0x07, 0x0E, 0x1D, 0x3B, 0x77, 0x3B,
        0x00, 0x01, 0x03, 0x06, 0x0D, 0x1B, 0x37, 0x1B,

        // Tile 2
        0x80, 0x40, 0x20, 0x10, 0x88, 0xC4, 0xE2, 0xC0,
        0x00, 0x00, 0x00, 0x00, 0x80, 0xC0, 0xE0, 0xDC,

        // Tile 3
        0x1D, 0x0E, 0x07, 0x03, 0x01, 0x16, 0x10, 0x00,
        0x0D, 0x06, 0x03, 0x01, 0x00, 0x0F, 0x0F, 0x00,

        // Tile 4
        0x80, 0x00, 0x00, 0x00, 0x00, 0x98, 0x18, 0x00,
        0xB8, 0x70, 0xE0, 0xC0, 0x80, 0xE0, 0xE0, 0x00,
    ];
    let off = rom.cpu_to_file_offset(7, 0x8CD6);
    rom.write_bytes(off, &fire_crystal_tiles);

    // 2. BANK7_SPRITEADDRS_START[20] → $0CD6 (Fire Crystal tiles at $8CD6)
    //    Entry at $800C + 20*2 = $8034.
    let off = rom.cpu_to_file_offset(7, 0x8034);
    rom.write_bytes(off, &[0xD6, 0x0C]);

    // 3. SPRITE_APPEARANCE_PHASE_OFFSETS[75] → $D7 (Eyeball TILEINFO, same as Crystal)
    //    Entry at $8C9F + 75 = $8CEA.
    let off = rom.cpu_to_file_offset(14, 0x8CEA);
    rom.write_bytes(off, &[0xD7]);

    // 4. SPRITES_PPU_TILE_COUNTS[75] → $04 (4 tiles for 2×2 Fire Crystal)
    //    Entry at $CE1B + 75 = $CE66.
    let off = rom.cpu_to_file_offset(15, 0xCE66);
    rom.write_bytes(off, &[0x04]);

    println!("patch: Entity 0x4B (Red Potion 2) repurposed as Fire Crystal overworld pickup");
}
