use crate::consts::mem;
use crate::subroutine::Subroutine;

pub fn build_health() -> Subroutine {
    let mut health = Subroutine::new();

    // Save health into memory + staging bytes
    health.add_abs(crate::consts::op::STA_ABS, mem::HEALTH_FULL);
    health.add_abs(crate::consts::op::STA_ABS, mem::BYTE_EC);
    health.add_abs(crate::consts::op::STA_ABS, mem::BYTE_EB);

    health.prepare_ascii();
    health.set_ascii_position(0x26);
    health.set_ascii_length(0x02);
    health.show_ascii();

    // Call fractional draw at $FEA0
    health.jsr(0xFEA0);

    // Restore A from EB
    health.add_abs(crate::consts::op::LDA_ABS, mem::BYTE_EB);

    health.rts();
    health
}
