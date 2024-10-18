use super::*;
use globals::*;

// This file contains code related to knockdown states

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_DamageFlyChkUniq,
            status_pre_Down,
            status_Down_Main,
            status_end_DownStandFb,
            bind_address_call_status_end_DownStandFb,
            sub_down_wait_common
        );
    }
}

// This runs as you enter knockdown
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_pre_Down)]
unsafe fn status_pre_Down(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_GROUND_STOP,
        *GROUND_CORRECT_KIND_GROUND as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        true,
        *FIGHTER_TREADED_KIND_DISABLE,
        false,
        true,
        false,
        0,
        *FIGHTER_STATUS_ATTR_SLOPE_TOP_UNLIMIT as u32,
        0,
        0
    );
    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_DamageFlyChkUniq)]
unsafe fn sub_DamageFlyChkUniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = call_original!(fighter);
    if ret.get_bool() {
        // reset speed mul for damage_speed_up when transitioning to down
        // without this, there be crazy momentum shenanigans
        if fighter.is_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGE_SPEED_UP) {
            fighter.off_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGE_SPEED_UP);
            fighter.set_float(0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_SPEED_UP_MAX_MAG);
        }
    }
    ret
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_Down_Main)]
unsafe fn status_Down_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_down_common();
    0.into()
}

// This runs at the end of getup rolls
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_end_DownStandFb)]
unsafe fn status_end_DownStandFb(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.0, 0.0);
    app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_bind_address_call_status_end_DownStandFb)]
unsafe fn bind_address_call_status_end_DownStandFb(fighter: &mut L2CFighterCommon, _agent: &mut L2CAgent) -> L2CValue {
    fighter.status_end_DownStandFb();
    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_down_wait_common)]
unsafe fn sub_down_wait_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return true.into();
    }

    if MotionModule::is_end(fighter.module_accessor)
    && fighter.is_motion_one_of(&[Hash40::new("down_bound_u"), Hash40::new("down_bound_d"), Hash40::new("down_eat_u"), Hash40::new("down_eat_d")]) {
        let down_motion = if fighter.global_table[PREV_STATUS_KIND] == FIGHTER_STATUS_KIND_GLIDE_LANDING || !fighter.is_flag(*FIGHTER_STATUS_DOWN_FLAG_UP) {
            Hash40::new("down_wait_d")
        } else {
            Hash40::new("down_wait_u")
        };
        MotionModule::change_motion(fighter.module_accessor, down_motion, 0.0, 1.0, false, 0.0, false, false);
    }

    if fighter.is_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_KNOCKOUT) 
    || fighter.get_int(*FIGHTER_STATUS_DOWN_WORK_INT_NO_ACTION_FRAME) != 0
    || fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
        return false.into();
    }

    if ItemModule::get_pickable_item_size(fighter.module_accessor) == *ITEM_SIZE_LIGHT as u64
    && {
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_CMD_ITEM_IS_GET_PICKABLE_ITEM);
        sv_module_access::item(fighter.lua_state_agent);
        fighter.pop_lua_stack(1).get_bool()
    }
    && {
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_PICKABLE_ITEM_TRAIT, ITEM_TRAIT_FLAG_FOOD);
        sv_module_access::item(fighter.lua_state_agent);
        fighter.pop_lua_stack(1).get_bool()
    }
    && fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0 {
        fighter.change_status(FIGHTER_STATUS_KIND_DOWN_EAT.into(), true.into());
        return true.into();
    }

    if fighter.is_cat_flag(Cat1::AttackN | Cat1::SpecialAny) {
        fighter.change_status(FIGHTER_STATUS_KIND_DOWN_STAND_ATTACK.into(), true.into());
        return true.into();
    }

    if fighter.is_cat_flag(Cat2::DownToDownStandFB) {
        fighter.change_status(FIGHTER_STATUS_KIND_DOWN_STAND_FB.into(), true.into());
        return true.into();
    }

    if fighter.is_cat_flag(Cat2::DownToDownStand) || fighter.is_button_trigger(Buttons::Guard) {
        fighter.change_status(FIGHTER_STATUS_KIND_DOWN_STAND.into(), true.into());
        return true.into();
    }

    if fighter.get_float(*FIGHTER_STATUS_DOWN_WORK_FLOAT_DOWN_FRAME) <= 0.0 {
        fighter.change_status(FIGHTER_STATUS_KIND_DOWN_STAND.into(), false.into());
        return true.into();
    }
    return false.into();
}
