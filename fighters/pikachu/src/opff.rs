// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
// Disable QA jump cancels if not directly QA into the ground
unsafe fn disable_qa_jc(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, frame: f32) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if status_kind == *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_WARP {
        // only allow QAC from QA1
        if WorkModule::get_int(boma, *FIGHTER_PIKACHU_STATUS_WORK_ID_INT_QUICK_ATTACK_COUNT) > 1 {
            VarModule::on_flag(boma.object(), vars::pikachu::instance::DISABLE_QA_JC);
        }
    }
    if status_kind == *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_END {
        // only allow QAC from QA into ground
        if situation_kind == *SITUATION_KIND_AIR && frame > 2.0 {
            VarModule::on_flag(boma.object(), vars::pikachu::instance::DISABLE_QA_JC);
        }
    }
}

// Reset JC disable flag
unsafe fn reset_jc_disable_flag(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32) {
    if situation_kind == *SITUATION_KIND_GROUND
    && ![*FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_WARP, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL].contains(&status_kind)
    && VarModule::is_flag(boma.object(), vars::pikachu::instance::DISABLE_QA_JC) {
        VarModule::off_flag(boma.object(), vars::pikachu::instance::DISABLE_QA_JC);
    }
}

// JC Quick Attack/Agility
unsafe fn jc_qa_agility(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, cat1: i32, stick_x: f32, facing: f32, frame: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL
    && boma.status_frame() > 3
    && situation_kind == *SITUATION_KIND_GROUND
    && StatusModule::prev_status_kind(boma, 0) == *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_END
    && !VarModule::is_flag(boma.object(), vars::pikachu::instance::DISABLE_QA_JC)
    {
        boma.check_jump_cancel(true);
    }
}

pub unsafe fn electric_rats_moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    disable_qa_jc(boma, id, status_kind, situation_kind, frame);
    reset_jc_disable_flag(boma, id, status_kind, situation_kind);
    jc_qa_agility(boma, id, status_kind, situation_kind, cat[0], stick_x, facing, frame);
    fastfall_specials(fighter);
}


#[no_mangle]
pub unsafe extern "Rust" fn electric_rats_common(fighter: &mut L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        electric_rats_moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_HOLD,
        *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_END,
        *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_END,
        *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_LW_HIT
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

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    // nothing lol
}

#[utils::macros::opff(FIGHTER_KIND_PIKACHU )]
pub fn pikachu_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		pikachu_frame(fighter);
        electric_rats_common(fighter);
    }
}

pub unsafe fn pikachu_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}