use super::*;
use globals::*;

 
pub fn install() {
    install_status_scripts!(
        jump_end
    );
}

// FIGHTER_STATUS_KIND_JUMP_AERIAL //

#[status_script(agent = "bayonetta", status = FIGHTER_STATUS_KIND_JUMP_AERIAL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn jump_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_JumpAerial();
    let frame = fighter.global_table[CURRENT_FRAME].get_i32() as f32;
    if frame <= fighter.get_param_float("param_special_hi", "jump_count_reset_frame") {
        if [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_S].contains(&fighter.global_table[STATUS_KIND].get_i32()) {
            fighter.set_int(1, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
            EFFECT(fighter, Hash40::new("bayonetta_witchtime_flash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
            PostureModule::set_pos(fighter.module_accessor, &Vector3f{x: PostureModule::pos_x(fighter.module_accessor), y: PostureModule::pos_y(fighter.module_accessor) - (frame * 1.2), z: PostureModule::pos_z(fighter.module_accessor)});
        }
        else if fighter.global_table[globals::STATUS_KIND] == FIGHTER_STATUS_KIND_ATTACK_AIR {
            EffectModule::req_follow(fighter.module_accessor, Hash40::new("bayonetta_feather_twinkle"), Hash40::new("waist"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &Vector3f::zero(), 0.8, true, 0, 0, 0, 0, 0, false, false);
        }
    }
    0.into()
}