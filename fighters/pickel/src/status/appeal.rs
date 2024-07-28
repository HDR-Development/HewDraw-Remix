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
    }
    
    if !is_training_mode() { 
        return 0.into(); 
    }

    if fighter.is_button_trigger(Buttons::Guard) {
        let materials: [[i32;3];7] = [
            // material, current material supply, max material supply
            [*FIGHTER_PICKEL_MATERIAL_KIND_GRADE_1, fighter.get_int(*FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_MATERIAL_NUM_GRADE_1), 40],
            [*FIGHTER_PICKEL_MATERIAL_KIND_WOOD, fighter.get_int(*FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_MATERIAL_NUM_WOOD), 20],
            [*FIGHTER_PICKEL_MATERIAL_KIND_STONE, fighter.get_int(*FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_MATERIAL_NUM_STONE), 20],
            [*FIGHTER_PICKEL_MATERIAL_KIND_IRON, fighter.get_int(*FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_MATERIAL_NUM_IRON), 20],
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

        let mut material = *FIGHTER_PICKEL_MATERIAL_KIND_WOOD;
        let mut effect = "pickel_craft_icon_wood";
        // checks the position of the control stick to determine what upgrade to give steve, if any
        if (-0.25..0.25).contains(&fighter.stick_y()) { // stick is moved mostly horizontally
            if fighter.stick_x() >= 0.5 { // holding right1
                material = *FIGHTER_PICKEL_MATERIAL_KIND_STONE;
                effect = "pickel_craft_icon_stone";
            } else if fighter.stick_x() <= -0.5 { // holding left
                material = *FIGHTER_PICKEL_MATERIAL_KIND_GOLD;
                effect = "pickel_craft_icon_gold";
            }
        }
        if (-0.25..0.25).contains(&fighter.stick_x()) { // stick is moved mostly vertically
            if fighter.stick_y() >= 0.5 { // holding up
                material = *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND;
                effect = "pickel_craft_icon_diamond";
            } else if fighter.stick_y() <= -0.5 { // holding down
                material = *FIGHTER_PICKEL_MATERIAL_KIND_IRON; 
                effect = "pickel_craft_icon_iron";
            }
        }
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