use super::*;
use globals::*;
// status script import
 
unsafe extern "C" fn when_shield(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
        && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
        && ControlModule::get_stick_y(fighter.module_accessor) < 0.3 {
        
        // if you are alraedy holding an item and you do the barrel pull input, just cargo carry that item instead
        if (ItemModule::is_have_item(fighter.module_accessor, 0)) {
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP.into(),true.into());
            return true.into();
        }
        
        // otherwise, pull a barrel if the timer is up
        else if VarModule::get_int(fighter.battle_object, vars::common::GIMMICK_TIMER) == 0 {
            VarModule::set_int(fighter.battle_object, vars::common::GIMMICK_TIMER, 1);
            ItemModule::have_item(fighter.module_accessor, ItemKind(*ITEM_KIND_BARREL),0,0,false,false);
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP.into(),true.into());
            return true.into();
        }
    }
    return false.into();
}
  
// setting the callback for shield to be used for b in shield
#[smashline::fighter_init]
fn donkey_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        if smash::app::utility::get_kind(&mut *fighter.module_accessor) == *FIGHTER_KIND_DONKEY {
        fighter.global_table[0x34].assign(&L2CValue::Ptr(when_shield as *const () as _));
        }
    }
}

// FIGHTER_STATUS_KIND_ITEM_THROW_HEAVY

#[status_script(agent = "donkey", status = FIGHTER_STATUS_KIND_ITEM_THROW_HEAVY, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn heavy_throw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("item_heavy_throw_b") {
        PostureModule::reverse_lr(fighter.module_accessor);
    }
    return original!(fighter);
}

// FIGHTER_STATUS_KIND_SPECIAL_HI

#[status_script(agent = "donkey", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
pub unsafe fn exec_special_hi(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR && StatusModule::prev_situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        GroundModule::set_cliff_check(fighter.module_accessor, app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES));
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_HI_FLAG_YACL_DEFAULT);
    }
    if GroundModule::can_entry_cliff(fighter.module_accessor) == 1 && KineticModule::get_kinetic_type(fighter.module_accessor) == *FIGHTER_KINETIC_TYPE_FALL && fighter.global_table[STICK_Y].get_f32() > -0.66 {
        fighter.change_status(
            L2CValue::I32(*FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE),
            L2CValue::Bool(false)
        );
        return 1.into()
    }
    return 0.into()
}


pub fn install() {
    install_status_scripts!(
        heavy_throw_end,
        exec_special_hi
    );
    smashline::install_agent_init_callbacks!(donkey_init);
}