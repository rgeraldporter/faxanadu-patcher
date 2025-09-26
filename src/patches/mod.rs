use crate::rom::Rom;

pub mod hud;
pub mod level_skip;
pub mod text;

pub fn apply_all_hud_patches(rom: &mut Rom) {
    // 1) HUD hooks
    hud::substitute_all_jsr(rom);

    // 2) HUD routines
    hud::apply_all_hud_patches(rom);
}
