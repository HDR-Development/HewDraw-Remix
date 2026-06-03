use super::*;

unsafe extern "C" fn attack_squat_4_check_attack(fighter: &mut L2CFighterCommon, param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    if (&param_3["object_category_"]).get_i32() == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if (&param_3["kind_"]).get_i32() == *COLLISION_KIND_HIT {
            let object_id = (&param_3["object_id_"]).get_u32();
            let opponent_boma = sv_battle_object::module_accessor(object_id);
            if StatusModule::situation_kind(opponent_boma) == *SITUATION_KIND_AIR {
                let opponent_object = utils::util::get_battle_object_from_accessor(opponent_boma);
                VarModule::on_flag(opponent_object, vars::common::instance::FORCE_TUMBLE_NO_BOUNCE);
                StatusModule::set_status_kind_interrupt(opponent_boma, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR);
            }
        }
    }
    
    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(CheckAttack, *FIGHTER_DEMON_STATUS_KIND_ATTACK_SQUAT_4, attack_squat_4_check_attack);
}