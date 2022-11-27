// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;
 
unsafe fn teleport_tech(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, frame: f32) {
    if boma.is_status(*FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_2) {
        if compare_mask(ControlModule::get_pad_flag(boma), *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER) {
            VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
            boma.change_status_req(*FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_3, false);
            ControlModule::clear_command(boma, false);
        }
    }

    // Wall Ride momentum fixes
    let mut wall_ride = Vector3f{x: 1.0, y: 1.0, z: 1.0};
    let touch_right = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_RIGHT_SIDE as u32);
    let touch_left = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_LEFT_SIDE as u32);
    let warp_speed = WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("wrap_speed_add")) + WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("wrap_speed_multi"));

    if boma.is_status(*FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_2) {
        // Manipulate ECB for landing purposes
        //GroundModule::set_rhombus_offset(boma, &Vector2f::new(0.0, 0.0));
        if touch_right || touch_left || VarModule::is_flag(boma.object(), vars::common::instance::IS_TELEPORT_WALL_RIDE) {
            VarModule::on_flag(boma.object(), vars::common::instance::IS_TELEPORT_WALL_RIDE);
            if (touch_right && KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) < 0.0) || (touch_left && KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) > 0.0) {
                let rise_speed = KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                if rise_speed > 0.0 {
                    wall_ride = Vector3f{x: 0.0, y: (warp_speed / rise_speed), z: 1.0};
                }
                else {
                    wall_ride = Vector3f{x: 0.0, y: 1.0, z: 1.0};
                }
                KineticModule::mul_speed(boma, &wall_ride, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            }
        }
    }
    else if boma.is_status(*FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_3) {
        // Manipulate ECB for landing purposes
        //GroundModule::set_rhombus_offset(boma, &Vector2f::new(0.0, 0.0));
        if touch_right || touch_left {
            if (touch_right && KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) < 0.0) || (touch_left && KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) > 0.0) {
                wall_ride = Vector3f{x: 0.0, y: 1.0, z: 1.0};
                KineticModule::mul_speed(boma, &wall_ride, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            }
        }
    }
    else {
        VarModule::off_flag(boma.object(), vars::common::instance::IS_TELEPORT_WALL_RIDE);
    }
}

unsafe fn phantom_special_cancel(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    if fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_ATTACK,
                                        *FIGHTER_STATUS_KIND_ATTACK_S3,
                                        *FIGHTER_STATUS_KIND_ATTACK_HI3,
                                        *FIGHTER_STATUS_KIND_ATTACK_LW3,
                                        *FIGHTER_STATUS_KIND_ATTACK_S4,
                                        *FIGHTER_STATUS_KIND_ATTACK_HI4,
                                        *FIGHTER_STATUS_KIND_ATTACK_LW4,
                                        *FIGHTER_STATUS_KIND_ATTACK_DASH,
                                        *FIGHTER_STATUS_KIND_ATTACK_AIR])
    && (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD))
    && !fighter.is_in_hitlag() {
        if fighter.is_cat_flag(Cat1::SpecialLw) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, false);
        }
    }
}

// unsafe fn teleport_startup_ledgegrab(fighter: &mut L2CFighterCommon) {
//     if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI) {
//         // allows ledgegrab during teleport startup
//         fighter.sub_transition_group_check_air_cliff();
//     }
// }

unsafe fn nayru_fastfall_land_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32, stick_y: f32, frame: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
        if situation_kind == *SITUATION_KIND_GROUND {
            if StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR && frame < 54.0 {
                //StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, false);
                WorkModule::on_flag(boma, *FIGHTER_ZELDA_STATUS_SPECIAL_N_FLAG_REFLECTOR_END);
                MotionModule::set_frame_sync_anim_cmd(boma, 56.0, true, true, false);
            }
        }
        else if situation_kind == *SITUATION_KIND_AIR {
            if frame >= 30.0 {
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
                if boma.is_cat_flag(Cat2::FallJump) && stick_y < -0.66 && KineticModule::get_sum_speed_y(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
                    WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
                }
            }
        }
    }
}

/// Handles land canceling when airborne
unsafe fn dins_fire_cancels(boma: &mut BattleObjectModuleAccessor){
    if boma.is_status(*FIGHTER_ZELDA_STATUS_KIND_SPECIAL_S_END) {
        if boma.is_situation(*SITUATION_KIND_GROUND) {
            if StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR {
                boma.change_status_req(*FIGHTER_STATUS_KIND_LANDING, false);
            }
        }
    }
}

/// Reset use of Din's Fire on stock loss or match end
unsafe fn dins_flag_reset(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_WIN,
        *FIGHTER_STATUS_KIND_LOSE,
        *FIGHTER_STATUS_KIND_ENTRY,
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_REBIRTH]) || !sv_information::is_ready_go() {
        VarModule::off_flag(boma.object(), vars::zelda::instance::DEIN_ACTIVE);
    }
}

pub unsafe fn moveset(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    teleport_tech(fighter, boma, frame);
    dins_fire_cancels(boma);
    dins_flag_reset(boma);
    nayru_fastfall_land_cancel(boma, status_kind, situation_kind, cat[2], stick_y, frame);
    phantom_special_cancel(fighter, boma);
}

#[utils::macros::opff(FIGHTER_KIND_ZELDA )]
pub fn zelda_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		zelda_frame(fighter)
    }
}

pub unsafe fn zelda_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

#[smashline::weapon_frame_callback]
pub fn phantom_callback(weapon: &mut smash::lua2cpp::L2CFighterBase) {
    unsafe { 
        if weapon.kind() != WEAPON_KIND_ZELDA_PHANTOM {
            return
        }
        GroundModule::correct(weapon.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        if weapon.is_status(*WEAPON_ZELDA_PHANTOM_STATUS_KIND_BUILD) {
            let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
            let zelda = utils::util::get_battle_object_from_id(owner_id);
            let zelda_boma = &mut *(*zelda).module_accessor;
            let remaining_hitstun = WorkModule::get_float(zelda_boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME);
            if zelda_boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_GUARD,
                                            *FIGHTER_STATUS_KIND_ESCAPE,
                                            *FIGHTER_STATUS_KIND_ESCAPE,
                                            *FIGHTER_STATUS_KIND_ESCAPE_F,
                                            *FIGHTER_STATUS_KIND_ESCAPE_B,
                                            *FIGHTER_STATUS_KIND_ESCAPE_AIR,
                                            *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE,
                                            *FIGHTER_STATUS_KIND_CATCH,
                                            *FIGHTER_STATUS_KIND_CATCH_DASH,
                                            *FIGHTER_STATUS_KIND_CATCH_TURN,
                                            *FIGHTER_STATUS_KIND_CATCH_PULL,
                                            *FIGHTER_STATUS_KIND_CATCH_WAIT,
                                            *FIGHTER_STATUS_KIND_CATCH_ATTACK,
                                            *FIGHTER_STATUS_KIND_CATCH_CUT,
                                            *FIGHTER_STATUS_KIND_SHOULDERED_DONKEY,
                                            *FIGHTER_STATUS_KIND_CATCHED_RIDLEY,
                                            *FIGHTER_STATUS_KIND_CATCHED_REFLET,
                                            *FIGHTER_STATUS_KIND_CATCHED_GANON,
                                            *FIGHTER_STATUS_KIND_CATCHED_AIR_GANON,
                                            *FIGHTER_STATUS_KIND_CATCHED_CUT_GANON,
                                            *FIGHTER_STATUS_KIND_DAMAGE,
                                            *FIGHTER_STATUS_KIND_DAMAGE_AIR,
                                            *FIGHTER_STATUS_KIND_DAMAGE_FALL,
                                            *FIGHTER_STATUS_KIND_DAMAGE_FLY,
                                            *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
                                            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
                                            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
                                            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
                                            *FIGHTER_STATUS_KIND_DAMAGE_FALL,
                                            *FIGHTER_STATUS_KIND_SPECIAL_LW,
                                            *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_LW_CHARGE,
                                            *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_LW_END,
                                            *FIGHTER_STATUS_KIND_THROW])
            || WorkModule::is_flag(zelda_boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_AIR)
            || WorkModule::is_flag(zelda_boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_GROUND)
            || (remaining_hitstun > 0.0){
                return
            }

            if AttackModule::is_infliction_status(zelda_boma, *COLLISION_KIND_MASK_HIT)
            && !AttackModule::is_infliction_status(zelda_boma, *COLLISION_KIND_MASK_SHIELD)
            && zelda_boma.is_cat_flag(Cat1::SpecialLw) {
                StatusModule::change_status_force(weapon.module_accessor, *WEAPON_ZELDA_PHANTOM_STATUS_KIND_ATTACK, false);
            }
        }
    }
}