// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

pub extern "C" fn miigunner_missile_frame(weapon: &mut smash::lua2cpp::L2CFighterBase) {
    unsafe {
        let boma = weapon.boma();
        let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        // Ensure the boma's owner is Mii Gunner
        if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_MIIGUNNER {
            let gunner = utils::util::get_battle_object_from_id(owner_id);
            let gunner_boma = &mut *(*gunner).module_accessor;
            if StatusModule::status_kind(boma) == *WEAPON_MIIGUNNER_SUPERMISSILE_STATUS_KIND_STRAIGHT
            && gunner_boma.is_cat_flag(Cat1::SpecialS)
            && VarModule::is_flag(gunner, vars::miigunner::instance::DETONATE_READY) {
                if WorkModule::is_enable_transition_term_group(gunner_boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ATTACK)
                    || WorkModule::is_enable_transition_term_group(gunner_boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK)
                    || WorkModule::is_enable_transition_term_group(gunner_boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_SPECIAL)
                    || WorkModule::is_enable_transition_term_group(gunner_boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL)
                    || WorkModule::is_enable_transition_term_group(gunner_boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP) {
                    StatusModule::change_status_request_from_script(boma, *WEAPON_MIIGUNNER_SUPERMISSILE_STATUS_KIND_S_BURST, false);
                    VarModule::on_flag(gunner, vars::miigunner::status::MISSILE_DETONATE);
                    VarModule::off_flag(gunner, vars::miigunner::instance::DETONATE_READY);
                    gunner_boma.clear_commands(Cat1::SpecialS); // Clear command so Gunner doesn't immediately fire another missile
                }
            }
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, miigunner_missile_frame);
}
