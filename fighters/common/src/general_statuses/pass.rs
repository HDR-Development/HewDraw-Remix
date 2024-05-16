use super::*;
use globals::*;

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_Pass_Main_sub_hook,
        );
    }
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

    // idk what this does
    if fighter.global_table[0x26].get_bool() {
        let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[0x26].get_ptr());
        callable(fighter);
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

    // DSpecial cancel
    if fighter.is_cat_flag(Cat1::SpecialLw)
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW) {
        if fighter.global_table[0x3B].get_bool() && {
            let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[0x3B].get_ptr());
            callable(fighter).get_bool()
        } {
            return 1.into();
        }
        else {
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
    if fighter.is_cat_flag(Cat1::AttackLw4) {
        if fighter.global_table[0x59].get_bool() && {
            let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[0x59].get_ptr());
            callable(fighter).get_bool()
        } {
            return 1.into();
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_LW4_START.into(), true.into());
            return 1.into();
        }
    }

    // DTilt cancel
    if fighter.is_cat_flag(Cat1::AttackLw3) {
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_LW3.into(), true.into());
        return 1.into();
    }
    return 0.into();
}