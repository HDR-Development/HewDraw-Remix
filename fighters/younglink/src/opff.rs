// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;


// Young Link Dash Attack Jump
unsafe fn da_jump(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH {
        if situation_kind == *SITUATION_KIND_AIR && !boma.is_in_hitlag() {
            EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_spin_wind_s"), true, true);
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP,true);
            KineticModule::add_speed(boma, &Vector3f::new(0.0, -2.0, 0.0)); //Reduces the jump height from fullhop height
        }
    }
}

// Young Link Fire Arrow drift
unsafe fn fire_arrow_drift(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32, stick_y: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
        if situation_kind == *SITUATION_KIND_AIR {
            if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_FALL {
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
            }
        }
    }
}


extern "Rust" {
    fn gimmick_flash(boma: &mut BattleObjectModuleAccessor);
}


// Bombchu Timer Count
unsafe fn bombchu_timer(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize) {
    let gimmick_timerr = VarModule::get_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER);
    if gimmick_timerr > 0 && gimmick_timerr < 721 {
        // Bombchu Timer Reset
        if gimmick_timerr > 719 {
            VarModule::set_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER, 0);
            gimmick_flash(boma);
        } else {
            VarModule::set_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER, gimmick_timerr + 1);
        }
    }
}

// Bombchu Timer Death Reset
unsafe fn bombchu_reset(fighter: &mut L2CFighterCommon, id: usize, status_kind: i32) {
    if [*FIGHTER_STATUS_KIND_ENTRY,
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_REBIRTH].contains(&status_kind) {
        VarModule::set_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER, 0);
    }
}

// Training Mode Bombchu Timer taunt reset
unsafe fn bombchu_training(fighter: &mut L2CFighterCommon, id: usize, status_kind: i32) {
    if is_training_mode() {
        if status_kind == *FIGHTER_STATUS_KIND_APPEAL {
            VarModule::set_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER, 0);
        }
    }
}

// Lengthen sword
unsafe fn sword_length(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
	let long_sword_scale = Vector3f{x: 1.0, y: 1.1, z: 1.0};
	ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("sword"), &long_sword_scale);
}


unsafe fn holdable_dair(boma: &mut BattleObjectModuleAccessor, motion_kind: u64, frame: f32) {
    // young link dair hold
    if motion_kind == hash40("attack_air_lw")
        && frame > 21.0 && frame < 58.0 
        && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_ATTACK)
    {
        MotionModule::set_frame_sync_anim_cmd(boma, 60.0, true, true, false);
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && ( fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_LINK_STATUS_KIND_SPECIAL_S2,
        *FIGHTER_LINK_STATUS_KIND_SPECIAL_LW_BLAST
        ])
        || (fighter.is_motion(Hash40::new("special_air_hi")) && fighter.motion_frame() > 53.0) )
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
    fire_arrow_drift(fighter, boma, status_kind, situation_kind, cat[1], stick_y);
    bombchu_timer(fighter, boma, id);
    bombchu_reset(fighter, id, status_kind);
    bombchu_training(fighter, id, status_kind);
	sword_length(fighter, boma);
    holdable_dair(boma, motion_kind,frame);
    da_jump(fighter, boma, status_kind, situation_kind);
    fastfall_specials(fighter);
}

// symbol-based call for the links' common opff
extern "Rust" {
    fn links_common(fighter: &mut smash::lua2cpp::L2CFighterCommon);
}


pub extern "C" fn younglink_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		younglink_frame(fighter);
        links_common(fighter);
    }
}

pub unsafe fn younglink_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub fn install() {
    smashline::Agent::new("younglink")
        .on_line(Main, younglink_frame_wrapper)
        .install();
}
