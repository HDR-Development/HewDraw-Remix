use super::*;
use globals::*;
utils::import!(common::djc::attack_air_main_status);
// status script import
 
pub fn install() {
    install_status_scripts!(
        move_exec,
        special_hi_hold_end,
        attack_air,
        special_hi_attack
    );
}

// WEAPON_NESS_PK_THUNDER_STATUS_KIND_MOVE

#[status_script(agent = "ness_pkthunder", status = WEAPON_NESS_PK_THUNDER_STATUS_KIND_MOVE, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn move_exec(weapon: &mut L2CFighterCommon) -> L2CValue {
    if !VarModule::is_flag(weapon.object(), vars::ness::status::THUNDER_LOOSE) {
        if LinkModule::get_parent_status_kind(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT) as i32 != *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_HOLD {
            VarModule::on_flag(weapon.object(), vars::ness::status::THUNDER_LOOSE);
            MotionModule::change_motion_force_inherit_frame(weapon.module_accessor, Hash40::new("move"), 0.0, 1.0, 1.0);
            return 0.into();
        }
        original!(weapon);
    }
    0.into() 
}

// FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_HOLD //

#[status_script(agent = "ness", status = FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_HOLD, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn special_hi_hold_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if LinkModule::is_link(fighter.module_accessor, *FIGHTER_NESS_LINK_NO_PK_THUNDER) {
        LinkModule::unlink(fighter.module_accessor, *FIGHTER_NESS_LINK_NO_PK_THUNDER);
    }
    if [*FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_ATTACK, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_REFLECT].contains(&fighter.global_table[STATUS_KIND].get_i32()) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_NESS_GENERATE_ARTICLE_PK_THUNDER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    if fighter.get_int(*FIGHTER_NESS_STATUS_SPECIAL_HI_WORK_INT_GUIDE_EFFECT_HANDLE) != 0 {
        EffectModule::detach(fighter.module_accessor, fighter.get_int(*FIGHTER_NESS_STATUS_SPECIAL_HI_WORK_INT_GUIDE_EFFECT_HANDLE) as u32, 5);
        fighter.set_int(0, *FIGHTER_NESS_STATUS_SPECIAL_HI_WORK_INT_GUIDE_EFFECT_HANDLE);
    }
    0.into() 
}

// FIGHTER_STATUS_KIND_ATTACK_AIR //

#[status_script(agent = "ness", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn attack_air(fighter: &mut L2CFighterCommon) -> L2CValue {
    common::djc::attack_air_main_status(fighter)
}

// FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_ATTACK //

#[status_script(agent = "ness", status = FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_hi_attack(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi"), 0.0, 1.0, false, 0.0, false, false);
    if !StopModule::is_stop(fighter.module_accessor) {
        sub_special_hi_attack(fighter);
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(sub_special_hi_attack as *const () as _));
    fighter.main_shift(special_hi_attack_main)
}

unsafe extern "C" fn sub_special_hi_attack(fighter: &mut L2CFighterCommon) -> L2CValue {
    // this does...something?
    if !fighter.is_flag(*FIGHTER_NESS_STATUS_SPECIAL_HI_FLAG_ATTACK_FALL_START) {
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    else {
        if fighter.is_situation(*SITUATION_KIND_AIR) {
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
        else {
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
    }
    if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) {
        AttackModule::clear_inflict_kind_status(fighter.module_accessor);
    }
    0.into()
}

unsafe extern "C" fn special_hi_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if !MotionModule::is_end(fighter.module_accessor) {
        if fighter.is_flag(*FIGHTER_NESS_STATUS_SPECIAL_HI_FLAG_LANDING_ENABLE) && fighter.is_situation(*SITUATION_KIND_GROUND) {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
            return 1.into();
        }
        // [insert stubbed redirection/bonk function here]
        // LOL good riddance fucker
        if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR && fighter.status_frame() >= 38 {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
            let fall_x_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "fall_max_x_mul");
            sv_kinetic_energy!(
                set_stable_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                air_speed_x_stable * fall_x_mul,
                0.0
            );
        }
        0.into()
    }
    else {
        fighter.change_status(FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END.into(), false.into());
        1.into()
    }
}