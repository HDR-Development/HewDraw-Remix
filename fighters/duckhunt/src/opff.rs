// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn duck_jump_cancel(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_DUCKHUNT_STATUS_KIND_SPECIAL_HI_FLY)
    && fighter.motion_frame() > 20.0
    && fighter.is_cat_flag(Cat1::SpecialHi) {
        fighter.change_status_req(*FIGHTER_DUCKHUNT_STATUS_KIND_SPECIAL_HI_END, true);
    }
}

unsafe fn frame_data(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_N) {
        if fighter.motion_frame() <= 16.0 {
            fighter.set_rate(2.0);
        }
        if fighter.motion_frame() > 16.0 {
            fighter.set_rate(1.0);
        }
    }
}

#[utils::macros::opff(FIGHTER_KIND_DUCKHUNT )]
pub fn duckhunt_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
        duck_jump_cancel(fighter);
        frame_data(fighter);
    }
}

#[smashline::weapon_frame_callback]
pub fn gunman_callback(weapon: &mut smash::lua2cpp::L2CFighterBase) {
    unsafe { 
        if weapon.kind() != WEAPON_KIND_DUCKHUNT_GUNMAN {
            return
        }
        let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        if weapon.is_status(*WEAPON_DUCKHUNT_GUNMAN_STATUS_KIND_READY) {
            let duckhunt = utils::util::get_battle_object_from_id(owner_id);
            let duckhunt_boma = &mut *(*duckhunt).module_accessor;
            if duckhunt_boma.is_cat_flag(Cat1::SpecialLw) && duckhunt_boma.is_button_trigger(Buttons::Special | Buttons::SpecialRaw) && WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE) > 20 {
                PLAY_STATUS(weapon, Hash40::new("se_duckhunt_special_l09"));
                WorkModule::set_int(weapon.module_accessor, 20, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
            }
        } 
    }
}

//FIGHTER_DUCKHUNT_STATUS_SPECIAL_LW_INT_GUNMAN_INIT_STATUS
//FIGHTER_DUCKHUNT_INSTANCE_WORK_ID_FLAG_GUNMAN_OK