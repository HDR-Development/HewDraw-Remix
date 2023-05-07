use super::*;


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
    || fighter.is_status_one_of(damage_statuses) {
        //This first conditional tree is used if the player selects Pyra first
        if let Some(object_id) = Some(fighter.battle_object_id + 0x10000){
            let object = crate::util::get_battle_object_from_id(object_id);
            if !object.is_null() {
                let object = unsafe { &mut *object };
                let kind = object.kind as i32;
                if kind == *FIGHTER_KIND_ELIGHT {
                    VarModule::off_flag(object, vars::common::instance::UP_SPECIAL_CANCEL);
                    return true.into();
                }
            }
        }
        //This is used if the player selects Mythra first
        if let Some(object_id) = Some(fighter.battle_object_id - 0x10000){
            let object = crate::util::get_battle_object_from_id(object_id);
            if !object.is_null() {
                let object = unsafe { &mut *object };
                let kind = object.kind as i32;
                if kind == *FIGHTER_KIND_ELIGHT {
                    VarModule::off_flag(object, vars::common::instance::UP_SPECIAL_CANCEL);
                    return true.into();
                }
            }
        }
    }
    return true.into();
}

#[smashline::fighter_init]
fn eflame_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        // set the callbacks on fighter init
        if fighter.kind() == *FIGHTER_KIND_EFLAME {
            fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));   
        }
    }
}


pub fn install() {
    smashline::install_agent_init_callbacks!(eflame_init);
}