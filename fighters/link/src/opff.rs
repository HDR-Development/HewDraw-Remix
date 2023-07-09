// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn bow_fastfall(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32, stick_y: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
        if situation_kind == *SITUATION_KIND_AIR {
            if boma.is_cat_flag(Cat2::FallJump) && stick_y < -0.66 && KineticModule::get_sum_speed_y(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
                WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
            }
        }
    }
}

// Land cancel flags
unsafe fn land_cancel_flags(boma: &mut BattleObjectModuleAccessor, fighter_kind: i32, motion_kind: u64, status_kind: i32, situation_kind: i32) {
    // not including Link bc this stuff is handled within his status scripts
    if fighter_kind != *FIGHTER_KIND_LINK {
        //println!("land cancel flag fn test");
        //if motion_kind == hash40("special_hi") {
        if [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END /*Spin attack release*/].contains(&status_kind) && situation_kind == *SITUATION_KIND_GROUND {
            //println!(" === SPIN ATTACK LAND CANCEL FLAG SET");
            VarModule::on_flag(boma.object(), vars::common::instance::SPIN_ATTACK_LAND_CANCEL);
        }

        //println!("status_kind: {}", status_kind);
        //println!("Special Hi status_kind: {}", *FIGHTER_STATUS_KIND_SPECIAL_HI);
        //println!("Special Hi End status_kind: {}", *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END);
        //if !([hash40("special_hi"), hash40("special_air_hi")].contains(&motion_kind)) {
        if ![*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END /*Spin attack release*/, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL].contains(&status_kind){
            //println!(" === SPIN ATTACK LAND CANCEL FLAG UN-SET");
            VarModule::off_flag(boma.object(), vars::common::instance::SPIN_ATTACK_LAND_CANCEL);
        }
        /*
        // Logging to check status kinds for up B
        if [*FIGHTER_STATUS_KIND_SPECIAL_HI, 465 /*Spin attack release*/].contains(&status_kind){
            println!(" ====== IN A SPECIAL_HI STATUS");
            VarModule::off_flag(boma.object(), vars::common::instance::SPIN_ATTACK_LAND_CANCEL);
        }
        */
    }
}


// Up special drift
unsafe fn up_special_drift(boma: &mut BattleObjectModuleAccessor, fighter_kind: i32, status_kind: i32, situation_kind: i32, stick_x: f32, facing: f32, frame: f32) {
    let value_link = 0.55;
    let value_young = 0.55;
    let value_toon = 1.0;
    let value_walk = 0.25;
    let mut motion_value = value_link;

    if fighter_kind == *FIGHTER_KIND_TOONLINK {
        motion_value = value_toon;
    }

   //if status_kind == *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_HOLD {
   //    if situation_kind == *SITUATION_KIND_GROUND {
   //        if stick_x != 0.0 {
   //            let motion_vec = x_motion_vec(value_walk, stick_x);
   //            KineticModule::add_speed_outside(boma, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
   //        }
   //    }
   //}
}

// Spin attack land cancel
unsafe fn up_special_land_cancel(boma: &mut BattleObjectModuleAccessor, fighter_kind: i32, status_kind: i32) {
    let fighter_kind = boma.kind();
    // not including Link bc this stuff is handled within his status scripts
    if fighter_kind != *FIGHTER_KIND_LINK {
        //println!("LINK BOIS");
        if VarModule::is_flag(boma.object(), vars::common::instance::SPIN_ATTACK_LAND_CANCEL){
            //println!(" === CAN LAND CANCEL SPIN ATTACK");
        }
        if status_kind == *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL && VarModule::is_flag(boma.object(), vars::common::instance::SPIN_ATTACK_LAND_CANCEL) {
            //println!(" ======= Spin Attack has been LAND CANCELED");
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
        }
    }
}

#[no_mangle]
pub unsafe extern "Rust" fn links_common(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        let fighter_kind = (&mut *info.boma).kind();
        //land_cancel_flags(&mut *info.boma, info.fighter_kind, info.motion_kind.hash, info.status_kind, info.situation_kind);
        up_special_drift(&mut *info.boma, info.fighter_kind, info.status_kind, info.situation_kind, info.stick_x, info.facing, info.frame);
        //up_special_land_cancel(&mut *info.boma, info.fighter_kind, info.status_kind);
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    //bow_fastfall(boma, status_kind, situation_kind, cat[1], stick_y);
}

#[utils::macros::opff(FIGHTER_KIND_LINK )]
pub fn link_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		link_frame(fighter);
        links_common(fighter);

        // sword slightly smaller during utilt, 
        if fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_ATTACK_HI3]) {
            ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("sword1"), &Vector3f::new(0.85, 1.0, 1.0));
        }
    };
}

pub unsafe fn link_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}