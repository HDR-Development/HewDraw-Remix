use super::*;
use globals::*;
// status script import

pub unsafe extern "C" fn end_run(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_Run();
    MotionAnimcmdModule::enable_skip_delay_update(fighter.module_accessor);
    0.into()
}

unsafe extern "C" fn pzenigame_special_s_loop_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_S_LOOP)(fighter);
    DamageModule::set_damage_lock(fighter.module_accessor, false);
    ret
}

unsafe extern "C" fn special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let parent_id = LinkModule::get_parent_id(fighter.module_accessor, *FIGHTER_POKEMON_LINK_NO_PTRAINER, true) as u32;
    let object = utils::util::get_battle_object_from_id(parent_id);
    VarModule::on_flag(object, vars::ptrainer::instance::IS_SWITCH_BACKWARDS); // we will turn this off in opff
    smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_LW)(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(End, *FIGHTER_STATUS_KIND_RUN, end_run);
    agent.status(Main,*FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_S_LOOP,pzenigame_special_s_loop_main);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_main);
}
