use super::*;
use globals::*;
// status script import
 
unsafe extern "C" fn when_shield(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
        && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
        && ControlModule::get_stick_y(fighter.module_accessor) < 0.3 {
        
        // when within a poison cloud, change to toxin
        if VarModule::is_flag(fighter.object(), vars::packun::status::IS_ENABLE_STANCE_CHANGE_TO_TOXIN) {
            VarModule::set_int(fighter.object(), vars::packun::instance::CURRENT_STANCE, 1);
            fighter.change_status(FIGHTER_STATUS_KIND_GUARD_OFF.into(),true.into());
            VarModule::on_flag(fighter.object(), vars::packun::instance::NEED_SET_SPEEDS);
            return true.into();
        }
        
        // when standing by a ptooie, change to brute
        else if VarModule::is_flag(fighter.object(), vars::packun::status::IS_ENABLE_STANCE_CHANGE_TO_BRUTE) {
            VarModule::set_int(fighter.object(), vars::packun::instance::CURRENT_STANCE, 2);
            fighter.change_status(FIGHTER_STATUS_KIND_GUARD_OFF.into(),true.into());
            VarModule::on_flag(fighter.object(), vars::packun::instance::NEED_SET_SPEEDS);
            return true.into();
        }

        // when in a stance, change back to normal
        else if VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE) != 0 {
            VarModule::set_int(fighter.object(), vars::packun::instance::CURRENT_STANCE, 0);
            fighter.change_status(FIGHTER_STATUS_KIND_GUARD_OFF.into(),true.into());
            VarModule::on_flag(fighter.object(), vars::packun::instance::NEED_SET_SPEEDS);
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
    install_agent_resets!(packun_reset);
}

#[fighter_reset]
fn packun_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        if fighter.kind() == *FIGHTER_KIND_PACKUN {
            VarModule::set_int(fighter.object(), vars::packun::instance::CURRENT_STANCE, 0);
            VarModule::on_flag(fighter.object(), vars::packun::instance::NEED_SET_SPEEDS);
        }
    }
}