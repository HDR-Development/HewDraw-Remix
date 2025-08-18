use super::*;

// FIGHTER_STATUS_KIND_APPEAL

pub unsafe extern "C" fn appeal_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    // down taunt looping
    if fighter.is_motion_one_of(&[Hash40::new("appeal_lw_l"), Hash40::new("appeal_lw_r")])
    && fighter.is_button_on(Buttons::AppealLw)
    && (39.0..62.0).contains(&fighter.motion_frame()) {
        MotionModule::set_frame(fighter.boma(), 28.0, true);
        let rate =  MotionModule::rate(fighter.boma());
        if rate == 1.0 {
            MotionModule::set_rate(fighter.boma(), 0.001); // slow down initially
        }
        if rate < 4.0 { // max rate of 4x speed
            MotionModule::set_rate(fighter.boma(), (rate * 1.01)); // gradually speed up
        }

        // special effects triggered at certain speed thresholds
        if (1.5..1.505).contains(&rate) {
            EFFECT_FLIP(fighter, Hash40::new("sys_ground_shockwave"), Hash40::new("sys_ground_shockwave"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_YZ);
            QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
            PLAY_SE(fighter, Hash40::new("se_common_smashswing_04"));
            ControlModule::set_rumble(fighter.boma(), Hash40::new("rbkind_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        if rate > 1.505 {
            LANDING_EFFECT(fighter, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, (0.35 * rate), 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_ALPHA(fighter, 0.1);
        }
        if (2.5..2.53).contains(&rate) {
            EFFECT_FLIP(fighter, Hash40::new("sys_ground_shockwave"), Hash40::new("sys_ground_shockwave"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_YZ);
            QUAKE(fighter, *CAMERA_QUAKE_KIND_L);
            PLAY_SE(fighter, Hash40::new("se_system_amiibo_entry_2"));
            ControlModule::set_rumble(fighter.boma(), Hash40::new("rbkind_explosionm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    }
    
    if !is_training_mode() { 
        return 0.into(); 
    }

    // training mode resource functionality
    if fighter.is_button_trigger(Buttons::Guard) {
        let materials: [[i32;3];7] = [
            // material, current material supply, max material supply
            [*FIGHTER_PICKEL_MATERIAL_KIND_GRADE_1, fighter.get_int(*FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_MATERIAL_NUM_GRADE_1), 15],
            [*FIGHTER_PICKEL_MATERIAL_KIND_WOOD, fighter.get_int(*FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_MATERIAL_NUM_WOOD), 15],
            [*FIGHTER_PICKEL_MATERIAL_KIND_STONE, fighter.get_int(*FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_MATERIAL_NUM_STONE), 15],
            [*FIGHTER_PICKEL_MATERIAL_KIND_IRON, fighter.get_int(*FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_MATERIAL_NUM_IRON), 15],
            [*FIGHTER_PICKEL_MATERIAL_KIND_GOLD, fighter.get_int(*FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_MATERIAL_NUM_GOLD), 12],
            [*FIGHTER_PICKEL_MATERIAL_KIND_RED_STONE, fighter.get_int(*FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_MATERIAL_NUM_RED_STONE), 15],
            [*FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND, fighter.get_int(*FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_MATERIAL_NUM_DIAMOND), 5]
        ];
        let remove_resources = fighter.is_button_on(Buttons::Special);
        for material in materials {
            if !remove_resources { // fill materials to defined amount
                FighterSpecializer_Pickel::add_material_num(fighter.boma(), material[0], material[2] - material[1]);
            } else { // remove all materials
                FighterSpecializer_Pickel::sub_material_num(fighter.boma(), material[0], material[1]);
            }
        }

        // checks the position of the control stick to determine what upgrade to give steve, if any
        let mut direction = "null"; 
        if (-0.25..0.25).contains(&fighter.stick_y()) { // stick is moved mostly horizontally
            if fighter.stick_x() >= 0.5 { direction = "right"; } 
            else if fighter.stick_x() <= -0.5 { direction = "left"; }
        } 
        else if (-0.25..0.25).contains(&fighter.stick_x()) { // stick is moved mostly vertically
            if fighter.stick_y() >= 0.5 { direction = "up"; } 
            else if fighter.stick_y() <= -0.5 { direction = "down"; }
        }
        let (mut material, effect) = match direction {
            "right" => (*FIGHTER_PICKEL_MATERIAL_KIND_STONE, "pickel_craft_icon_stone"),
            "down" => (*FIGHTER_PICKEL_MATERIAL_KIND_IRON, "pickel_craft_icon_iron"),
            "left" => (*FIGHTER_PICKEL_MATERIAL_KIND_GOLD, "pickel_craft_icon_gold"),
            "up" => (*FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND, "pickel_craft_icon_diamond"),
            /* neutral */ _ => (*FIGHTER_PICKEL_MATERIAL_KIND_WOOD, "pickel_craft_icon_wood")
        };

        if !remove_resources {
            let lr = PostureModule::lr(fighter.boma()) as i32;
            EFFECT_FOLLOW(fighter, Hash40::new(effect), Hash40::new("top"), 0, 20, -5 * lr, 0, 0, 0, 1, true);
            PLAY_SE(fighter, Hash40::new("se_pickel_special_n05")); // ding
        } else { // if special is held, his tools will instead break
            material = *FIGHTER_PICKEL_MATERIAL_KIND_NONE;
            PLAY_SE(fighter, Hash40::new("se_pickel_special_n_item_break")); // break sfx
        }
        let weapons: [i32;4] = [
            *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_SWORD, 
            *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_PICK, 
            *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_AXE, 
            *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_SHOVEL
        ];
        let pickel = fighter.global_table[0x4].get_ptr() as *mut Fighter;
        for kind in weapons { // assign the appropriate material
            FighterSpecializer_Pickel::set_craft_weapon_param(
                pickel, 
                FighterPickelCraftWeaponKind(kind), 
                FighterPickelMaterialKind(material), 
                100.0 // weapon durability
            ) 
        }
    }

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Exec, *FIGHTER_STATUS_KIND_APPEAL, appeal_exec);
}