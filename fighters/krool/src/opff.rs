// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn jetpack_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, cat1: i32) {
    if status_kind == *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI {
        let fuel = VarModule::get_int(boma.object(), vars::krool::instance::SPECIAL_HI_FUEL);
        VarModule::set_int(
            boma.object(),
            vars::krool::instance::SPECIAL_HI_FUEL,
            fuel - 3,
        );
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) || fuel <= 0 {
            StatusModule::change_status_request_from_script(
                boma,
                *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_AIR_END,
                true,
            );
        }
    } else if VarModule::get_int(boma.object(), vars::krool::instance::SPECIAL_HI_FUEL) < 540
        && boma.is_situation(*SITUATION_KIND_GROUND)
    {
        VarModule::inc_int(boma.object(), vars::krool::instance::SPECIAL_HI_FUEL);
    }
}

// K. Rool Side B Crown Item Grab
unsafe fn crownerang_item_grab(boma: &mut BattleObjectModuleAccessor, status_kind: i32, cat1: i32) {
    if [
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_KROOL_STATUS_KIND_SPECIAL_S_THROW,
    ]
    .contains(&status_kind)
    {
        //println!("K. Rool side B");
        if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
            //println!("K. Rool crown grab");
            if !ItemModule::is_have_item(boma, 0) {
                ItemModule::have_item(
                    boma,
                    app::ItemKind(*ITEM_KIND_KROOLCROWN),
                    0,
                    0,
                    false,
                    false,
                );
            }
        }
    }
}

pub unsafe fn moveset(
    fighter: &mut L2CFighterCommon,
    boma: &mut BattleObjectModuleAccessor,
    id: usize,
    cat: [i32; 4],
    status_kind: i32,
    situation_kind: i32,
    motion_kind: u64,
    stick_x: f32,
    stick_y: f32,
    facing: f32,
    frame: f32,
) {
    jetpack_cancel(boma, status_kind, cat[0]);
    //crownerang_item_grab(boma, status_kind, cat[0]);
}

#[utils::macros::opff(FIGHTER_KIND_KROOL)]
pub fn krool_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
        krool_frame(fighter)
    }
}

pub unsafe fn krool_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(
            fighter,
            &mut *info.boma,
            info.id,
            info.cat,
            info.status_kind,
            info.situation_kind,
            info.motion_kind.hash,
            info.stick_x,
            info.stick_y,
            info.facing,
            info.frame,
        );
    }
}
