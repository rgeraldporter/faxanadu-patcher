use crate::consts::mem;
use crate::subroutine::Subroutine;

pub fn build_level_display() -> Subroutine {
    let mut level = Subroutine::new();
    level.set_ascii_value(mem::CURRENT_LEVEL);
    level.prepare_ascii();
    level.set_ascii_position(0x21);
    level.set_ascii_length(0x01);
    level.show_ascii();
    level.rts();
    level
}
