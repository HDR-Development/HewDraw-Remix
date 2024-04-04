// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

/// prevents rocket from despawning in the blastzone
unsafe extern "C" fn dragon_frame(weapon: &mut L2CFighterBase) {
    let boma = weapon.module_accessor;

    let mut is_dragonized = WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_IS_DRAGONIZE);
    if !WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_IS_LEFT)
    && !WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_IS_KIRBY)
    {
        let minmin_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
        if sv_battle_object::is_active(minmin_id) {
            let minmin = utils::util::get_battle_object_from_id(minmin_id);
            let minmin_boma = &mut *(*minmin).module_accessor;
            let bigScale = WorkModule::get_param_float(minmin_boma,hash40("param_private"),hash40("arm_l_big_scale"));

            //Only update if previously was not dragonized
            if !is_dragonized {
                is_dragonized = WorkModule::get_int(minmin_boma, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_ARM_L_BIG_FRAME) > 0;
                WorkModule::set_flag(boma, is_dragonized,*WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_IS_DRAGONIZE);  

                if is_dragonized {
                    let handle = EffectModule::req_follow(boma, Hash40::new("tantan_dragon_attack_fire"), Hash40::new("gimmickc"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, bigScale, true, 0, 0, 0, 0, 0, false, false);
                }
            }
            if is_dragonized {
                PostureModule::set_scale(boma, bigScale, false);
            }
        }
    }
    if is_dragonized {
        AttackModule::set_power_mul(boma, 1.15);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, dragon_frame);
}
