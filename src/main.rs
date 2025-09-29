#![allow(dead_code, unused_imports)]
mod consts;
mod ips;
mod patches;
mod rom;
mod subroutine;

use crate::ips::write_ips;
use patches::apply_all_hud_patches;
use patches::inventory::crystal::install_crystal_warp_stub;
use patches::inventory::crystal::patch_crystal_warp;
use patches::level_skip::apply_pause_select_warp_patch;
use patches::music::pause::apply_pause_music_patch;
use patches::screens::{eolis, set_start_screen};
use patches::shops::shops::*;
use patches::text::title_screen::patch_title_lines;

use std::env;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 2 {
        eprintln!("Usage: fax_practice_rom <input.nes> <output.nes>");
        std::process::exit(1);
    }

    let mut rom = rom::Rom::from_file(&args[0])?;

    //apply_pause_music_patch(&mut rom);

    use crate::patches::shops::shops::{
        debug_print_shop, write_shop, ShopId, ShopItem, ShopItemId,
    };
    use crate::rom::Rom;

    pub fn patch_eolis_item_shop(rom: &mut Rom) {
        // Replace with Book, Crystal, Wingboots, FireCrystal
        let new_items = vec![
            ShopItem {
                id: ShopItemId::Book,
                price: 0,
            },
            ShopItem {
                id: ShopItemId::Crystal,
                price: 0,
            },
            ShopItem {
                id: ShopItemId::Lamp,
                price: 0,
            },
            ShopItem {
                id: ShopItemId::FireCrystal,
                price: 0,
            },
        ];

        write_shop(rom, ShopId::EolisItemShop, &new_items);

        println!("Eolis Item shop updated:");
        debug_print_shop(rom, ShopId::EolisItemShop);
    }

    //set_starting_gold(&mut rom, 1500);
    install_crystal_warp_stub(&mut rom);
    patch_crystal_warp(&mut rom);

    patch_eolis_item_shop(&mut rom);

    //apply_all_hud_patches(&mut rom);
    apply_pause_select_warp_patch(&mut rom);
    /*patch_title_lines(
        &mut rom,
        "MUSIC PAUSE PATCH",
        "   IT IS PRONOUNCED",
        " FAX-AN-ADOO     v1",
    );*/
    set_start_screen(&mut rom, eolis::EolisScreen::ShopsExterior);

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
