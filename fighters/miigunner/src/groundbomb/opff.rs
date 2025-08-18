utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;

pub extern "C" fn groundbomb_frame(weapon: &mut smash::lua2cpp::L2CFighterBase) {
    unsafe {
        if weapon.is_status(*WEAPON_MIIGUNNER_GROUNDBOMB_STATUS_KIND_FLY) {
            if StatusModule::is_changing(weapon.module_accessor) {
                TeamModule::set_hit_team(weapon.module_accessor, -1);
            }
            if StopModule::is_hit(weapon.module_accessor) {
                // Disable hit detection so whoever is hitting doesn't (instantly) blow themselves up
                HitModule::set_whole(weapon.module_accessor, HitStatus(*HIT_STATUS_OFF), 0);
                AttackModule::sleep(weapon.module_accessor, true);
                GroundModule::set_attach_ground(weapon.module_accessor, false);
                let life = weapon.get_param_int("param_groundbomb", "life");
                let explosion_frame = weapon.get_param_int("param_groundbomb", "damage_reflect_explosion_frame");
                weapon.set_int(life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
                weapon.set_int(explosion_frame, *WEAPON_MIIGUNNER_GROUNDBOMB_INSTANCE_WORK_ID_INT_DAMAGE_REFLECT_AFTER_COUNT);
                let num_players = Fighter::get_fighter_entry_count();
                for i in 0..num_players {
                    let opponent_boma = sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(i));
                    if AttackModule::is_infliction(opponent_boma, *COLLISION_KIND_MASK_HIT) {
                        let h = (&mut *(opponent_boma)).kind();
                        let hit_team = TeamModule::team_no(opponent_boma);
                        TeamModule::set_team(weapon.module_accessor, hit_team as i32, false);
                        TeamModule::set_team_second(weapon.module_accessor, hit_team as i32);
                    }
                }
            }
            if !weapon.is_flag(*WEAPON_MIIGUNNER_GROUNDBOMB_INSTANCE_WORK_ID_FLAG_DAMAGE_REFLECT)
            && weapon.get_int(*WEAPON_MIIGUNNER_GROUNDBOMB_INSTANCE_WORK_ID_INT_DAMAGE_REFLECT_AFTER_COUNT) < 37 {
                if weapon.is_situation(*SITUATION_KIND_GROUND) {
                    AttackModule::sleep(weapon.module_accessor, true);
                }
                else {
                    AttackModule::sleep(weapon.module_accessor, false);
                }
            }
            if weapon.get_int(*WEAPON_MIIGUNNER_GROUNDBOMB_INSTANCE_WORK_ID_INT_DAMAGE_REFLECT_AFTER_COUNT) == 37 {
                HitModule::set_whole(weapon.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
                AttackModule::sleep(weapon.module_accessor, false);
                GroundModule::set_attach_ground(weapon.module_accessor, true);
                weapon.off_flag(*WEAPON_MIIGUNNER_GROUNDBOMB_INSTANCE_WORK_ID_FLAG_DAMAGE_REFLECT);
                weapon.off_flag(*WEAPON_MIIGUNNER_GROUNDBOMB_INSTANCE_WORK_ID_FLAG_DAMAGE_REFLECTED);
                weapon.off_flag(*WEAPON_MIIGUNNER_GROUNDBOMB_INSTANCE_WORK_ID_FLAG_REFLECT);
            }
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, groundbomb_frame);
}