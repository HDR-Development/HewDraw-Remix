use super::*;
use globals::*;
// status script import

mod attack_air;
mod catch_attack;
mod throw_kirby;

mod special_hi;

pub const THROW_HI_STATUS_KIND: i32 = 0x47;

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    fighter.global_table[THROW_HI_STATUS_KIND].assign(&FIGHTER_STATUS_KIND_THROW_KIRBY.into());
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);

    attack_air::install(agent);
    catch_attack::install(agent);
    throw_kirby::install(agent);

    special_hi::install(agent);
}
