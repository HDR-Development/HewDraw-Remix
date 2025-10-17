use super::*;
use globals::*;

unsafe extern "C" fn special_s_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, // was NONE
        *FS_SUCCEEDS_KEEP_ATTACK // was 0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S |
            *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S
        ) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn special_s_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(Hash40::new("special_s_end").into(), Hash40::new("special_air_s_end").into(), false.into());
    fighter.sub_set_ground_correct_by_situation(false.into());
    fighter.sub_set_special_start_common_kinetic_setting(hash40("param_special_s").into());

    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
    let speed_x = sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);

    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x.clamp(-1.0, 1.0), 0);

    let main_loop = smashline::api::get_target_function("lua2cpp_elight.nrs", 0x12610).unwrap();
    fighter.sub_shift_status_main(L2CValue::Ptr(main_loop as *const () as _))
}

unsafe extern "C" fn special_s_end_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        let rot = VarModule::get_float(fighter.battle_object, vars::elight::status::SPECIAL_S_ANGLE);
        let frame = fighter.global_table[CURRENT_FRAME].get_f32();
        let lerp = fighter.lerp(rot.into(), 0.0_f32.into(), ((frame - 10.0).clamp(0.0, 20.0) / 20.0).into()).get_f32();
        fighter.set_joint_rotate("rot", Vector3f::new(lerp, 0.0, 0.0));
    }

    0.into()
}

unsafe extern "C" fn special_s_end_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    VisibilityModule::set_whole(fighter.module_accessor, true);

    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_BUNSHIN, ArticleOperationTarget(0));
    }

    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_S_END, special_s_end_pre);
    agent.status(Main, *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_S_END, special_s_end_main);
    agent.status(Exec, *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_S_END, special_s_end_exec);
    agent.status(ExecStop, *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_S_END, special_s_end_exec);
    agent.status(End, *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_S_END, special_s_end_end);
}