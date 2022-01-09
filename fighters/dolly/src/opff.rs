use super::*;

use globals::*;


#[utils::opff(FIGHTER_KIND_DOLLY)]
unsafe fn dolly_frame(fighter: &mut L2CFighterCommon) {
    MeterModule::update(fighter.battle_object, true);
    println!("Meter: {}", MeterModule::meter(fighter.battle_object));
}