use super::*;
use globals::*;
 

////changed rpg7 side-smash to multi-hit knife

unsafe extern "C" fn snake_side_smash_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::reset_trigger(fighter.module_accessor);
    smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_ATTACK_S4)(fighter)
}

unsafe extern "C" fn snake_side_smash_status_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    VarModule::off_flag(fighter.object(), vars::snake::instance::KNIFE_COMBO_ENABLE); 
    VarModule::off_flag(fighter.object(), vars::snake::instance::KNIFE_COMBO_IS_BUFFERED); 
    VarModule::set_int(fighter.object(), vars::snake::instance::KNIFE_COMBO_COUNT, 0); 
    smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_ATTACK_S4)(fighter)
}

pub fn install() {
    smashline::Agent::new("snake")
        .status(
            Main,
            *FIGHTER_STATUS_KIND_ATTACK_S4,
            snake_side_smash_status_main,
        )
        .status(
            End,
            *FIGHTER_STATUS_KIND_ATTACK_S4,
            snake_side_smash_status_end,
        )
        .install();
}
