// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn laser_land_cancel(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_N) {
        fighter.check_land_cancel(None);
    }
}

unsafe fn firebird_startup_ledgegrab(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI) {
        // allows ledgegrab during Firebird startup
        fighter.sub_transition_group_check_air_cliff();
    }
}

unsafe fn check_special_lw_hit(fighter: &mut L2CFighterCommon) {
    if fighter.is_flag(0x200000e0) // FIGHTER_FALCO_INSTANCE_WORK_ID_FLAG_REFLECTOR
    && (!fighter.is_status(statuses::falco::SPECIAL_LW_HIT) || fighter.motion_frame() > 10.0) {
        fighter.change_status(statuses::falco::SPECIAL_LW_HIT.into(), false.into());
    }
}

unsafe fn check_special_lw_parried(fighter: &mut L2CFighterCommon) {
    if fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        statuses::falco::SPECIAL_LW_LOOP,
        statuses::falco::SPECIAL_LW_END,
        statuses::falco::SPECIAL_LW_HIT
    ]) 
    && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_PARRY) {
        VarModule::on_flag(fighter.battle_object, vars::falco::instance::SPECIAL_LW_DISABLE_JC);
        if !fighter.is_status(statuses::falco::SPECIAL_LW_END)
        && !fighter.is_in_hitlag() {
            fighter.change_status(statuses::falco::SPECIAL_LW_END.into(), false.into());
        }
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_FALCO_STATUS_KIND_SPECIAL_HI_RUSH_END,
        *FIGHTER_FALCO_STATUS_KIND_SPECIAL_HI_BOUND
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {

    laser_land_cancel(fighter);
    firebird_startup_ledgegrab(fighter);
    check_special_lw_hit(fighter);
    check_special_lw_parried(fighter);
    fastfall_specials(fighter);
}

pub extern "C" fn falco_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		falco_frame(fighter)
    }
}

pub unsafe fn falco_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, falco_frame_wrapper);
}
