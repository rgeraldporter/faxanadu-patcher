use crate::consts::mem;
use crate::subroutine::Subroutine;

pub fn build_health_frac() -> Subroutine {
    let mut frac = Subroutine::new();
    frac.set_ascii_value(mem::HEALTH_FRAC);
    frac.prepare_ascii();
    frac.set_ascii_position(0x29);
    frac.set_ascii_length(0x03);
    frac.show_ascii();
    frac.rts();
    frac
}
