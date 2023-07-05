use super::*;
use globals::*;
// status script import
 
#[status_script(agent = "pzenigame", status = FIGHTER_STATUS_KIND_RUN, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn end_run(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_Run();
    MotionAnimcmdModule::enable_skip_delay_update(fighter.module_accessor);
    0.into()
}

#[status_script(agent = "pzenigame", status = FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_S_LOOP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn pzenigame_special_s_loop_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original!(fighter);
    DamageModule::set_damage_lock(fighter.module_accessor, false);
    ret
}

pub fn install() {
    install_status_scripts!(
        end_run,
        pzenigame_special_s_loop_main
    );
}