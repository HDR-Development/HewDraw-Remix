// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn grab_walk(boma: &mut BattleObjectModuleAccessor, status_kind: i32, cat2: i32) {
    if status_kind == *FIGHTER_STATUS_KIND_CATCH_WAIT {
        let motion_value = 0.65;
        let mut motion_vec = Vector3f{x: 0.0, y: 0.0, z: 0.0};

        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
            motion_vec.x = motion_value;
        } else if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_S_L) {
            motion_vec.x = -motion_value;
        }
        KineticModule::add_speed_outside(boma, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
    }
}

// Snake Grenade Counter reset
unsafe fn grenade_counter_reset(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32) {
    if [*FIGHTER_STATUS_KIND_ENTRY,
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_REBIRTH].contains(&status_kind) {
        VarModule::set_int(boma.object(), vars::snake::instance::SNAKE_GRENADE_COUNTER, 0);
    }
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {

    grab_walk(boma, status_kind, cat[1]);
    grenade_counter_reset(boma, id, status_kind);
}

#[utils::macros::opff(FIGHTER_KIND_SNAKE )]
pub fn snake_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		snake_frame(fighter)
    }
}

pub unsafe fn snake_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

#[smashline::weapon_frame_callback]
pub fn c4_callback(weapon: &mut smash::lua2cpp::L2CFighterBase) {
    unsafe { 
        if weapon.kind() != WEAPON_KIND_SNAKE_C4 {
            return
        }
        if weapon.is_status(*WEAPON_SNAKE_C4_STATUS_KIND_STICK_TARGET) {
            let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
            let snake = utils::util::get_battle_object_from_id(owner_id);
            let snake_boma = &mut *(*snake).module_accessor;
            let snake_status_kind = StatusModule::status_kind(snake_boma);
            // Despawn sticky when snake dies
            if snake_status_kind == *FIGHTER_STATUS_KIND_DEAD {
                ArticleModule::remove_exist(snake_boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            }
        }
    }
}