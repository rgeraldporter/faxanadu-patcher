use crate::consts::mem;
use crate::subroutine::Subroutine;

pub fn build_screen_id() -> Subroutine {
    let mut screen_id = Subroutine::new();
    screen_id.jsr(0xC205);
    screen_id.jsr(0xFD60); // call into level_display routine
    screen_id.set_ascii_value(mem::CURRENT_SCREEN);
    screen_id.prepare_ascii();
    screen_id.set_ascii_position(0x23);
    screen_id.set_ascii_length(0x02);
    screen_id.show_ascii();
    screen_id.rts();
    screen_id
}
