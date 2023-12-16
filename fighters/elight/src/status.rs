use super::*;

mod special_hi_attack;
mod special_hi_jump;
mod special_hi_finish;
mod special_hi_finish2;
mod special_hi;

/// Prevents side b from being used again in air when it has been disabled by up-b fall
unsafe extern "C" fn should_use_special_s_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) && VarModule::is_flag(fighter.battle_object, vars::elight::instance::DISABLE_SPECIAL_S) {
        false.into()
    } else {
        true.into()
    }
}

/// Prevents up b from being used again in air when it has been disabled by up-b fall
unsafe extern "C" fn should_use_special_hi_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) && VarModule::is_flag(fighter.battle_object, vars::elight::instance::DISABLE_SPECIAL_HI) {
        false.into()
    } else {
        true.into()
    }
}

/// Re-enables the ability to use aerial specials when connecting to ground or cliff
unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    let damage_statuses = &[*FIGHTER_STATUS_KIND_DAMAGE,
    *FIGHTER_STATUS_KIND_DAMAGE_AIR,
    *FIGHTER_STATUS_KIND_DAMAGE_FLY,
    *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
    *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
    *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
    *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
    *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
    *FIGHTER_STATUS_KIND_DAMAGE_FALL];


    if (fighter.is_situation(*SITUATION_KIND_GROUND) || fighter.is_situation(*SITUATION_KIND_CLIFF))
    || fighter.is_status_one_of(damage_statuses) || fighter.is_status(*FIGHTER_STATUS_KIND_LANDING){ 
        //Re-enable Mythra UpB 
        VarModule::off_flag(fighter.battle_object, vars::elight::instance::DISABLE_SPECIAL_HI);
        //Re-enable Mythra SideB
        VarModule::off_flag(fighter.battle_object, vars::elight::instance::DISABLE_SPECIAL_S);

        //Re-enable Pyra UpB
        Set_Pyra_Up_Special_Cancel(fighter,false);
    }
    if fighter.is_situation(*SITUATION_KIND_GROUND) || fighter.is_situation(*SITUATION_KIND_CLIFF)
    || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LANDING]) {
        VarModule::off_flag(fighter.battle_object, vars::elight::instance::UP_SPECIAL_FREEFALL);
    }
    return true.into();
}

unsafe extern "C" fn Set_Pyra_Up_Special_Cancel(fighter: &mut L2CFighterCommon, cancel_state: bool)
{
    //This first conditional tree is used if the player selects Pyra first
    if let Some(object_id) = Some(fighter.battle_object_id + 0x10000){
        let object = crate::util::get_battle_object_from_id(object_id);
        if !object.is_null() {
            let object = unsafe { &mut *object };
            let kind = object.kind as i32;
            if kind == *FIGHTER_KIND_EFLAME {
                if cancel_state {
                    VarModule::on_flag(object, vars::eflame::instance::DISABLE_SPECIAL_HI);
                }
                else{
                    VarModule::off_flag(object, vars::eflame::instance::DISABLE_SPECIAL_HI);
                }
                return;
            }
        }
    }
    //This is used if the player selects Mythra first
    if let Some(object_id) = Some(fighter.battle_object_id - 0x10000){
        let object = crate::util::get_battle_object_from_id(object_id);
        if !object.is_null() {
            let object = unsafe { &mut *object };
            let kind = object.kind as i32;
            if kind == *FIGHTER_KIND_EFLAME {
                if cancel_state {
                    VarModule::on_flag(object, vars::eflame::instance::DISABLE_SPECIAL_HI);
                }
                else{
                    VarModule::off_flag(object, vars::eflame::instance::DISABLE_SPECIAL_HI);
                }
            }
        }
    }
}

#[smashline::fighter_init]
fn elight_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        // set the callbacks on fighter init
        if fighter.kind() == *FIGHTER_KIND_ELIGHT {
            fighter.global_table[globals::USE_SPECIAL_S_CALLBACK].assign(&L2CValue::Ptr(should_use_special_s_callback as *const () as _));   
            fighter.global_table[globals::USE_SPECIAL_HI_CALLBACK].assign(&L2CValue::Ptr(should_use_special_hi_callback as *const () as _));   
            fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));   
        }
    }
}


pub fn install() {
    smashline::install_agent_init_callbacks!(elight_init);
    special_hi_attack::install();
    special_hi_jump::install();
    special_hi_finish::install();
    special_hi::install();
}

pub fn add_statuses() {
    special_hi_finish2::install();
}