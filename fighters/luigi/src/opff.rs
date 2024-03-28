// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn luigi_missle_ledgegrab(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_END) {
        if StatusModule::is_changing(fighter.module_accessor) {
            fighter.select_cliff_hangdata_from_name("special_s");
        }
       // allows ledgegrab during Luigi Missile
    fighter.sub_transition_group_check_air_cliff();
    }
}

unsafe fn luigi_always_misfire_training_mode(fighter: &mut L2CFighterCommon, status_kind: i32) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_ENTRY) && fighter.status_frame() <= 10 {
        super::calculate_misfire_number(fighter);
    }
    if is_training_mode() {
        if status_kind == *FIGHTER_STATUS_KIND_APPEAL && ControlModule::check_button_trigger(fighter.boma(), *CONTROL_PAD_BUTTON_GUARD) { 
            if !VarModule::is_flag(fighter.battle_object, vars::luigi::instance::TRAINING_ALWAYS_MISFIRES) {
                VarModule::on_flag(fighter.battle_object, vars::luigi::instance::TRAINING_ALWAYS_MISFIRES);
                EffectModule::req_on_joint(fighter.module_accessor, Hash40::new("sys_flash"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 0.5, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0);
                
            }
            else {
                VarModule::off_flag(fighter.battle_object, vars::luigi::instance::TRAINING_ALWAYS_MISFIRES);
                VarModule::off_flag(fighter.battle_object, vars::luigi::instance::IS_MISFIRE_STORED);
                EffectModule::req_on_joint(fighter.module_accessor, Hash40::new("sys_flash"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 0.5, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0);
            }
        }
        if VarModule::is_flag(fighter.battle_object, vars::luigi::instance::TRAINING_ALWAYS_MISFIRES) {
        VarModule::on_flag(fighter.battle_object, vars::luigi::instance::IS_MISFIRE_STORED);
        }
    }
}

unsafe fn special_hi_proper_landing(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_LUIGI_STATUS_KIND_SPECIAL_HI_DROP) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            fighter.change_status_req(*FIGHTER_LUIGI_STATUS_KIND_SPECIAL_HI_LANDING_FALL, false);
        }
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_CHARGE,
        *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_END,
        *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_HI_DROP,
        *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_HI_FALL,
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

unsafe fn luigi_missile_edge_cancel(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_END) {
        if fighter.global_table[PREV_SITUATION_KIND] == SITUATION_KIND_GROUND
        && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
            fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL, false);
        }
    }
}

pub unsafe fn moveset(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    luigi_always_misfire_training_mode(fighter, status_kind);
    luigi_missle_ledgegrab(fighter);
    special_s_charge_init(fighter, status_kind);
    special_hi_proper_landing(fighter);
    fastfall_specials(fighter);
    luigi_missile_edge_cancel(fighter);
}

pub extern "C" fn luigi_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		luigi_frame(fighter);
    }
}

pub unsafe fn luigi_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

unsafe fn special_s_charge_init(fighter: &mut smash::lua2cpp::L2CFighterCommon, status_kind: i32) {
    if [*FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_ENTRY].contains(&status_kind)  || !sv_information::is_ready_go() {
        VarModule::off_flag(fighter.object(), vars::luigi::instance::IS_MISFIRE_STORED);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, luigi_frame_wrapper);
}
