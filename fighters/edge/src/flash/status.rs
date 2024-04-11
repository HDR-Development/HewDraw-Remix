use super::*;
use globals::*;

unsafe extern "C" fn wait_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("wait"), 0.0, 1.0, false, 0.0, false, false);
    VarModule::set_int(weapon.battle_object, vars::edge_flash::status::LIFE, 600); //make param
    weapon.fastshift(L2CValue::Ptr(wait_main_loop as *const () as _))
}

unsafe extern "C" fn wait_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let owner_object = utils::util::get_battle_object_from_id(owner_id);
    let edge_boma = &mut *(*owner_object).module_accessor;
    if VarModule::get_int(weapon.battle_object, vars::edge_flash::status::LIFE) <= 0 {
        StatusModule::change_status_force(weapon.module_accessor, statuses::edge_flash::VANISH, false);
        return 1.into()
    }
    if edge_boma.kind() == *FIGHTER_KIND_EDGE {
        // break
        if edge_boma.is_status_one_of(&[*FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_CHARGED_RUSH]) {
            let pos_x = PostureModule::pos_x(weapon.module_accessor) - PostureModule::pos_x(edge_boma);
            let pos_y = PostureModule::pos_y(weapon.module_accessor) - PostureModule::pos_y(edge_boma);
            let dist_mod_x = 10.0 * PostureModule::scale(weapon.module_accessor);   //make param
            let dist_mod_y = 18.0 * PostureModule::scale(weapon.module_accessor);   //make param
            if pos_x.abs() < dist_mod_x && pos_y.abs() < dist_mod_y {
                SoundModule::play_se(weapon.module_accessor, Hash40::new("se_edge_special_l04"), true, false, false, false, app::enSEType(0));
                StatusModule::change_status_force(weapon.module_accessor, statuses::edge_flash::BURST, false);
                return 1.into()
            }
        }
        // reflect flare
        if VarModule::get_int(owner_object, vars::edge::instance::FIRE_ID) != -1 {
            let pos_x = VarModule::get_float(owner_object, vars::edge::instance::FIRE_POS_X) - PostureModule::pos_x(weapon.module_accessor);
            let pos_y = VarModule::get_float(owner_object, vars::edge::instance::FIRE_POS_Y) - PostureModule::pos_y(weapon.module_accessor);
            let dist_mod_x = 5.0 * PostureModule::scale(weapon.module_accessor);    //make param
            let dist_mod_y = 18.0 * PostureModule::scale(weapon.module_accessor);   //make param
            if pos_x.abs() < dist_mod_x && pos_y.abs() < dist_mod_y {
                if VarModule::get_int(weapon.battle_object, vars::edge_flash::status::REFLECT_COOOLDOWN) <= 0 {
                    VarModule::set_int(weapon.battle_object, vars::edge_flash::status::REFLECT_COOOLDOWN, 30);   //make param?
                    VarModule::on_flag(owner_object, vars::edge::instance::FLASH_REFLECT);
                }
            }
        }
        // refract shadow flare
        if VarModule::get_int(owner_object, vars::edge::instance::FLARE1_ID) != -1 {
            let pos_x = VarModule::get_float(owner_object, vars::edge::instance::FLARE1_POS_X) - PostureModule::pos_x(weapon.module_accessor);
            let pos_y = VarModule::get_float(owner_object, vars::edge::instance::FLARE1_POS_X) - PostureModule::pos_y(weapon.module_accessor);
            let dist_mod_x = 5.0 * PostureModule::scale(weapon.module_accessor);    //make param
            let dist_mod_y = 18.0 * PostureModule::scale(weapon.module_accessor);   //make param
            if pos_x.abs() < dist_mod_x && pos_y.abs() < dist_mod_y {
                VarModule::on_flag(owner_object, vars::edge::instance::FLASH_REFRACT);
            }
        }
    }

    return 0.into()
}

unsafe extern "C" fn wait_exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
    VarModule::dec_int(weapon.battle_object, vars::edge_flash::status::LIFE);
    if VarModule::get_int(weapon.battle_object, vars::edge_flash::status::REFLECT_COOOLDOWN) > 0 {
        VarModule::dec_int(weapon.battle_object, vars::edge_flash::status::REFLECT_COOOLDOWN);
    }
    return 0.into()
}

unsafe extern "C" fn burst_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    EffectModule::kill_kind(weapon.module_accessor, Hash40::new("edge_senkou_shield"), false, false);
    smashline::original_status(Pre, weapon, *WEAPON_EDGE_FLASH_STATUS_KIND_ATTACK)(weapon)
}

unsafe extern "C" fn burst_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    EffectModule::req_on_joint(weapon.module_accessor, Hash40::new("edge_senkou_shield_break"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 0.6, &Vector3f::zero(), &Vector3f::zero(), true, 0, 0, 0);
    smashline::original_status(Main, weapon, *WEAPON_EDGE_FLASH_STATUS_KIND_ATTACK)(weapon)
}

unsafe extern "C" fn burst_exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
    smashline::original_status(Exec, weapon, *WEAPON_EDGE_FLASH_STATUS_KIND_ATTACK)(weapon)
}

unsafe extern "C" fn burst_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    smashline::original_status(End, weapon, *WEAPON_EDGE_FLASH_STATUS_KIND_ATTACK)(weapon)
}

unsafe extern "C" fn vanish_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    EffectModule::kill_kind(weapon.module_accessor, Hash40::new("edge_senkou_shield"), false, false);
    StatusModule::init_settings(
        weapon.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NORMAL,
        0,
        app::GroundCliffCheckKind(*GROUND_CORRECT_KIND_AIR),
        false,
        0,
        0,
        0,
        0
    );
    return 0.into()
}

unsafe extern "C" fn vanish_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("vanish"), 0.0, 1.0, false, 0.0, false, false);
    return 0.into()
}

unsafe extern "C" fn vanish_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    return 0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *WEAPON_EDGE_FLASH_STATUS_KIND_ATTACK, wait_main);
    agent.status(Exec, *WEAPON_EDGE_FLASH_STATUS_KIND_ATTACK, wait_exec);

    agent.status(Pre, statuses::edge_flash::BURST, burst_pre);
    agent.status(Main, statuses::edge_flash::BURST, burst_main);
    agent.status(Exec, statuses::edge_flash::BURST, burst_exec);
    agent.status(End, statuses::edge_flash::BURST, burst_end);

    agent.status(Pre, statuses::edge_flash::VANISH, vanish_pre);
    agent.status(Main, statuses::edge_flash::VANISH, vanish_main);
    agent.status(End, statuses::edge_flash::VANISH, vanish_end);
}