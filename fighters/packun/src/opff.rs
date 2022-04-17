// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn piranhacopter_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, cat1: i32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_END, false);
        }
    }
}

// Spike Mesh Visibility Test
unsafe fn spike_head_mesh_test(boma: &mut BattleObjectModuleAccessor) {
    // HeadA is the normal head
	// HeadB is the poison head
	// HeadS is the spike head
	ModelModule::set_mesh_visibility(boma, Hash40::new("heada"), false);
	ModelModule::set_mesh_visibility(boma, Hash40::new("headb"), false);
	ModelModule::set_mesh_visibility(boma, Hash40::new("heads"), true);
}

extern "Rust" {
    fn gimmick_flash(boma: &mut BattleObjectModuleAccessor);
}

// Stance change
unsafe fn stance_change(boma: &mut BattleObjectModuleAccessor) {
    // Set the flag to always on for testing...
    VarModule::on_flag(boma.object(), vars::packun::IS_ENABLE_STANCE_CHANGE);
    
    // Disable brute change if not swallowing Ptooie
    if !boma.is_status(*FIGHTER_PACKUN_STATUS_KIND_SPECIAL_N_END){
        VarModule::off_flag(boma.object(), vars::packun::IS_ENABLE_STANCE_CHANGE_TO_BRUTE);
    }
    // Shield + B input any time
    if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) && (ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)) {
        if VarModule::is_flag(boma.object(), vars::packun::IS_ENABLE_STANCE_CHANGE){
            if VarModule::is_flag(boma.object(), vars::packun::IS_ENABLE_STANCE_CHANGE_TO_TOXIN){
                VarModule::set_int(boma.object(), vars::packun::CURRENT_STANCE, 1);
                VarModule::off_flag(boma.object(), vars::packun::IS_ENABLE_STANCE_CHANGE_TO_TOXIN);
                gimmick_flash(boma);
            }
            else if VarModule::is_flag(boma.object(), vars::packun::IS_ENABLE_STANCE_CHANGE_TO_BRUTE){
                VarModule::set_int(boma.object(), vars::packun::CURRENT_STANCE, 2);
                VarModule::off_flag(boma.object(), vars::packun::IS_ENABLE_STANCE_CHANGE_TO_BRUTE);
                gimmick_flash(boma);
            }
            else{
                VarModule::set_int(boma.object(), vars::packun::CURRENT_STANCE, 0);
                ModelModule::set_mesh_visibility(boma, Hash40::new("heada"), true);
                ModelModule::set_mesh_visibility(boma, Hash40::new("headb"), false);
                ModelModule::set_mesh_visibility(boma, Hash40::new("heads"), false);
                gimmick_flash(boma);
            }
        }
    }
    
    // Enable meshes for stances
    if VarModule::get_int(boma.object(), vars::packun::CURRENT_STANCE) == 1 {
        ModelModule::set_mesh_visibility(boma, Hash40::new("headb"), true);
        ModelModule::set_mesh_visibility(boma, Hash40::new("heada"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("heads"), false);
    }
    else if VarModule::get_int(boma.object(), vars::packun::CURRENT_STANCE) == 2 {
        ModelModule::set_mesh_visibility(boma, Hash40::new("heads"), true);
        ModelModule::set_mesh_visibility(boma, Hash40::new("headb"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("heada"), false);
    }
    else if VarModule::get_int(boma.object(), vars::packun::CURRENT_STANCE) == 3 {
        // Flame tint
        //ModelModule::set_color_rgb(boma, 0.5, 0.2, 0.2, app::MODEL_COLOR_TYPE{_address: *MODEL_COLOR_TYPE_NORMAL as u8});
    }
    else{
        //ModelModule::set_color_rgb(boma, 0.5, 0.2, 0.2, app::MODEL_COLOR_TYPE{_address: *MODEL_COLOR_TYPE_NORMAL as u8});
        //ModelModule::set_mesh_visibility(boma, Hash40::new("heada"), true);
    }
}

unsafe fn sspecial_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    //PM-like neutral-b canceling
    if status_kind == *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_CANCEL {
        if situation_kind == *SITUATION_KIND_AIR {
            if WorkModule::get_int(boma, *FIGHTER_PACKUN_STATUS_SPECIAL_S_WORK_INT_CANCEL_TYPE) == *FIGHTER_PACKUN_SPECIAL_S_CANCEL_TYPE_AIR_ESCAPE_AIR {
                WorkModule::set_int(boma, *FIGHTER_PACKUN_SPECIAL_S_CANCEL_TYPE_NONE, *FIGHTER_PACKUN_STATUS_SPECIAL_S_WORK_INT_CANCEL_TYPE);
                //ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
            }
        }
    }
}


pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    piranhacopter_cancel(boma, status_kind, cat[0]);
	//spike_head_mesh_test(boma);
    sspecial_cancel(boma, status_kind, situation_kind);
    stance_change(boma);
}

#[utils::macros::opff(FIGHTER_KIND_PACKUN )]
pub fn packun_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		packun_frame(fighter)
    }
}

pub unsafe fn packun_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

#[smashline::weapon_frame_callback]
pub fn poisonbreath_callback(weapon: &mut smash::lua2cpp::L2CFighterBase) {
    unsafe { 
        if weapon.kind() != WEAPON_KIND_PACKUN_POISONBREATH {
            return
        }
        if weapon.is_status(*WEAPON_PACKUN_POISONBREATH_STATUS_KIND_SHOOT) {
            let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
            let packun = utils::util::get_battle_object_from_id(owner_id);
            let packun_boma = &mut *(*packun).module_accessor;

            let pos_x = PostureModule::pos_x(weapon.module_accessor);
            let pos_y = PostureModule::pos_x(weapon.module_accessor);
            let packun_pos_x = PostureModule::pos_x(packun_boma);
            let packun_pos_y = PostureModule::pos_x(packun_boma);
            if (packun_pos_x < pos_x + 10.0) && (packun_pos_x > pos_x - 10.0) && (packun_pos_y < pos_y + 10.0) && (packun_pos_y > pos_y - 10.0){
                //StatusModule::change_status_request_from_script(weapon.module_accessor, *WEAPON_PALUTENA_REFLECTIONBOARD_STATUS_KIND_BREAK, false);
                //DamageModule::add_damage(packun_boma, 10.0,0);
                if VarModule::get_int(packun_boma.object(), vars::packun::CURRENT_STANCE) != 1 {
                    VarModule::on_flag(packun_boma.object(), vars::packun::IS_ENABLE_STANCE_CHANGE_TO_TOXIN);
                }
            }
            else{
                VarModule::off_flag(packun_boma.object(), vars::packun::IS_ENABLE_STANCE_CHANGE_TO_TOXIN);
            }
        }
    }
}

#[smashline::weapon_frame_callback]
pub fn spikeball_callback(weapon: &mut smash::lua2cpp::L2CFighterBase) {
    unsafe { 
        if weapon.kind() != WEAPON_KIND_PACKUN_SPIKEBALL {
            return
        }
        if weapon.is_status(*WEAPON_PACKUN_SPIKEBALL_STATUS_KIND_HOP) {
            let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
            let packun = utils::util::get_battle_object_from_id(owner_id);
            let packun_boma = &mut *(*packun).module_accessor;
            println!("Spike ball motion: {}", MotionModule::motion_kind(weapon.module_accessor));
            /*
            if weapon.is_motion(Hash40::new("hop")){
                DamageModule::add_damage(packun_boma, 10.0,0);
            }
            */
        }
    }
}