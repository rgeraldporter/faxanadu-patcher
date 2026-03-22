#![allow(dead_code, unused_imports)]
mod allocator;
mod consts;
mod ips;
mod patches;
mod rom;
mod subroutine;

use crate::allocator::FreeSpaceAllocator;
use crate::ips::write_ips;
use crate::rom::Rom;
use patches::bugfixes::*;
use patches::indoor::*;
use patches::items::*;
use patches::music::apply_pause_music_patch;
use patches::player::*;
use patches::shops::*;
use patches::sprites::*;
use patches::text::*;

use std::env;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 2 {
        eprintln!("Usage: fax_practice_rom <input.nes> <output.nes>");
        std::process::exit(1);
    }

    let mut rom = rom::Rom::from_file(&args[0])?;
    let mut alloc = FreeSpaceAllocator::new(&rom);

    apply_pause_music_patch(&mut rom);
    equip_items_indoors(&mut rom);
    allow_items_indoors(&mut rom);
    draw_weapon_indoors(&mut rom);
    fix_hourglass(&mut rom);
    shield_ointment_fix(&mut rom, &mut alloc);
    let mana_handler = black_potion_to_mana_potion(&mut rom, &mut alloc);
    poison_to_black_potion(&mut rom);
    no_knockback_on_ladders(&mut rom, &mut alloc);
    //faster_text(&mut rom);
    faster_text_v2(&mut rom, &mut alloc);
    allow_lower_respawn(&mut rom);
    allow_all_items_to_be_sold(&mut rom, &mut alloc);
    add_killswitch(&mut rom, &mut alloc);
    ointment_sugata_fix(&mut rom, &mut alloc);
    pendant_quest_25(&mut rom, &mut alloc);
    fire_crystal_screen_damage(&mut rom, &mut alloc, mana_handler);
    crystal_warp_to_spawn(&mut rom, &mut alloc);
    crystal_overworld_pickup(&mut rom, &mut alloc);
    fire_crystal_overworld_pickup(&mut rom, &mut alloc);
    fix_fire_spell_animation(&mut rom);
    fix_studded_mail_climb_tile(&mut rom);
    clone_sprite_1d_to_25(&mut rom);
    allow_menu_on_first_screen(&mut rom);
    //timer_hud_display(&mut rom, &mut alloc); // lag in this!
    //area_name_hud(&mut rom, &mut alloc);
    //fog_palettes(&mut rom, &mut alloc, &[0x0A, 0x0B]);

    // for speedrunning purposes
    /*patch_title_lines(
        &mut rom,
        "MUSIC PAUSE PATCH",
        "   IT IS PRONOUNCED",
        " FAX-AN-ADOO     v1",
    );*/

    alloc.report();

    rom.save(&args[1])?;
    println!("Patched ROM written to {}", args[1]);

    // Optional IPS export
    if args.contains(&"--ips".to_string()) {
        let modified = std::fs::read(&args[1])?;
        let original = std::fs::read(&args[0])?;
        let edits = diff_roms(&original, &modified);
        write_ips("patch_output.ips", &edits)?;
        println!("IPS patch written to patch_output.ips");
    }

    Ok(())
}

fn diff_roms(original: &[u8], modified: &[u8]) -> Vec<(usize, Vec<u8>)> {
    assert_eq!(original.len(), modified.len(), "ROMs must be same size");
    let mut edits = Vec::new();

    let mut i = 0;
    while i < original.len() {
        if original[i] != modified[i] {
            // Start of a changed region
            let start = i;
            let mut bytes = Vec::new();
            while i < original.len() && original[i] != modified[i] {
                bytes.push(modified[i]);
                i += 1;
            }
            edits.push((start, bytes));
        } else {
            i += 1;
        }
    }

    edits
}
