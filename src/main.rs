mod consts;
mod patches;
mod rom;
mod subroutine;

use patches::apply_all_hud_patches;
use patches::level_skip::apply_pause_select_warp_patch;
use patches::text::title_screen::patch_title_lines;

use std::env;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 2 {
        eprintln!("Usage: fax_practice_rom <input.nes> <output.nes>");
        std::process::exit(1);
    }

    let mut rom = rom::Rom::from_file(&args[0])?;

    apply_all_hud_patches(&mut rom);
    apply_pause_select_warp_patch(&mut rom);
    patch_title_lines(
        &mut rom,
        "MUSIC PAUSE PATCH",
        "   IT IS PRONOUNCED",
        " FAX-AN-ADOO     v1",
    );

    rom.save(&args[1])?;
    println!("Patched ROM written to {}", args[1]);
    Ok(())
}
