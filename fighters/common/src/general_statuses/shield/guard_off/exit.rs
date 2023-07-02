// status imports
use super::*;
use globals::*;

#[common_status_script(status = FIGHTER_STATUS_KIND_GUARD_OFF, condition = LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS,
    symbol = "_ZN7lua2cpp16L2CFighterCommon42sub_ftStatusUniqProcessGuardOff_exitStatusEv")]
unsafe fn ftStatusUniqProcessGuardOff_exitStatus(fighter: &mut L2CFighterCommon) -> L2CValue {
    if app::FighterUtil::is_valid_just_shield(fighter.module_accessor) {
        ShieldModule::set_status(fighter.module_accessor, *FIGHTER_SHIELD_KIND_GUARD, app::ShieldStatus(*SHIELD_STATUS_NONE), 0);
        let shield_type = app::FighterUtil::get_shield_type_of_guard(fighter.global_table[FIGHTER_KIND].get_i32()) as i32;
        ShieldModule::set_shield_type(fighter.module_accessor, app::ShieldType(shield_type), *FIGHTER_SHIELD_KIND_GUARD, 0);
        ReflectorModule::set_status(fighter.module_accessor, 0, app::ShieldStatus(*SHIELD_STATUS_NONE), *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD);
        ShieldModule::set_hit_stop_mul(fighter.module_accessor, 1.0);
    }
    if fighter.global_table[STATUS_KIND] == FIGHTER_STATUS_KIND_GUARD_DAMAGE 
        && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
        effect!(fighter, MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new_raw(0xafae75f05), true, true);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x262a7a102d));
    }
    let shield_type = app::FighterUtil::get_shield_type_of_guard(fighter.global_table[FIGHTER_KIND].get_i32()) as i32;
    ShieldModule::set_shield_type(fighter.module_accessor, app::ShieldType(shield_type), *FIGHTER_SHIELD_KIND_GUARD, 0);
    if fighter.global_table[STATUS_KIND] == FIGHTER_STATUS_KIND_SPECIAL_HI
        || fighter.global_table[STATUS_KIND] == FIGHTER_STATUS_KIND_JUMP_SQUAT
        || fighter.global_table[STATUS_KIND] == FIGHTER_STATUS_KIND_ATTACK_HI4_START {
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_GUARD_FRAME);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_ESCAPE_FRAME);
    }
    L2CValue::I32(0)
}

pub fn install() {
    install_status_scripts!(
        ftStatusUniqProcessGuardOff_exitStatus
    );
}