// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn wings_of_rebellion_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if status_kind == *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI2_RUSH {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI2_END, true);
        }
        if boma.get_num_used_jumps() < boma.get_jump_count_max() {
            if boma.get_aerial() != None {
                WorkModule::inc_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
                VarModule::on_flag(boma.object(), vars::common::instance::UP_SPECIAL_CANCEL);
                boma.change_status_req(*FIGHTER_STATUS_KIND_ATTACK_AIR, false);
            }
        }
        
    }
}

unsafe fn arsene_summon_desmummon(boma: &mut BattleObjectModuleAccessor) {

    //if boma.is_motion_one_of(&[Hash40::new("special_lw_start"), Hash40::new("special_lw_loop")]) {
    if boma.is_status(*FIGHTER_STATUS_KIND_ATTACK_AIR){
        if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_GUARD) {
            let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
            //DamageModule::add_damage(boma, 1.0, 0);
            ArticleModule::generate_article(boma, *FIGHTER_JACK_GENERATE_ARTICLE_DOYLE, false, 0);
            WorkModule::on_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE);
            WorkModule::on_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_RESERVE_SUMMON_DISPATCH);
            WorkModule::on_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE_EXIST);
            //WorkModule::on_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_RESERVE_SUMMON_DISPATCH);
            //boma.change_status_req(*FIGHTER_JACK_STATUS_KIND_SUMMON, false);
            //smash::app::FighterSpecializer_Jack::add_rebel_gauge(boma, app::FighterEntryID(entry_id), 100.0);
        }
    }
    
}

// Joker Arsene Grappling Hook
unsafe fn arsene_grappling_hook(boma: &mut BattleObjectModuleAccessor, situation_kind: i32, motion_kind: u64) {
    if motion_kind == hash40("special_hi_start") {//&& situation_kind == *SITUATION_KIND_GROUND {
        MotionModule::change_motion_kind(boma, smash::phx::Hash40::new("special_hi"));
    }
}

// Joker Aerial Grappling Hook stall
unsafe fn aerial_grappling_hook_stall(boma: &mut BattleObjectModuleAccessor, motion_kind: u64, frame: f32) {
    if motion_kind == hash40("special_air_hi_throw") {
        if frame < 37.0 {
            KineticModule::unable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
        if frame >= 37.0 {
            KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
    }
}

// Joker Grappling Hook Spike Cancel
unsafe fn grappling_hook_spike_cancel (fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI) && fighter.is_situation(*SITUATION_KIND_AIR) && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !boma.is_in_hitlag() {
        MotionModule::set_rate(boma, 2.0);
    }
}

// Lengthen knife
unsafe fn knife_length(boma: &mut BattleObjectModuleAccessor) {
	let long_sword_scale = Vector3f{x: 1.01, y: 1.1, z: 1.01};
	ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("knife"), &long_sword_scale);
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI_THROW,
        *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI2_END,
        *FIGHTER_JACK_STATUS_KIND_SPECIAL_LW_HOLD,
        *FIGHTER_JACK_STATUS_KIND_SPECIAL_LW_END,
        *FIGHTER_JACK_STATUS_KIND_SPECIAL_LW_ATTACK,
        *FIGHTER_JACK_STATUS_KIND_SPECIAL_LW_ENDURE,
        *FIGHTER_JACK_STATUS_KIND_SPECIAL_LW2_COUNTER,
        *FIGHTER_JACK_STATUS_KIND_SPECIAL_LW2_REFLECTOR
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
    wings_of_rebellion_cancel(boma, status_kind);
    //arsene_grappling_hook(boma, situation_kind, motion_kind);
    aerial_grappling_hook_stall(boma, motion_kind, frame);
    grappling_hook_spike_cancel(fighter, boma);
	knife_length(boma);
    //arsene_summon_desmummon(boma);
    fastfall_specials(fighter);
}

#[utils::macros::opff(FIGHTER_KIND_JACK )]
pub fn jack_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		jack_frame(fighter)
    }
}

pub unsafe fn jack_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}