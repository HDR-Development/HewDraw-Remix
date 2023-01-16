use super::*;
use globals::*;
// status script import
 
unsafe extern "C" fn when_shield(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
        && ControlModule::get_stick_y(fighter.module_accessor) < 0.3 {
        
        // side on d-pad for standard
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R)
        || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) {
            VarModule::set_int(fighter.object(), vars::packun::instance::CURRENT_STANCE, 0);
            LANDING_EFFECT(fighter, Hash40::new("sys_grass"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
            fighter.change_status(FIGHTER_STATUS_KIND_GUARD_OFF.into(),true.into());
            VarModule::on_flag(fighter.object(), vars::packun::instance::STANCE_NEED_SET_SPEEDS);
            return true.into();
        }
        
        // up on d-pad for putrid
        else if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) {
            VarModule::set_int(fighter.object(), vars::packun::instance::CURRENT_STANCE, 1);
            LANDING_EFFECT(fighter, Hash40::new("packun_poison_max"), Hash40::new("mouth"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
            fighter.change_status(FIGHTER_STATUS_KIND_GUARD_OFF.into(),true.into());
            VarModule::on_flag(fighter.object(), vars::packun::instance::STANCE_NEED_SET_SPEEDS);
            return true.into();
        }

        // down on d-pad for prickly
        else if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) {
            VarModule::set_int(fighter.object(), vars::packun::instance::CURRENT_STANCE, 2);
            LANDING_EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
            fighter.change_status(FIGHTER_STATUS_KIND_GUARD_OFF.into(),true.into());
            VarModule::on_flag(fighter.object(), vars::packun::instance::STANCE_NEED_SET_SPEEDS);
            return true.into();
        }
    }
    return false.into();
}
  
// setting the callback for shield to be used for b in shield
#[smashline::fighter_init]
fn packun_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        if smash::app::utility::get_kind(&mut *fighter.module_accessor) == *FIGHTER_KIND_PACKUN {
            fighter.global_table[0x34].assign(&L2CValue::Ptr(when_shield as *const () as _));
        }
    }
}

pub fn install() {
    install_status_scripts!(
        // none
    );
    smashline::install_agent_init_callbacks!(packun_init);
    //install_agent_resets!(packun_reset);
}

/*#[fighter_reset]
fn packun_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        if fighter.kind() == *FIGHTER_KIND_PACKUN {
            VarModule::set_int(fighter.object(), vars::packun::instance::CURRENT_STANCE, 0);
        }
    }
}*/