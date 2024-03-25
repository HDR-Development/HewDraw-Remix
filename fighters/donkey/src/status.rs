use super::*;
use globals::*;
// status script import

mod item_throw_heavy;
mod special_hi;
mod special_lw;
mod catch_pull;
mod shoulder;
mod super_lift;

/*unsafe extern "C" fn when_shield(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
        && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
        && ControlModule::get_stick_y(fighter.module_accessor) < 0.3 {
        // if you are alraedy holding an item and you do the barrel pull input, just cargo carry that item instead
        if (ItemModule::is_have_item(fighter.module_accessor, 0)) {
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP.into(), true.into());
            return true.into();
        }
        // otherwise, pull a barrel if the timer is up
        else if VarModule::get_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER)
            == 0
        {
            VarModule::set_int(
                fighter.battle_object,
                vars::common::instance::GIMMICK_TIMER,
                1,
            );
            ItemModule::have_item(
                fighter.module_accessor,
                ItemKind(*ITEM_KIND_BARREL),
                0,
                0,
                false,
                false,
            );
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP.into(), true.into());
            return true.into();
        }
    }
    return false.into();
}*/

unsafe extern "C" fn status_change(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR
        || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD])
    {
        VarModule::off_flag(
            fighter.battle_object,
            vars::donkey::instance::SPECIAL_AIR_LW_USED_STALL,
        );
    }
    0.into()
}

// setting the callback for shield to be used for b in shield

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    //fighter.global_table[0x34].assign(&L2CValue::Ptr(when_shield as *const () as _));
    fighter.global_table[STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(status_change as *const () as _));
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);

    item_throw_heavy::install(agent);
    special_hi::install(agent);
    special_lw::install(agent);
    catch_pull::install(agent);
    shoulder::install(agent);
    super_lift::install(agent);
}
