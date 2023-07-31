// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn elytra_cancel(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if (status_kind == *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_HI_GLIDING) {
        // Increment glide timer during elytra
        VarModule::add_float(boma.object(), vars::pickel::status::GLIDE_TIMER, 1.0);
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
            if(VarModule::get_float(boma.object(), vars::pickel::status::GLIDE_TIMER) > (25.0) && VarModule::get_float(boma.object(), vars::pickel::status::GLIDE_TIMER) < (45.0) ){
                //VarModule::on_flag(boma.object(), vars::common::instance::UP_SPECIAL_CANCEL);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_HI_FALL_SPECIAL,false);
            }
        }
    }
    // Reset glide timer if not gliding
    // No longer needed due to it being a status variable
    // else{
    //     if(VarModule::get_float(boma.object(), vars::pickel::status::GLIDE_TIMER) > 0.0){
    //         VarModule::set_float(boma.object(), vars::pickel::status::GLIDE_TIMER, 0.0);
    //     }
    // }
}


// Glow red during hitstun and green during tumble
unsafe fn hitstun_tumble_glow(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32) {

    // Glow red during hitstun
    let cbm_vec1 = Vector4f{ /* Red */ x: 0.85, /* Green */ y: 0.85, /* Blue */ z: 0.85, /* Alpha */ w: 0.2};
    let cbm_vec2 = Vector4f{ /* Red */ x: 0.9907, /* Green */ y: 0.02, /* Blue */ z: 0.0251, /* Alpha */ w: 0.8};
    if (WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME) > 0.0) {
        if  !VarModule::is_flag(boma.object(), vars::common::instance::IS_IN_HITSTUN) {
            VarModule::on_flag(boma.object(), vars::common::instance::HITSTUN_START);
        }
    } else {
        if  VarModule::is_flag(boma.object(), vars::common::instance::IS_IN_HITSTUN) {
            ColorBlendModule::cancel_main_color(boma, 0);
        }
        VarModule::off_flag(boma.object(), vars::common::instance::IS_IN_HITSTUN);
    }
    if  VarModule::is_flag(boma.object(), vars::common::instance::HITSTUN_START) {
        VarModule::on_flag(boma.object(), vars::common::instance::IS_IN_HITSTUN);
        ColorBlendModule::set_main_color(boma, /* Brightness */ &cbm_vec1, /* Diffuse */ &cbm_vec2, 0.21, 2.2, 5, /* Display Color */ true);
        VarModule::off_flag(boma.object(), vars::common::instance::HITSTUN_START);
    }

    // Glow green during tumble
    let cbm_t_vec1 = Vector4f{ /* Red */ x: 0.85, /* Green */ y: 0.85, /* Blue */ z: 0.85, /* Alpha */ w: 0.2};
    let cbm_t_vec2 = Vector4f{ /* Red */ x: 0.1612, /* Green */ y: 0.2549, /* Blue */ z: 0.098, /* Alpha */ w: 0.8};
    if (status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FALL) {
        if  !VarModule::is_flag(boma.object(), vars::pickel::instance::IS_IN_TUMBLE) {
            VarModule::on_flag(boma.object(), vars::pickel::instance::TUMBLE_START);
        }
    } else {
        if  VarModule::is_flag(boma.object(), vars::pickel::instance::IS_IN_TUMBLE) {
            ColorBlendModule::cancel_main_color(boma, 0);
        }
        VarModule::off_flag(boma.object(), vars::pickel::instance::IS_IN_TUMBLE);
    }
    if  VarModule::is_flag(boma.object(), vars::pickel::instance::TUMBLE_START) {
        VarModule::on_flag(boma.object(), vars::pickel::instance::IS_IN_TUMBLE);
        ColorBlendModule::set_main_color(boma, /* Brightness */ &cbm_t_vec1, /* Diffuse */ &cbm_t_vec2, 0.21, 2.2, 3, /* Display Color */ true);
        VarModule::off_flag(boma.object(), vars::pickel::instance::TUMBLE_START);
    }

}

// this makes it easier to place blocks diagonally down during build-walk
unsafe fn buildwalk_crouch_disable(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if [*FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_WALK, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_WALK_BACK].contains(&status_kind) {
        WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT);
    }
}

unsafe fn build_ecb_shift(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if [*FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_FALL,
        *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_FALL_AERIAL,
        *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_JUMP,
        *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_JUMP_AERIAL,
        *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_WAIT,
        *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_WALK,
        *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_WALK_BACK].contains(&status_kind)
    && !VarModule::is_flag(boma.object(), vars::common::status::DISABLE_ECB_SHIFT)
    {
        VarModule::on_flag(boma.object(), vars::common::status::DISABLE_ECB_SHIFT);
    }
}

// Logging for deciphering ACMD scripts
unsafe fn logging_for_acmd(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {

    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3 || status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
        // println!("craft_weapon_kind: {}", WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_KIND));
        // println!("request_have_craft_weapon_kind: {}", WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND));
        // println!("craft_sword: {}", *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_SWORD);
        // println!("craft_axe: {}", *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_AXE);
        // println!("craft_pick: {}", *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_PICK);
        // println!("craft_shovel: {}", *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_SHOVEL);
    }


}

// lets steve respawn table during first 5 frames of standing mine
unsafe fn pickel_table_recreate(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    let prev_status = StatusModule::prev_status_kind(boma, 0);
    if status_kind == *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N1_WAIT // if steve is in stationary mining status
    && MotionModule::frame(boma) <= 5.0 //during first 5 frames of animation
    && ![*FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N1_WALK, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N1_WALK_BACK].contains(&prev_status)  // and is not returning to still from a walking mine
    && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) { // press shield
        StatusModule::change_status_force(boma, *FIGHTER_PICKEL_STATUS_KIND_RECREATE_TABLE, true); // ta da
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    elytra_cancel(boma, id, status_kind, situation_kind, cat[0], frame);
    hitstun_tumble_glow(boma, id, status_kind);
    //buildwalk_crouch_disable(boma, status_kind);
    build_ecb_shift(boma, status_kind);
    //logging_for_acmd(boma, status_kind);
    pickel_table_recreate(fighter, boma, status_kind);
}

#[utils::macros::opff(FIGHTER_KIND_PICKEL )]
pub fn pickel_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		pickel_frame(fighter)
    }
}

pub unsafe fn pickel_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

#[smashline::weapon_frame(agent = WEAPON_KIND_PICKEL_TROLLEY, main)]
pub fn pickel_trolley_frame(weapon: &mut smash::lua2cpp::L2CFighterBase) {
    unsafe {
        let boma = weapon.boma();
        let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        // Ensure the boma's owner is Steve
        if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_PICKEL {
            let pickel = utils::util::get_battle_object_from_id(owner_id);
            let pickel_boma = &mut *(*pickel).module_accessor;
            // Burn double jump when jumping out of Minecart
            if boma.is_situation(*SITUATION_KIND_AIR)
            && pickel_boma.is_status(*FIGHTER_PICKEL_STATUS_KIND_SPECIAL_S_JUMP) {
                if MotionModule::frame(pickel_boma) <= 1.0
                && pickel_boma.get_num_used_jumps() < pickel_boma.get_jump_count_max() {
                    WorkModule::inc_int(pickel_boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
                }
            }
            // Restore double jump when landing with Minecart
            if boma.is_situation(*SITUATION_KIND_GROUND)
            && pickel_boma.is_status(*FIGHTER_PICKEL_STATUS_KIND_SPECIAL_S_DRIVE) {
                if pickel_boma.get_num_used_jumps() >= pickel_boma.get_jump_count_max() {
                    WorkModule::dec_int(pickel_boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
                }
            }
        }
    }
}