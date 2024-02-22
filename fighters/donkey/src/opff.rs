// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn dash_attack_jump_cancels(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    if StatusModule::is_changing(boma) {
        return;
    }
    //PM-like neutral-b canceling
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH
    && situation_kind == *SITUATION_KIND_AIR
    && MotionModule::frame(boma) >= 26.0 {
        fighter.check_jump_cancel(false, false);
    }
}

unsafe fn nspecial_cancels(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    //PM-like neutral-b canceling
    if status_kind == *FIGHTER_DONKEY_STATUS_KIND_SPECIAL_N_CANCEL {
        if situation_kind == *SITUATION_KIND_AIR {
            if WorkModule::get_int(boma, *FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE) == *FIGHTER_DONKEY_SPECIAL_N_CANCEL_TYPE_AIR_ESCAPE_AIR {
                WorkModule::set_int(boma, *FIGHTER_DONKEY_SPECIAL_N_CANCEL_TYPE_NONE, *FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
            }
        }
    }
}

extern "Rust" {
    fn gimmick_flash(boma: &mut BattleObjectModuleAccessor);
}

unsafe fn barrel_pull(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    // barrel pull
    if situation_kind == *SITUATION_KIND_GROUND {
        if status_kind == *FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP {
            let lift_rate = match VarModule::is_flag(fighter.object(), vars::donkey::instance::DID_SPAWN_BARREL) {
                true => 1.4,
                false => 2.0
            };
            MotionModule::set_rate(boma, lift_rate);
        } else if [*FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_STATUS_KIND_GUARD_OFF, *FIGHTER_STATUS_KIND_WAIT].contains(&status_kind) {
            if ItemModule::is_have_item(boma, 0)
                && ItemModule::get_have_item_kind(fighter.module_accessor, 0) == *ITEM_KIND_BARREL {
                    fighter.change_status(FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP.into(),false.into());
            }
        }
    } else {
        if status_kind == *FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP {
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_FALL {
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
            }
        }
    }
}

// DK Headbutt aerial stall
unsafe fn headbutt_aerial_stall(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, frame: f32) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
        let motion_value = KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        let motion_vec = Vector3f{x: 1.0, y: 0.0, z: 1.0};
        if situation_kind == *SITUATION_KIND_AIR {
            if  !VarModule::is_flag(boma.object(), vars::common::instance::SPECIAL_STALL_USED) {
                if frame < 26.0 {
                    if motion_value < 0.0 {
                        VarModule::on_flag(boma.object(), vars::common::instance::SPECIAL_STALL);
                        KineticModule::mul_speed(boma, &motion_vec, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                    }
                }
            }
        }
    }
    if VarModule::is_flag(boma.object(), vars::common::instance::SPECIAL_STALL) && (status_kind != *FIGHTER_STATUS_KIND_SPECIAL_S || (status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S && frame >= 26.0)) {
            VarModule::on_flag(boma.object(), vars::common::instance::SPECIAL_STALL_USED);
            VarModule::off_flag(boma.object(), vars::common::instance::SPECIAL_STALL);
    }
    if situation_kind == *SITUATION_KIND_GROUND && VarModule::is_flag(boma.object(), vars::common::instance::SPECIAL_STALL_USED) {
        VarModule::off_flag(boma.object(), vars::common::instance::SPECIAL_STALL_USED);
    }
}

// Down Special Cancels
unsafe fn down_special_cancels(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if [*FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_DONKEY_STATUS_KIND_SPECIAL_LW_LOOP,
        *FIGHTER_DONKEY_STATUS_KIND_SPECIAL_LW_END].contains(&status_kind) {
        if AttackModule::is_infliction(boma, 2) {
            VarModule::on_flag(boma.object(), vars::donkey::status::SPECIAL_CHECKS);
        }
        if VarModule::is_flag(boma.object(), vars::donkey::status::SPECIAL_CHECKS) && frame > 6.0 {
            boma.check_jump_cancel(false, false);
        }
    } else {
        VarModule::off_flag(boma.object(), vars::donkey::status::SPECIAL_CHECKS);
    }
}

/// this sets the ledgegrab box for the backside of up special, which 
/// enables DK to more consistently grab ledge with slipoff uspecial
pub unsafe fn special_hi_slipoff_grab(fighter: &mut L2CFighterCommon) {
    if fighter.is_situation(*SITUATION_KIND_AIR) && fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI) {
        fighter.set_back_cliff_hangdata(20.0, 10.0);
        fighter.set_front_cliff_hangdata(20.0, 10.0);
    }
}

/// make grounded uspecial flat, so that moving forward and back isnt jarring
pub unsafe fn flatten_uspecial(fighter: &mut L2CFighterCommon) {
    if fighter.is_motion(Hash40::new("special_hi")) && fighter.motion_frame() > 16.0 && fighter.motion_frame() < 61.0 {
        // flattens dk out during uspecial
        fighter.set_joint_rotate("rot", Vector3f::new(0.0, 20.0, 50.0));

        // moves dk's trans bone slightly down to compensate for lifted feet during uspecial
        let slightly_lower = Vector3f{x:0.0, y: -4.0, z: 0.0 };
        ModelModule::set_joint_translate(fighter.boma(), Hash40::new("trans"), &slightly_lower, false, false);

        // leans left and right based on movement
        let movement_lean = 20.0 * KineticModule::get_sum_speed_x(fighter.boma(), *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        fighter.set_joint_rotate("trans", Vector3f::new(0.0, movement_lean, 0.0));
    }
}

// prevent donkey kong from carrying/throwing steve's blocks
pub unsafe fn remove_pickelobject(fighter: &mut L2CFighterCommon) {
    if ItemModule::get_have_item_kind(fighter.boma(), 0) == *ITEM_KIND_PICKELOBJECT {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 0, 10, 15, 0, 0, 0, 1, false);
        LAST_EFFECT_SET_COLOR(fighter, 0.6, 0.6, 0.6);
        ItemModule::remove_item(fighter.boma(), 0);
        MotionModule::set_rate(fighter.boma(), 0.1);
        PLAY_SE(fighter, Hash40::new("se_common_famicom_hit"));
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_DONKEY_STATUS_KIND_SPECIAL_N_LOOP,
        *FIGHTER_DONKEY_STATUS_KIND_SPECIAL_N_ATTACK,
        *FIGHTER_DONKEY_STATUS_KIND_SPECIAL_N_END,
        *FIGHTER_DONKEY_STATUS_KIND_SPECIAL_N_CANCEL,
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
        if fighter.is_flag(*FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
            if [*FIGHTER_KINETIC_TYPE_MOTION_AIR, *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE].contains(&KineticModule::get_kinetic_type(fighter.module_accessor)) {
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
                let speed_y = app::sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);

                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y, 0.0, 0.0, 0.0);
                app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
                
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                app::sv_kinetic_energy::enable(fighter.lua_state_agent);

                KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
            }
        }
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    dash_attack_jump_cancels(fighter, boma, status_kind, situation_kind);
    nspecial_cancels(fighter, boma, status_kind, situation_kind);
    barrel_pull(fighter, boma, status_kind, situation_kind);
    headbutt_aerial_stall(fighter, boma, id, status_kind, situation_kind, frame);
    remove_pickelobject(fighter);
    fastfall_specials(fighter);
}

pub extern "C" fn donkey_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		donkey_frame(fighter);
        flatten_uspecial(fighter);
    }
}

pub unsafe fn donkey_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}
pub fn install() {
    smashline::Agent::new("donkey")
        .on_line(Main, donkey_frame_wrapper)
        .install();
}
