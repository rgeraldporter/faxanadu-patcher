use crate::subroutine::Subroutine;

pub fn build_player_stats_hook() -> Subroutine {
    let mut hook = Subroutine::new();
    hook.jsr(0xFE60); // jump into mana routine
    hook.rts(); // restore flow
    hook
}
