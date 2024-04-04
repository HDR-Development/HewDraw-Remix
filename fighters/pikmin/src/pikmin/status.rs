use smash::lib::L2CAgent_clear_lua_stack;
use super::*;
use globals::*;

// WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_S_CLING

pub unsafe extern "C" fn special_s_cling_main(weapon: &mut L2CFighterCommon) -> L2CValue {
    weapon.off_flag(*WEAPON_PIKMIN_PIKMIN_STATUS_SPECIAL_S_WORK_FLAG_POKEMON_CHANGE_START);
    AttackModule::set_ignore_just_shield(weapon.module_accessor, true);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("sp_s_grab_attack"), 0.0, 1.0, false, 0.0, false, false);

    if !weapon.is_flag(*WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_FLAG_IS_SPECIAL_S_CLING_ENEMY)
    && !weapon.is_flag(*WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_FLAG_IS_SPECIAL_S_CLING_CARRIER) {
        let target = weapon.get_int64(*WEAPON_PIKMIN_PIKMIN_STATUS_SPECIAL_S_WORK_INT_TARGET_NODE);
        LinkModule::set_model_constraint_pos_ort(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT, Hash40::new("top"), Hash40::new_raw(target), (*CONSTRAINT_FLAG_SET_MATRIX | *CONSTRAINT_FLAG_NO_FLIP) as u32, true);
        weapon.clear_lua_stack();
        weapon.push_lua_stack(&mut L2CValue::new_int(0x367b17cdfd));
        sv_battle_object::notify_event_msc_cmd(weapon.lua_state_agent);
    }

    let founder_id = sv_battle_object::get_founder_id(weapon.global_table[OBJECT_ID].get_u32());
    let mut founder_module_accessor = sv_battle_object::module_accessor(founder_id as u32);
    if !founder_module_accessor.is_null() {
        let power_special_s = WorkModule::get_param_float(founder_module_accessor, hash40("power_special_s"), 0);
        AttackModule::set_power_up(weapon.module_accessor, power_special_s);
        let founder_power_mul_status = AttackModule::power_mul_status(founder_module_accessor);
        AttackModule::set_power_mul_status(weapon.module_accessor, founder_power_mul_status);
    }

    let variation = weapon.get_int(*WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
    let p = PikminInfo::from(variation);
    VarModule::set_int(weapon.battle_object, vars::pikmin::status::SPECIAL_S_PIKMIN_DETONATE_TIMER, 0);
    VarModule::off_flag(weapon.battle_object, vars::pikmin::status::SPECIAL_S_PIKMIN_DETONATE_IS_ATTACK_LAST_FRAME);
    VarModule::off_flag(weapon.battle_object, vars::pikmin::instance::SPECIAL_S_PIKMIN_DETONATE_IS_DETACH_FOR_DETONATE);

    weapon.fastshift(L2CValue::Ptr(special_s_cling_main_loop as *const () as _))
}

unsafe extern "C" fn special_s_cling_main_loop(weapon: &mut L2CFighterCommon) -> L2CValue {
    let frame = MotionModule::frame(weapon.module_accessor);
    let clatter_time = ControlModule::get_clatter_time(weapon.module_accessor, 0);
    weapon.set_int(clatter_time as i32, *WEAPON_PIKMIN_PIKMIN_STATUS_SPECIAL_S_WORK_INT_CLATTER_TIME);
    let variation = weapon.get_int(*WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
    let p = PikminInfo::from(variation);

    if !weapon.global_table[IS_STOPPING].get_bool()
    && !StatusModule::is_changing(weapon.module_accessor)
    {
        let is_attack = AttackModule::is_attack(weapon.module_accessor, 0, false);
        if is_attack && !VarModule::is_flag(weapon.battle_object, vars::pikmin::status::SPECIAL_S_PIKMIN_DETONATE_IS_ATTACK_LAST_FRAME) {
            VarModule::inc_int(weapon.battle_object, vars::pikmin::status::SPECIAL_S_PIKMIN_DETONATE_TIMER);
        }
        VarModule::set_flag(weapon.battle_object, vars::pikmin::status::SPECIAL_S_PIKMIN_DETONATE_IS_ATTACK_LAST_FRAME, is_attack);
        if VarModule::get_int(weapon.battle_object, vars::pikmin::status::SPECIAL_S_PIKMIN_DETONATE_TIMER) >= p.cling_frame{
            VarModule::on_flag(weapon.battle_object, vars::pikmin::instance::SPECIAL_S_PIKMIN_DETONATE_IS_DETACH_FOR_DETONATE);
            weapon.change_status(WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_S_CLING_REMOVE.into(), false.into());
            return 1.into();
        }
    }

    if weapon.is_flag(*WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_FLAG_IS_SPECIAL_S_CLING_ENEMY)
    || weapon.is_flag(*WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_FLAG_IS_SPECIAL_S_CLING_CARRIER) {
        return 0.into();
    }

    if !weapon.is_flag(*WEAPON_PIKMIN_PIKMIN_STATUS_SPECIAL_S_CLING_WORK_FLAG_IS_CONSTRAINT) {
        weapon.clear_lua_stack();
        weapon.push_lua_stack(&mut L2CValue::new_int(0x367b17cdfd));
        sv_battle_object::notify_event_msc_cmd(weapon.lua_state_agent);
        if frame > 3.0 {
            weapon.on_flag(*WEAPON_PIKMIN_PIKMIN_STATUS_SPECIAL_S_CLING_WORK_FLAG_IS_CONSTRAINT);
            let target = weapon.get_int64(*WEAPON_PIKMIN_PIKMIN_STATUS_SPECIAL_S_WORK_INT_TARGET_NODE);
            LinkModule::set_model_constraint_pos_ort(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT, Hash40::new("top"), Hash40::new_raw(target), (*CONSTRAINT_FLAG_POSITION | *CONSTRAINT_FLAG_NO_FLIP | *CONSTRAINT_FLAG_ORIENTATION) as u32, true);
        }
    }
    if LinkModule::is_link(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT) {
        LinkModule::send_event_parents(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT, Hash40::new_raw(0x23fcecfa97));
    }

    return 0.into();
}

// WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_S_CLING_REMOVE

pub unsafe extern "C" fn special_s_cling_remove_end(weapon: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(weapon.battle_object, vars::pikmin::instance::SPECIAL_S_PIKMIN_DETONATE_IS_DETACH_FOR_DETONATE);
    smashline::original_status(End, weapon, *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_S_CLING_REMOVE)(weapon)
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_S_CLING, special_s_cling_main);
    agent.status(End, *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_S_CLING_REMOVE, special_s_cling_remove_end);
}
