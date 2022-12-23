// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn piranhacopter_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, cat1: i32) {
    if VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE) == 0 && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_END, false);
        }
    }
}

extern "Rust" {
    fn gimmick_flash(boma: &mut BattleObjectModuleAccessor);
}

// Stance change
unsafe fn stance_change(boma: &mut BattleObjectModuleAccessor) {
    // Enable meshes for stances
    // HeadA is the normal head
	// HeadB is the poison head
	// HeadS is the spike head
    if VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE) == 0 {
        ModelModule::set_mesh_visibility(boma, Hash40::new("heada"), true);
        ModelModule::set_mesh_visibility(boma, Hash40::new("headb"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("heads"), false);
    }
    else if VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE) == 1 {
        ModelModule::set_mesh_visibility(boma, Hash40::new("headb"), true);
        ModelModule::set_mesh_visibility(boma, Hash40::new("heada"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("heads"), false);
    }
    else if VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE) == 2 {
        ModelModule::set_mesh_visibility(boma, Hash40::new("heads"), true);
        ModelModule::set_mesh_visibility(boma, Hash40::new("headb"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("heada"), false);
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

unsafe fn check_status_speed_mul(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if VarModule::is_flag(fighter.object(), vars::packun::instance::NEED_SET_SPEEDS) {
        let mul = [1.0, 0.8, 0.5];
        let stance = VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE) as usize;
        println!("{}", mul[stance]);
        apply_status_speed_mul(fighter, mul[stance]);
        VarModule::off_flag(fighter.object(), vars::packun::instance::NEED_SET_SPEEDS);
    }
}

unsafe fn apply_status_speed_mul(fighter: &mut smash::lua2cpp::L2CFighterCommon, mul: f32) {
    // set the X motion speed multiplier (where movement is baked into an anim)
    lua_bind::FighterKineticEnergyMotion::set_speed_mul(fighter.get_motion_energy(), mul);

    // set the X motion accel multiplier for control energy (used in the air, during walk, fall, etc)
    lua_bind::FighterKineticEnergyController::mul_x_accel_mul( fighter.get_controller_energy(), mul);

    // set the X motion accel multiplier for control energy (used in the air, during walk, fall, etc)
    lua_bind::FighterKineticEnergyController::mul_x_accel_add( fighter.get_controller_energy(), mul);

    // set the X speed max multiplier for control energy (used in the air, during walk, fall, etc)
    lua_bind::FighterKineticEnergyController::mul_x_speed_max(fighter.get_controller_energy(), mul);
    println!("la planta speed update");
}


pub unsafe fn moveset(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    piranhacopter_cancel(boma, status_kind, cat[0]);
    sspecial_cancel(boma, status_kind, situation_kind);
    stance_change(boma);
    check_status_speed_mul(fighter);
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
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
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
                if VarModule::get_int(packun_boma.object(), vars::packun::instance::CURRENT_STANCE) != 2 {
                    VarModule::on_flag(packun_boma.object(), vars::packun::status::IS_ENABLE_STANCE_CHANGE_TO_TOXIN);
                }
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
            
            let pos_x = PostureModule::pos_x(weapon.module_accessor);
            let pos_y = PostureModule::pos_x(weapon.module_accessor);
            let packun_pos_x = PostureModule::pos_x(packun_boma);
            let packun_pos_y = PostureModule::pos_x(packun_boma);
            if (packun_pos_x < pos_x + 10.0) && (packun_pos_x > pos_x - 10.0) && (packun_pos_y < pos_y + 10.0) && (packun_pos_y > pos_y - 10.0){
                //StatusModule::change_status_request_from_script(weapon.module_accessor, *WEAPON_PALUTENA_REFLECTIONBOARD_STATUS_KIND_BREAK, false);
                //DamageModule::add_damage(packun_boma, 10.0,0);
                if VarModule::get_int(packun_boma.object(), vars::packun::instance::CURRENT_STANCE) != 1 {
                    VarModule::on_flag(packun_boma.object(), vars::packun::status::IS_ENABLE_STANCE_CHANGE_TO_BRUTE);
                }
            }
        }
    }
}