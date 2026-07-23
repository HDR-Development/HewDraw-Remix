// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn bow_lc(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status(*FIGHTER_PIT_STATUS_KIND_SPECIAL_N_SHOOT) {
        let landing_lag = 7.0;
        boma.check_land_cancel(Some(landing_lag));
    }
}

unsafe fn guardian_orbitar(fighter: &mut L2CFighterCommon) {
    // happens on init
    if fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
    ])
    && StatusModule::is_changing(fighter.module_accessor) {
        if !VarModule::is_flag(fighter.battle_object, vars::pitb::instance::SPECIAL_LW_DISABLE_STALL) {
            sv_kinetic_energy!(
                set_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                0.0
            );
            sv_kinetic_energy!(
                set_accel,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                0.0
            );
        }
        VarModule::off_flag(fighter.battle_object, vars::pitb::instance::SPECIAL_LW_DISABLE_LC);
        VarModule::on_flag(fighter.battle_object, vars::pitb::instance::SPECIAL_LW_DISABLE_STALL);
    }

    // happens on main for all statuses
    if fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_PIT_STATUS_KIND_SPECIAL_LW_HOLD,
        *FIGHTER_PIT_STATUS_KIND_SPECIAL_LW_END,
    ]) {
        // disable land cancel on parry
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_PARRY) {
            VarModule::on_flag(fighter.battle_object, vars::pitb::instance::SPECIAL_LW_DISABLE_LC);
            if !fighter.is_status(*FIGHTER_PIT_STATUS_KIND_SPECIAL_LW_END)
            && !fighter.is_in_hitlag() {
                fighter.change_status(FIGHTER_PIT_STATUS_KIND_SPECIAL_LW_END.into(), false.into());
            }
        }

        // land cancel
        if !VarModule::is_flag(fighter.battle_object, vars::pitb::instance::SPECIAL_LW_DISABLE_LC) {
            fighter.check_land_cancel(None);
        }
    }
}

unsafe fn electroshock_land_cancel_on_hit(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_PIT_STATUS_KIND_SPECIAL_S_END)
    && fighter.get_int(*FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_INT_START_SITUATION) == *SITUATION_KIND_AIR
    && StatusModule::is_situation_changed(fighter.module_accessor)
    && fighter.is_situation(*SITUATION_KIND_GROUND)
    && fighter.is_flag(*FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_HIT)
    && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
        
        fighter.set_int(*SITUATION_KIND_GROUND, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_INT_START_SITUATION);
    }
}

extern "Rust" {
    fn pits_common(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32);
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    bow_lc(boma);
    guardian_orbitar(fighter);
    electroshock_land_cancel_on_hit(fighter);
    pits_common(fighter, boma, status_kind);
}

pub extern "C" fn pitb_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		pitb_frame(fighter)
    }
}

pub unsafe fn pitb_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, pitb_frame_wrapper);
}
