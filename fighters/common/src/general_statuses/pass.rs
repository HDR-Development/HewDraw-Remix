use super::*;
use globals::*;

#[skyline::hook(replace = L2CFighterCommon_status_pre_Pass)]
unsafe extern "C" fn status_pre_Pass(fighter: &mut L2CFighterCommon) {
	StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_GROUND_STOP,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_ENABLE,
        false,
        false,
        true,
        0,
        0,
        0,
        0
    );
}

#[skyline::hook(replace = L2CFighterCommon_status_Pass_common)]
unsafe extern "C" fn status_Pass_common(fighter: &mut L2CFighterCommon) {
    fighter.sub_air_check_fall_common_pre();
    fighter.enable_transition_term_many(&[
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_STAND
    ]);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("pass"), 0.0, 1.0, false, 0.0, false, false);
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_fall_common_uniq(false.into());
    }
    sv_kinetic_energy!(
        set_brake,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        0.0,
        0.0
    );
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_fall_common_uniq as *const () as _));
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_Pass_Main_sub)]
unsafe fn status_Pass_Main_sub(fighter: &mut L2CFighterCommon, arg1: L2CValue) -> L2CValue {
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

#[skyline::hook(replace = L2CFighterCommon_sub_set_pass)]
unsafe extern "C" fn sub_set_pass(fighter: &mut L2CFighterCommon) {
    ControlModule::reset_flick_y(fighter.module_accessor);
    ControlModule::reset_flick_sub_y(fighter.module_accessor);
    fighter.global_table[FLICK_Y].assign(&0xFE.into());

    GroundModule::pass_floor(fighter.module_accessor);
    StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), false);
    let prev_situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    fighter.global_table[PREV_SITUATION_KIND].assign(&L2CValue::I32(prev_situation_kind));
    fighter.global_table[SITUATION_KIND].assign(&L2CValue::I32(*SITUATION_KIND_AIR));
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));

    let pass_speed_y = fighter.get_param_float("common", "pass_speed_y");
    KineticModule::add_speed(fighter.module_accessor, &Vector3f::new(0.0, pass_speed_y, 0.0));

    // if entering platdrop with more than max horizontal air speed
    // multiply horizontal speed by horizontal jump speed multiplier
    // then clamp it to between max horizontal air speed and 1.8 times max horizontal air speed
    let curr_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
    let air_speed_x_stable = fighter.get_param_float("air_speed_x_stable", "");
    if curr_speed_x.abs() > air_speed_x_stable {
        let jump_speed_x_mul = fighter.get_param_float("jump_speed_x_mul", "").sqrt(); // normalized
        let pass_air_speed_x_max_mul = ParamModule::get_float(fighter.object(), ParamType::Shared, "pass_air_speed_x_max_mul");
        let new_speed_x = (curr_speed_x.abs() * jump_speed_x_mul).clamp(air_speed_x_stable, dbg!(air_speed_x_stable * pass_air_speed_x_max_mul)) * curr_speed_x.signum();
        let adjust_speed_x = (dbg!(new_speed_x) - curr_speed_x) * PostureModule::lr(fighter.module_accessor);
        KineticModule::add_speed(fighter.module_accessor, &Vector3f::new(adjust_speed_x, 0.0, 0.0));
    }
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_Pass,
            status_Pass_common,
            status_Pass_Main_sub,
            sub_set_pass
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}