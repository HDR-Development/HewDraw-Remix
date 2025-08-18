use super::*;
use utils::ext::*;

// #[skyline::hook(offset = 0x33e29c0)]
// pub unsafe extern "C" fn can_on_damage(vtable: u64, weapon: &mut app::Weapon, param_3: *const u64) -> u64 {
//     let collisionLog = *(param_3.add(0x28 / 0x8)) as *const CollisionLog;
//     let opponent_boma = &mut *(sv_battle_object::module_accessor((*collisionLog).opponent_battle_object_id));
//     if opponent_boma.kind() == *FIGHTER_KIND_DUCKHUNT {
//         if opponent_boma.is_status(*FIGHTER_DUCKHUNT_STATUS_KIND_SPECIAL_HI_END) {
//             let weapon_boma = weapon.battle_object.module_accessor;
//             StatusModule::change_status_request(weapon_boma, *WEAPON_DUCKHUNT_CAN_STATUS_KIND_EXPLODE, false);
//         }
//     }

//     return call_original!(vtable, weapon, param_3);
// }

#[skyline::hook(offset = 0x9a5090)]
pub unsafe extern "C" fn duckhunt_on_attack(vtable: u64, battleObject: *mut BattleObject, collisionLog: CollisionLog) -> u64 {
    let boma = &mut (*(&mut *(battleObject)).module_accessor);
    if boma.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI) {
        let opponent_boma = &mut *(sv_battle_object::module_accessor(collisionLog.opponent_battle_object_id));
        if opponent_boma.is_weapon() && opponent_boma.kind() == *WEAPON_KIND_DUCKHUNT_CAN {
            if boma.is_motion(Hash40::new("special_hi")) {
                let data = AttackModule::attack_data(boma, 0, false);
                (*data).target_lr = *ATTACK_LR_CHECK_F as u16;
                (*data).vector = ParamModule::get_int(boma.object(), ParamType::Agent, "param_special_hi.hi1_can_angle");
            }
            else if boma.is_motion(Hash40::new("special_hi_2")) {
                let data = AttackModule::attack_data(boma, 0, false);
                (*data).target_lr = *ATTACK_LR_CHECK_F as u16;
                (*data).vector = ParamModule::get_int(boma.object(), ParamType::Agent, "param_special_hi.hi2_can_angle");
            }
        }
    }
    if boma.is_status(*FIGHTER_DUCKHUNT_STATUS_KIND_SPECIAL_HI_END) {
        let opponent_boma = &mut *(sv_battle_object::module_accessor(collisionLog.opponent_battle_object_id));
        if opponent_boma.is_weapon() && opponent_boma.kind() == *WEAPON_KIND_DUCKHUNT_CAN {
            VarModule::on_flag(opponent_boma.object(), vars::duckhunt_can::instance::KILLSHOT_EXPLODE);
            StatusModule::change_status_request(opponent_boma, *WEAPON_DUCKHUNT_CAN_STATUS_KIND_EXPLODE, false);
        }
    }

    return call_original!(vtable, battleObject, collisionLog);
}

pub fn install() {
    skyline::install_hooks!(
        duckhunt_on_attack
    );
}