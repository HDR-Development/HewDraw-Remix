use super::*;

unsafe extern "C" fn mewtwo_jump_aerial_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_FLOAT);
    MotionModule::set_trans_move_speed_no_scale(fighter.module_accessor, true);
    fighter.status_JumpAerial();
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_JUMP_AERIAL, mewtwo_jump_aerial_main);
}