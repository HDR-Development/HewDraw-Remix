// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn quickdraw_walljump_leniency(boma: &mut BattleObjectModuleAccessor) {
    if [*FIGHTER_IKE_STATUS_KIND_SPECIAL_S_END].contains(&boma.status())
    && boma.status_frame() < ParamModule::get_int(boma.object(), ParamType::Agent, "param_special_s.end_walljump_valid_frame") {
        boma.check_wall_jump_cancel();
    }
}

unsafe fn quickdraw_instakill(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor){
    if StatusModule::is_changing(boma) {
        return;
    }
    if fighter.is_status(*FIGHTER_IKE_STATUS_KIND_SPECIAL_S_HOLD) && fighter.is_situation(*SITUATION_KIND_GROUND){
        if WorkModule::get_int(boma, *FIGHTER_IKE_STATUS_SPECIAL_S_WORK_INT_CHARGE_COUNT) > 160 {
            // Glow blue when attack is charged enough
            let cbm_vec1 = Vector4f{ /* Red */ x: 0.85, /* Green */ y: 0.85, /* Blue */ z: 0.85, /* Alpha */ w: 0.2}; // Brightness vector
            let cbm_vec2 = Vector4f{ /* Red */ x: 0.125, /* Green */ y: 0.4, /* Blue */ z: 1.0, /* Alpha */ w: 0.45}; // Diffuse vector
            if !VarModule::is_flag(boma.object(), vars::ike::status::SPECIAL_S_INSTAKILL) {
                VarModule::on_flag(boma.object(), vars::ike::status::SPECIAL_S_INSTAKILL);
                EFFECT_FOLLOW(fighter, Hash40::new("ike_volcano_hold"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1.0, false);
                ColorBlendModule::set_main_color(boma, /* Brightness */ &cbm_vec1, /* Diffuse */ &cbm_vec2, 0.21, 2.2, /*Fadein time*/ 30, /* Display Color */ true);
            }
        }
    }
    if fighter.is_status(*FIGHTER_IKE_STATUS_KIND_SPECIAL_S_ATTACK) {
        if VarModule::is_flag(boma.object(), vars::ike::status::SPECIAL_S_INSTAKILL)
        && VarModule::is_flag(boma.object(), vars::ike::status::SPECIAL_S_INSTAKILL_CHECK_HIT) {
            if VarModule::is_flag(boma.object(), vars::ike::status::SPECIAL_S_INSTAKILL_HIT) {
                StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_APPEAL, false);
                let motion = if PostureModule::lr(boma) > 0.0 {
                    Hash40::new("appeal_lw_r")
                }
                else {
                    Hash40::new("appeal_lw_l")
                };
                MotionModule::change_motion(boma, motion, -1.0, 1.0, false, 0.0, false, false);
            }
            VarModule::off_flag(boma.object(), vars::ike::status::SPECIAL_S_INSTAKILL_CHECK_HIT);
            VarModule::off_flag(boma.object(), vars::ike::status::SPECIAL_S_INSTAKILL_HIT);
        }
    }
}

// Need to consolidate the following bone manipulation functions later

// boma: its a boma
// return_frame: frame to start interpolating back to regular angle
// straight_frame: frame the arm bones should be at the regular angle again
unsafe fn quickdraw_attack_arm_bend(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_motion_one_of(&[Hash40::new("special_s_attack"), Hash40::new("special_air_s_attack")])
    && !VarModule::is_flag(boma.object(), vars::ike::status::SPECIAL_S_INSTAKILL) {
        let frame = MotionModule::frame(boma);
        let straight_frame = 0.1;
        if frame <= straight_frame {
            let return_frame = 0.0;
            let end_frame = MotionModule::end_frame(boma);
            let max_x_rotation = 0.0;
            let max_y_rotation = 0.0;
            let max_z_rotation = 75.0;
            let mut rotation = Vector3f{x: 0.0, y: 0.0, z: 0.0};
            // linear interpolate back to normal
            let calc_x_rotate = max_x_rotation *(1.0 - (frame - return_frame) / (straight_frame - return_frame));
            let x_rotation = calc_x_rotate.clamp(0.0, max_x_rotation);
            let calc_y_rotate = max_y_rotation *(1.0 - (frame - return_frame) / (straight_frame - return_frame));
            let y_rotation = calc_y_rotate.clamp(0.0, max_y_rotation);
            let calc_z_rotate = max_z_rotation *(1.0 - (frame - return_frame) / (straight_frame - return_frame));
            let z_rotation = calc_z_rotate.clamp(0.0, max_z_rotation);
            rotation = Vector3f{x: x_rotation, y: y_rotation, z: z_rotation};
            ModelModule::set_joint_rotate(boma, Hash40::new("armr"), &rotation, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
            ModelModule::set_joint_rotate(boma, Hash40::new("handr"), &rotation, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
        }
    }
}

unsafe fn quickdraw_attack_freefall(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_IKE_STATUS_KIND_SPECIAL_S_ATTACK)
    && fighter.is_situation(*SITUATION_KIND_AIR)
    && CancelModule::is_enable_cancel(fighter.module_accessor) {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
        let cancel_module = *(fighter.module_accessor as *mut BattleObjectModuleAccessor as *mut u64).add(0x128 / 8) as *const u64;
        *(((cancel_module as u64) + 0x1c) as *mut bool) = false;  // CancelModule::is_enable_cancel = false
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_LOOP,
        *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_END,
        *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_END_MDL,
        *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_END_MAX,
        *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_HOLD,
        *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_ATTACK,
        *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_END,
        *FIGHTER_IKE_STATUS_KIND_SPECIAL_LW_HIT
        ])
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

pub unsafe fn moveset(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    quickdraw_walljump_leniency(boma);
    quickdraw_instakill(fighter, boma);
    quickdraw_attack_arm_bend(boma);
    quickdraw_attack_freefall(fighter);
    fastfall_specials(fighter);
}

pub extern "C" fn ike_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		ike_frame(fighter)
    }
}

pub unsafe fn ike_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, ike_frame_wrapper);
}
