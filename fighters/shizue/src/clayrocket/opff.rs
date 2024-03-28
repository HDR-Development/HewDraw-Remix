// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

//Lloid explode on hit
unsafe extern "C" fn lloid_callback(weapon : &mut L2CFighterBase) {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER);
    let shizue = utils::util::get_battle_object_from_id(owner_id as u32);
    let shizue_boma = &mut *(*shizue).module_accessor;
    let status = StatusModule::status_kind(weapon.module_accessor);
    if [*WEAPON_SHIZUE_CLAYROCKET_STATUS_KIND_BURST,
        *WEAPON_SHIZUE_CLAYROCKET_STATUS_KIND_DISAPPEAR].contains(&status) {
        VarModule::off_flag(shizue, vars::shizue::instance::LLOID_ASYNC);
        VarModule::set_int(shizue, vars::shizue::instance::LLOID_TIMER, 0);
    }
    if status == *WEAPON_SHIZUE_CLAYROCKET_STATUS_KIND_READY {
        if shizue_boma.is_cat_flag(Cat1::SpecialLw)
        && !VarModule::is_flag(shizue, vars::shizue::instance::LLOID_ASYNC)
        && !shizue_boma.is_status(*FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_LW_SET)
        && !CancelModule::is_enable_cancel(shizue_boma)
        && !shizue_boma.is_status_one_of(&[
            *FIGHTER_STATUS_KIND_DAMAGE,
            *FIGHTER_STATUS_KIND_DAMAGE_AIR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
            *FIGHTER_STATUS_KIND_DAMAGE_FALL]) {
            VarModule::on_flag(shizue, vars::shizue::instance::LLOID_ASYNC);
            VarModule::set_int(shizue, vars::shizue::instance::LLOID_TIMER, 10);
            EFFECT(&mut weapon.agent_base, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        }
    }
    if VarModule::is_flag(shizue, vars::shizue::instance::LLOID_ASYNC) {
        if VarModule::get_int(shizue, vars::shizue::instance::LLOID_TIMER) > 0 {
            VarModule::dec_int(shizue, vars::shizue::instance::LLOID_TIMER);
        }
        else {
            VarModule::off_flag(shizue, vars::shizue::instance::LLOID_ASYNC);
            StatusModule::change_status_request_from_script(weapon.boma(), *WEAPON_SHIZUE_CLAYROCKET_STATUS_KIND_FLY, true);
        }
    }
    if status == *WEAPON_SHIZUE_CLAYROCKET_STATUS_KIND_FLY
    && (AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_HIT)
    || AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_SHIELD))
    {
        StatusModule::change_status_request_from_script(weapon.boma(), *WEAPON_SHIZUE_CLAYROCKET_STATUS_KIND_BURST, true);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, lloid_callback);
}
