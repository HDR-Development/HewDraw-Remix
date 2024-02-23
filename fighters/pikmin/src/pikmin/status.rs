use smash::lib::L2CAgent_clear_lua_stack;
use super::*;
use globals::*;

pub fn install() {
    install_status_scripts!(
        special_s_cling_main,
        special_s_cling_remove_end,
    );
}

// WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_S_CLING

#[status_script(agent = "pikmin_pikmin", status = WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_S_CLING, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_s_cling_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.off_flag(*WEAPON_PIKMIN_PIKMIN_STATUS_SPECIAL_S_WORK_FLAG_POKEMON_CHANGE_START);
    AttackModule::set_ignore_just_shield(fighter.module_accessor, true);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("sp_s_grab_attack"), 0.0, 1.0, false, 0.0, false, false);

    if !fighter.is_flag(*WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_FLAG_IS_SPECIAL_S_CLING_ENEMY)
    && !fighter.is_flag(*WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_FLAG_IS_SPECIAL_S_CLING_CARRIER) {
        let target = fighter.get_int64(*WEAPON_PIKMIN_PIKMIN_STATUS_SPECIAL_S_WORK_INT_TARGET_NODE);
        LinkModule::set_model_constraint_pos_ort(fighter.module_accessor, *WEAPON_LINK_NO_CONSTRAINT, Hash40::new("top"), Hash40::new_raw(target), (*CONSTRAINT_FLAG_SET_MATRIX | *CONSTRAINT_FLAG_NO_FLIP) as u32, true);
        fighter.clear_lua_stack();
        fighter.push_lua_stack(&mut L2CValue::new_int(0x367b17cdfd));
        sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
    }

    let founder_id = sv_battle_object::get_founder_id(fighter.global_table[OBJECT_ID].get_u32());
    let mut founder_module_accessor = sv_battle_object::module_accessor(founder_id as u32);
    if !founder_module_accessor.is_null() {
        let power_special_s = WorkModule::get_param_float(founder_module_accessor, hash40("power_special_s"), 0);
        AttackModule::set_power_up(fighter.module_accessor, power_special_s);
        let founder_power_mul_status = AttackModule::power_mul_status(founder_module_accessor);
        AttackModule::set_power_mul_status(fighter.module_accessor, founder_power_mul_status);
    }

    let variation = fighter.get_int(*WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
    let p = PikminInfo::from(variation);
    VarModule::set_int(fighter.battle_object, vars::pikmin::status::SPECIAL_S_PIKMIN_DETONATE_TIMER, 0);
    VarModule::off_flag(fighter.battle_object, vars::pikmin::status::SPECIAL_S_PIKMIN_DETONATE_IS_ATTACK_LAST_FRAME);
    VarModule::off_flag(fighter.battle_object, vars::pikmin::instance::SPECIAL_S_PIKMIN_DETONATE_IS_DETACH_FOR_DETONATE);

    fighter.fastshift(L2CValue::Ptr(special_s_cling_main_loop as *const () as _))
}

unsafe extern "C" fn special_s_cling_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = MotionModule::frame(fighter.module_accessor);
    let clatter_time = ControlModule::get_clatter_time(fighter.module_accessor, 0);
    fighter.set_int(clatter_time as i32, *WEAPON_PIKMIN_PIKMIN_STATUS_SPECIAL_S_WORK_INT_CLATTER_TIME);
    let variation = fighter.get_int(*WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
    let p = PikminInfo::from(variation);

    if !fighter.global_table[IS_STOPPING].get_bool()
    && !StatusModule::is_changing(fighter.module_accessor)
    {
        let is_attack = AttackModule::is_attack(fighter.module_accessor, 0, false);
        if is_attack && !VarModule::is_flag(fighter.battle_object, vars::pikmin::status::SPECIAL_S_PIKMIN_DETONATE_IS_ATTACK_LAST_FRAME) {
            VarModule::inc_int(fighter.battle_object, vars::pikmin::status::SPECIAL_S_PIKMIN_DETONATE_TIMER);
        }
        VarModule::set_flag(fighter.battle_object, vars::pikmin::status::SPECIAL_S_PIKMIN_DETONATE_IS_ATTACK_LAST_FRAME, is_attack);
        if VarModule::get_int(fighter.battle_object, vars::pikmin::status::SPECIAL_S_PIKMIN_DETONATE_TIMER) >= p.cling_frame{
            VarModule::on_flag(fighter.battle_object, vars::pikmin::instance::SPECIAL_S_PIKMIN_DETONATE_IS_DETACH_FOR_DETONATE);
            fighter.change_status(WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_S_CLING_REMOVE.into(), false.into());
            return 1.into();
        }
    }

    if fighter.is_flag(*WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_FLAG_IS_SPECIAL_S_CLING_ENEMY)
    || fighter.is_flag(*WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_FLAG_IS_SPECIAL_S_CLING_CARRIER) {
        return 0.into();
    }

    if !fighter.is_flag(*WEAPON_PIKMIN_PIKMIN_STATUS_SPECIAL_S_CLING_WORK_FLAG_IS_CONSTRAINT) {
        fighter.clear_lua_stack();
        fighter.push_lua_stack(&mut L2CValue::new_int(0x367b17cdfd));
        sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
        if frame > 3.0 {
            fighter.on_flag(*WEAPON_PIKMIN_PIKMIN_STATUS_SPECIAL_S_CLING_WORK_FLAG_IS_CONSTRAINT);
            let target = fighter.get_int64(*WEAPON_PIKMIN_PIKMIN_STATUS_SPECIAL_S_WORK_INT_TARGET_NODE);
            LinkModule::set_model_constraint_pos_ort(fighter.module_accessor, *WEAPON_LINK_NO_CONSTRAINT, Hash40::new("top"), Hash40::new_raw(target), (*CONSTRAINT_FLAG_POSITION | *CONSTRAINT_FLAG_NO_FLIP | *CONSTRAINT_FLAG_ORIENTATION) as u32, true);
        }
    }
    if LinkModule::is_link(fighter.module_accessor, *WEAPON_LINK_NO_CONSTRAINT) {
        LinkModule::send_event_parents(fighter.module_accessor, *WEAPON_LINK_NO_CONSTRAINT, Hash40::new_raw(0x23fcecfa97));
    }

    return 0.into();
}

// WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_S_CLING_REMOVE

#[status_script(agent = "pikmin_pikmin", status = WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_S_CLING_REMOVE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn special_s_cling_remove_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.battle_object, vars::pikmin::instance::SPECIAL_S_PIKMIN_DETONATE_IS_DETACH_FOR_DETONATE);
    original!(fighter)
}