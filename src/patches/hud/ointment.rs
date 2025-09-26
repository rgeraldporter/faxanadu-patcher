use crate::consts::mem;
use crate::subroutine::Subroutine;

/// Decrement + forward (called from hook)
pub fn build_ointment_decrement() -> Subroutine {
    let mut dec = Subroutine::new();
    dec.add_abs(crate::consts::op::DEC_ABS, mem::DURATION_OINTMENT);
    dec.jsr(0xFD00); // ointment draw
    dec.add_abs(crate::consts::op::LDA_ABS, mem::DURATION_OINTMENT);
    dec.rts();
    dec
}

/// Draw ointment count
pub fn build_ointment_draw() -> Subroutine {
    let mut draw = Subroutine::new();
    draw.set_ascii_value(mem::DURATION_OINTMENT);
    draw.prepare_ascii();
    draw.set_ascii_position(0x78);
    draw.set_ascii_length(0x02);
    draw.show_ascii();
    draw.rts();
    draw
}
