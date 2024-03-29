use super::*;

unsafe extern "C" fn game_specialsthrow(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let gordo_speed_x = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);    
    
    if is_excute(agent){
        if VarModule::is_flag(owner_module_accessor.object(), vars::dedede::instance::IS_DASH_GORDO){
            PostureModule::reverse_rot_y_lr(boma);
            PostureModule::reverse_lr(boma);
        }
        /* Prevents backwards gordos */
        if PostureModule::lr(owner_module_accessor) * gordo_speed_x < 0.0{
            KineticModule::mul_speed(boma, &Vector3f{x: -1.0, y: 1.0,z:  1.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        }
    }
    /* Checks every frame the gordo is active, set equal to the gordo life param */
    for _ in 0..181{
        if is_excute(agent) {
            if !boma.is_status(*WEAPON_DEDEDE_GORDO_STATUS_KIND_HOP) {
                if VarModule::is_flag(owner_module_accessor.object(), vars::dedede::instance::IS_DASH_GORDO){

                    let bounce_dmg_multiplier = ((WorkModule::get_int(boma, *WEAPON_DEDEDE_GORDO_STATUS_WORK_INT_BOUND_COUNT) as f32 + 2.0) * 0.25);
                    ATTACK(agent, 0, 0, Hash40::new("hip"), 7.5 * bounce_dmg_multiplier, 120, 110, 60, 0, 6.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
                    ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 0.7);
                    
                    //Reduces the max amount of bounces by 1 per recatch on the same gordo
                    if (WorkModule::get_int(boma, *WEAPON_DEDEDE_GORDO_STATUS_WORK_INT_BOUND_COUNT) - VarModule::get_int(owner_module_accessor.object(), vars::dedede::instance::RECATCH_COUNTER)) < 0{
                        StatusModule::change_status_request(agent.module_accessor, *WEAPON_DEDEDE_GORDO_STATUS_KIND_DEAD, true);
                    }
                }
                else{
                    /* Reduces damage on every bounce, by 10% of its last damage in this case */
                    let bounce_dmg_multiplier = ((WorkModule::get_int(boma, *WEAPON_DEDEDE_GORDO_STATUS_WORK_INT_BOUND_COUNT) as f32 + 8.0) * 0.1);
                    ATTACK(agent, 0, 0, Hash40::new("hip"), 7.5 * bounce_dmg_multiplier, 60, 110, 60, 0, 6.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
                    ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 0.7);
                }
            }
        }
        wait(lua_state, 1.0);
    }
}

unsafe extern "C" fn effect_specialsthrow(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent){
        //Intentionally blank to kill vanilla effects
    }
}

unsafe extern "C" fn game_specialsshot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    for _ in 0..181{
        if is_excute(agent) {
            if !boma.is_status(*WEAPON_DEDEDE_GORDO_STATUS_KIND_HOP) {
                GroundModule::update_force(boma);
                /* Reduces damage on every bounce, by 12.5% of its last damage in this case */
                let bounce_dmg_multiplier = ((WorkModule::get_int(boma, *WEAPON_DEDEDE_GORDO_STATUS_WORK_INT_BOUND_COUNT) as f32 + 5.0) * 0.125);
                ATTACK(agent, 0, 0, Hash40::new("hip"), 12.8 * bounce_dmg_multiplier, 60, 50, 0, 60, 0.9, 3.8, 3.8, 0.0, Some(-3.8), Some(-3.8), Some(0.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -8.4, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
                ATTACK(agent, 1, 0, Hash40::new("hip"), 12.8 * bounce_dmg_multiplier, 60, 50, 0, 60, 0.9, 3.8, -3.8, 0.0, Some(-3.8), Some(3.8), Some(0.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -8.4, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
                ATTACK(agent, 2, 0, Hash40::new("hip"), 12.8 * bounce_dmg_multiplier, 60, 50, 0, 60, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -8.4, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
            }
        }
        wait(lua_state, 1.0);
    }
}

unsafe extern "C" fn effect_specialsshot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();

}

unsafe extern "C" fn game_specialsattack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);

    //Disables vanilla regrab searchbox, this ALWAYS needs to be on due to new regrab
    WorkModule::on_flag(owner_module_accessor, *FIGHTER_DEDEDE_INSTANCE_WORK_ID_FLAG_PERSONAL); 
    let mut speed_x = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
    let mut speed_y = KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);

    if is_excute(agent) {
        WorkModule::set_int(boma, 300, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
        /* below grabs the boma of the opponent hitting gordo, the attack data of that hit, and adjusts the speed accordingly */
        let num_players = Fighter::get_fighter_entry_count(); 
        if StopModule::is_hit(boma)
        && !StopModule::is_hit(owner_module_accessor) { 
            for i in 0..num_players{
                let opponent_boma = sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(i));

                if AttackModule::is_infliction(opponent_boma, *COLLISION_KIND_MASK_HIT){
                    let data = AttackModule::attack_data(opponent_boma, 0, false);
                    let mut angle = (*data).vector as f32;
                    let mut damage = (*data).power;
                    let kbg = (*data).r_eff as f32;
                    let bkb = (*data).r_add as f32;
                    
                    //Covering sakurai angle and other funky angles
                    if angle > 360.0{ 
                        angle = 32.0;
                    }
                    //Damage cap, gordo goes to the moon otherwise
                    if damage > 25.0{
                        damage = 25.0;
                    }

                    let radians = angle.to_radians();
                    let cos = radians.cos();
                    let sin = radians.sin();

                    //formulas for the speed multipliers
                    let x_speed_mul = cos * ((kbg * 0.3718) + bkb / 100.0) * (damage / 8.0) / 70.0;
                    let y_speed_mul =  sin  *  (damage / 2.5) * ((kbg * 0.3718) + bkb / 100.0) / 60.0 / speed_y;

                    KineticModule::mul_speed(boma, &Vector3f{x: x_speed_mul, y: y_speed_mul , z: 1.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL); 
                }
            }
        /* Seeing the speed is still the same. This only occurs if the above did not run, which happens on projectiles or non-direct hits (Bayo smash attacks) */
        if speed_x == KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) &&  speed_y == KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL){
            let damage = DamageModule::damage(boma, 0);
            if damage > 11.0{
                KineticModule::mul_speed(boma, &Vector3f{x: 0.8, y: 1.0, z: 1.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
            }
            else{
                KineticModule::mul_speed(boma, &Vector3f{x: 0.4 + 0.05 * (damage - 5.0), y: 1.0, z: 1.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
            }
        }
        }
    }
    for _ in 0..301{
        if is_excute(agent) {
            if !boma.is_status(*WEAPON_DEDEDE_GORDO_STATUS_KIND_HOP) {
                /* Reduces damage on every bounce, by 12.5% of its last damage in this case */
                let bounce_dmg_multiplier = ((WorkModule::get_int(boma, *WEAPON_DEDEDE_GORDO_STATUS_WORK_INT_BOUND_COUNT) as f32 + 5.0) * 0.125);
                if !StopModule::is_stop(boma){
                    speed_x = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL).abs();
                    speed_y = KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL).abs();
                }
                let mut speed = speed_x.max(speed_y);
                let mut damage = (7.5 * (speed * 0.6));
                damage = damage.max(7.5);

                ATTACK(agent, 0, 0, Hash40::new("hip"), damage * bounce_dmg_multiplier, 60, 110, 60, 0, 6.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
            }
        }
        wait(lua_state, 1.0);
    }
}

unsafe extern "C" fn effect_specialsattack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    //Intentionally blank to kill vanilla effects

}

unsafe extern "C" fn game_specialswallstop(agent: &mut L2CAgentBase){
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        
    }
}

unsafe extern "C" fn game_specialsstart(agent: &mut L2CAgentBase){
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    if is_excute(agent){
        if VarModule::is_flag(owner_module_accessor.object(), vars::dedede::instance::IS_DASH_GORDO){
            WorkModule::on_flag(boma, *WEAPON_DEDEDE_GORDO_STATUS_WORK_FLAG_VISIBILITY_ON);
        }
    }
    frame(lua_state, 7.0);
    if is_excute(agent){
        WorkModule::on_flag(boma, *WEAPON_DEDEDE_GORDO_STATUS_WORK_FLAG_VISIBILITY_ON);
    }
}

unsafe extern "C" fn game_specialairsstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);

    if is_excute(agent){
        if VarModule::is_flag(owner_module_accessor.object(), vars::dedede::instance::IS_DASH_GORDO){
            WorkModule::on_flag(boma, *WEAPON_DEDEDE_GORDO_STATUS_WORK_FLAG_VISIBILITY_ON);
        }
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *WEAPON_DEDEDE_GORDO_STATUS_WORK_FLAG_VISIBILITY_ON);
    }
}

pub fn install(agent: &mut Agent){
    agent.acmd("game_specialsthrow", game_specialsthrow);
    agent.acmd("effect_specialsthrow", effect_specialsthrow);

    agent.acmd("game_specialsshot", game_specialsshot);
    agent.acmd("effect_specialsshot", effect_specialsshot);

    agent.acmd("game_specialsattack", game_specialsattack);
    agent.acmd("effect_specialsattack", effect_specialsattack);

    agent.acmd("game_specialswallstop", game_specialswallstop);

    agent.acmd("game_specialsstart", game_specialsstart);

    agent.acmd("game_specialairsstart", game_specialairsstart);
}
