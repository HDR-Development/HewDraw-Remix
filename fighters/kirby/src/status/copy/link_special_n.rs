use super::*;

// FIGHTER_KIRBY_STATUS_KIND_LINK_SPECIAL_N

// symbol-based call for link's arrow fuse
extern "Rust" {
    fn fuse_special_n_exit(fighter: &mut L2CFighterCommon);
}

// drops the fused item if the arrow was never fired
unsafe extern "C" fn special_n_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    fuse_special_n_exit(fighter);
    smashline::original_status(Exit, fighter, *FIGHTER_KIRBY_STATUS_KIND_LINK_SPECIAL_N)(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(Exit, *FIGHTER_KIRBY_STATUS_KIND_LINK_SPECIAL_N, special_n_exit);
}
