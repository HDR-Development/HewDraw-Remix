use super::*;

unsafe extern "C" fn bullet_fly_uniq_process(weapon: &mut L2CFighterBase, arg: L2CValue) -> L2CValue {
    if !arg.get_bool() {
        return 0.into();
    }

    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if weapon.get_int(*WEAPON_INSTANCE_WORK_ID_INT_LIFE) <= 0 {
        AttackModule::reset_safe_pos(weapon.module_accessor);
        {
            weapon.clear_lua_stack();
            lua_args!(weapon, Hash40::new_raw(0x199c462b5d));
            app::sv_battle_object::notify_event_msc_cmd(weapon.lua_state_agent);
        }
    }

    0.into()
}

#[status_script(agent = "falco_blaster_bullet", status = WEAPON_FALCO_BLASTER_BULLET_STATUS_KIND_FLY, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn bullet_fly_pre(weapon: &mut L2CFighterBase) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NORMAL,
        *GROUND_CORRECT_KIND_NONE as u32,
        app::GroundCliffCheckKind(0),
        false,
        0,
        0,
        0,
        0
    );
    0.into()
}

#[status_script(agent = "falco_blaster_bullet", status = WEAPON_FALCO_BLASTER_BULLET_STATUS_KIND_FLY, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn bullet_fly(weapon: &mut L2CFighterBase) -> L2CValue {
    weapon.global_table[globals::SUB_STATUS].assign(&L2CValue::Ptr(bullet_fly_uniq_process as *const () as _));
    weapon.fast_shift(bullet_fly_main)
}


unsafe extern "C" fn bullet_fly_main(weapon: &mut L2CFighterBase) -> L2CValue {
    let flags = if weapon.global_table[globals::CURRENT_FRAME].get_i32() <= 6 {
        let flags = (GroundModule::get_touch_flag(weapon.module_accessor) as u32) & 0xF;
        if PostureModule::lr(weapon.module_accessor) < 0.0 {
            flags & !(*GROUND_TOUCH_FLAG_RIGHT as u32)
        } else {
            flags & !(*GROUND_TOUCH_FLAG_LEFT as u32)
        }
    } else {
        (GroundModule::get_touch_flag(weapon.module_accessor) as u32) & 0xFF
    };
    
    if flags != 0 && !weapon.is_passable_collision_touch(flags) {
        weapon.change_status(WEAPON_FALCO_BLASTER_BULLET_STATUS_KIND_STAY.into(), false.into());
    }

    0.into()
}

pub fn install() {
    smashline::install_status_scripts!(
        bullet_fly_pre,
        bullet_fly
    );
}