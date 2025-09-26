use crate::consts::mem;
use crate::subroutine::Subroutine;

pub fn build_frame_counter() -> Subroutine {
    let mut frame = Subroutine::new();

    // LDA InterruptCounter → EC
    frame.set_ascii_value(mem::INTERRUPT_COUNTER);
    frame.jsr(0xCA25);
    frame.prepare_ascii();

    // ASCII Position = 0x38, length = 3
    frame.set_ascii_position(0x38);
    frame.set_ascii_length(0x03);

    frame.show_ascii();
    frame.rts();

    frame
}
