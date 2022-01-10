use common::opff_import::*;
use super::*;
use globals::*;
use common::opff::*;
 
unsafe fn elytra_cancel(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if (status_kind == *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_HI_GLIDING) {
        // Increment glide timer during elytra
        glide_timer[id] += 1.0;
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
            if(glide_timer[id] > (25.0) && glide_timer[id] < (45.0) ){
                //VarModule::on_flag(boma, common::UP_SPECIAL_CANCEL);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_HI_FALL_SPECIAL,false);
            }
        }
    }
    // Reset glide timer if not gliding
    else{
        if(glide_timer[id] > 0.0){
            glide_timer[id] = 0.0;
        }
    }
}


// Glow red during hitstun and green during tumble
unsafe fn hitstun_tumble_glow(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32) {

    // Glow red during hitstun
    let cbm_vec1 = Vector4f{ /* Red */ x: 0.85, /* Green */ y: 0.85, /* Blue */ z: 0.85, /* Alpha */ w: 0.2};
    let cbm_vec2 = Vector4f{ /* Red */ x: 0.9907, /* Green */ y: 0.02, /* Blue */ z: 0.0251, /* Alpha */ w: 0.8};
    if (WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME) > 0.0) {
        if  !VarModule::is_flag(get_battle_object_from_accessor(boma), vars::common::IS_IN_HITSTUN) {
            hitstun_start[id] = true;
        }
    } else {
        if  VarModule::is_flag(get_battle_object_from_accessor(boma), vars::common::IS_IN_HITSTUN) {
            ColorBlendModule::cancel_main_color(boma, 0);
        }
        VarModule::off_flag(get_battle_object_from_accessor(boma), vars::common::IS_IN_HITSTUN);
    }
    if (hitstun_start[id]) {
        VarModule::on_flag(get_battle_object_from_accessor(boma), vars::common::IS_IN_HITSTUN);
        ColorBlendModule::set_main_color(boma, /* Brightness */ &cbm_vec1, /* Diffuse */ &cbm_vec2, 0.21, 2.2, 5, /* Display Color */ true);
        hitstun_start[id] = false;
    }

    // Glow green during tumble
    let cbm_t_vec1 = Vector4f{ /* Red */ x: 0.85, /* Green */ y: 0.85, /* Blue */ z: 0.85, /* Alpha */ w: 0.2};
    let cbm_t_vec2 = Vector4f{ /* Red */ x: 0.1612, /* Green */ y: 0.2549, /* Blue */ z: 0.098, /* Alpha */ w: 0.8};
    if (status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FALL) {
        if (!is_in_tumble[id]) {
            tumble_start[id] = true;
        }
    } else {
        if (is_in_tumble[id]) {
            ColorBlendModule::cancel_main_color(boma, 0);
        }
        is_in_tumble[id] = false;
    }
    if (tumble_start[id]) {
        is_in_tumble[id] = true;
        ColorBlendModule::set_main_color(boma, /* Brightness */ &cbm_t_vec1, /* Diffuse */ &cbm_t_vec2, 0.21, 2.2, 3, /* Display Color */ true);
        tumble_start[id] = false;
    }

}

// this makes it easier to place blocks diagonally down during build-walk
unsafe fn buildwalk_crouch_disable(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if [*FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_WALK, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_WALK_BACK].contains(&status_kind) {
        WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT);
    }
}

// Logging for deciphering ACMD scripts
unsafe fn logging_for_acmd(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {

    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3 || status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
        println!("craft_weapon_kind: {}", WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_KIND));
        println!("request_have_craft_weapon_kind: {}", WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND));
        println!("craft_sword: {}", *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_SWORD);
        println!("craft_axe: {}", *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_AXE);
        println!("craft_pick: {}", *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_PICK);
        println!("craft_shovel: {}", *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_SHOVEL);
    }

}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    elytra_cancel(boma, id, status_kind, situation_kind, cat[0], frame);
    hitstun_tumble_glow(boma, id, status_kind);
    buildwalk_crouch_disable(boma, status_kind);
    //logging_for_acmd(boma, status_kind);
}

#[utils::opff(FIGHTER_KIND_PICKEL )]
pub fn pickel_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        fighter_common_opff(fighter);
		pickel_frame(fighter)
    }
}

pub unsafe fn pickel_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = crate::hooks::sys_line::FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}
