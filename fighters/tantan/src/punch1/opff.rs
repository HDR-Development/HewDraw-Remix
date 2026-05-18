// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe extern "C" fn dragon_frame(weapon: &mut L2CFighterBase) {
    if !WorkModule::is_flag(weapon.module_accessor, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_IS_LEFT)
    && !WorkModule::is_flag(weapon.module_accessor, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_IS_KIRBY) {
        let minmin_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
        if sv_battle_object::is_active(minmin_id) {
            let minmin = utils::util::get_battle_object_from_id(minmin_id);
            let minmin_boma = &mut *(*minmin).module_accessor;
            let bigScale = WorkModule::get_param_float(minmin_boma,hash40("param_private"),hash40("arm_l_big_scale"));

            // Only update if previously was not dragonized
            let mut is_dragonized = WorkModule::is_flag(weapon.module_accessor, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_IS_DRAGONIZE);
            if !is_dragonized {
                is_dragonized = WorkModule::get_int(minmin_boma, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_ARM_L_BIG_FRAME) > 0;
                WorkModule::set_flag(weapon.module_accessor, is_dragonized, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_IS_DRAGONIZE);  
                if is_dragonized {
                    let handle = EffectModule::req_follow(weapon.module_accessor, Hash40::new("tantan_dragon_attack_fire"), Hash40::new("gimmickc"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, bigScale, true, 0, 0, 0, 0, 0, false, false);
                }
            }
            if is_dragonized {
                PostureModule::set_scale(weapon.module_accessor, bigScale, false);
                AttackModule::set_power_mul_5th(weapon.module_accessor, 1.5);
            }
        }
    }

    if StatusModule::status_kind(weapon.module_accessor) == *WEAPON_TANTAN_PUNCH1_STATUS_KIND_BACK {
        let owner_boma = weapon.get_owner_boma();
        if owner_boma.is_status(*FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_AIR_REACH) {
            VarModule::off_flag(owner_boma.object(), vars::tantan::instance::SPECIAL_HI_ENABLE_FREEFALL);
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, dragon_frame);
}