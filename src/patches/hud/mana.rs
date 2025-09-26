use crate::consts::mem;
use crate::subroutine::Subroutine;

pub fn build_mana() -> Subroutine {
    let mut mana = Subroutine::new();

    // Save mana → staging
    mana.add_abs(crate::consts::op::STA_ABS, mem::MANA_POINTS);
    mana.add_abs(crate::consts::op::STA_ABS, mem::BYTE_EC);
    mana.add_abs(crate::consts::op::STA_ABS, mem::BYTE_EB);

    mana.prepare_ascii();
    mana.set_ascii_position(0x2D);
    mana.add_abs(crate::consts::op::STY_ABS, mem::BYTE_EA);
    mana.set_ascii_length(0x02);
    mana.show_ascii();

    // Restore Y, A
    mana.add_abs(0xAC, mem::BYTE_EA); // LDY abs
    mana.add_abs(crate::consts::op::LDA_ABS, mem::BYTE_EB);

    mana.rts();
    mana
}
