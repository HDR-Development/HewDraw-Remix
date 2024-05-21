use super::*;
use globals::*;

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pass_common,
            status_Pass_Main_sub_hook,
        );
    }
}

#[skyline::hook(replace = L2CFighterCommon_status_Pass_common)]
unsafe extern "C" fn status_pass_common(fighter: &mut L2CFighterCommon) {
    fighter.sub_air_check_fall_common_pre();
    let transitions = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_STAND
    ];
    for val in transitions.iter() {
        WorkModule::enable_transition_term(fighter.module_accessor, *val);
    }
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("pass"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_fall_common_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_fall_common_uniq as *const () as _));
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_Pass_Main_sub)]
pub unsafe fn status_Pass_Main_sub_hook(fighter: &mut L2CFighterCommon, arg1: L2CValue) -> L2CValue {
    let pass_frame = fighter.get_int(*FIGHTER_STATUS_PASS_WORK_INT_FRAME);
    if pass_frame == 0 {
        if !fighter.is_flag(*FIGHTER_STATUS_PASS_FLAG_IS_SET_PASS) {
            let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(arg1.get_ptr());
            if callable(fighter).get_bool() {
                return 0.into();
            }
        }
        if !fighter.sub_air_check_fall_common().get_bool() {
            if MotionModule::is_end(fighter.module_accessor) {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
        }
        return 0.into();
    }
    if pass_frame <= 0 {
        return 0.into();
    }

    // skip direct cancels from restricted statuses
    let skip_cancels = fighter.is_prev_status_one_of(&[
        *FIGHTER_STATUS_KIND_GUARD,
        *FIGHTER_STATUS_KIND_GUARD_DAMAGE,
        *FIGHTER_STATUS_KIND_GUARD_ON,
        *FIGHTER_STATUS_KIND_ESCAPE_AIR,
        *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE
    ]);
    if skip_cancels {
        return 0.into();
    }

    // idk what this does
    if fighter.global_table[0x26].get_bool() {
        let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[0x26].get_ptr());
        callable(fighter);
    }

    // DSpecial cancel
    if fighter.is_cat_flag(Cat1::SpecialLw)
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW) {
        let mut cont = true;
        if fighter.global_table[0x3B].get_bool() {
            let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[0x3B].get_ptr());
            cont = callable(fighter).get_bool();
        }
        if cont {
            fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), true.into());
            return 1.into();
        }
    }

    // Item throw cancels
    if fighter.is_cat_flag(Cat1::Catch)
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE)
    && ItemModule::is_have_item(fighter.module_accessor, 0) {
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_NO_THROW);
        sv_module_access::item(fighter.lua_state_agent);
        if fighter.pop_lua_stack(1).get_bool() {
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), false.into());
        }
    }
    if fighter.is_cat_flag(Cat1::AttackN)
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW) {
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_THROW);
        sv_module_access::item(fighter.lua_state_agent);
        let mut throw = fighter.pop_lua_stack(1).get_bool();
        if !throw {
            fighter.clear_lua_stack();
            lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_SHOOT);
            sv_module_access::item(fighter.lua_state_agent);
            if !fighter.pop_lua_stack(1).get_bool() {
                throw = false;
            }
            else {
                throw = ItemModule::get_shoot_item_bullet(fighter.module_accessor, 0) <= 0;
            }
        }
        if throw {
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), false.into());
            return 1.into();
        }
    }

    // DSmash cancel
    if fighter.is_cat_flag(Cat1::AttackLw4)
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START) {
        let mut cont = true;
        if fighter.global_table[0x59].get_bool() {
            let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[0x59].get_ptr());
            cont = callable(fighter).get_bool();
        }
        if cont {
            fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_LW4_START.into(), true.into());
            return 1.into();
        }
    }

    // DTilt cancel
    if fighter.global_table[0x55].get_bool() {
        let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[0x55].get_ptr());
        if callable(fighter).get_bool() {
            return 1.into();
        }
    }
    if fighter.is_cat_flag(Cat1::AttackLw3)
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3) {
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_LW3.into(), true.into());
        return 1.into();
    }
    return 0.into();
}