use super::*;

use globals::*;


#[utils::opff(FIGHTER_KIND_KEN)]
unsafe fn ken_frame(fighter: &mut L2CFighterCommon) {
    MeterModule::update(fighter.battle_object, true);
}