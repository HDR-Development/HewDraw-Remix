use super::*;
use globals::*;
 

pub fn install() {
    install_status_scripts!(
        snake_side_smash_status_main,
        snake_side_smash_status_end
    );
}

////changed rpg7 side-smash to multi-hit knife
#[status_script(agent = "snake", status = FIGHTER_STATUS_KIND_ATTACK_S4, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn snake_side_smash_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::reset_trigger(fighter.module_accessor);
    original!(fighter)
}
#[status_script(agent = "snake", status = FIGHTER_STATUS_KIND_ATTACK_S4, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn snake_side_smash_status_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    VarModule::off_flag(fighter.object(), vars::snake::instance::KNIFE_COMBO_ENABLE); 
    VarModule::off_flag(fighter.object(), vars::snake::instance::KNIFE_COMBO_IS_BUFFERED); 
    VarModule::set_int(fighter.object(), vars::snake::instance::KNIFE_COMBO_COUNT, 0); 
    original!(fighter)
}