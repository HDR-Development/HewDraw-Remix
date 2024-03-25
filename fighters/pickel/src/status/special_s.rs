use super::*;
use globals::*;
 
// FIGHTER_STATUS_KIND_SPECIAL_S

pub unsafe extern "C" fn special_s_pre(fighter: &mut L2CFighterCommon) -> L2CValue{
    let hasIron = WorkModule::get_int(fighter.module_accessor,*FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_MATERIAL_NUM_IRON) > 0;
    let trolleyArticle = ArticleModule::is_exist(fighter.module_accessor,*FIGHTER_PICKEL_GENERATE_ARTICLE_TROLLEY);
    let canCart = hasIron && !trolleyArticle;
    if canCart {
        VarModule::on_flag(fighter.battle_object, vars::pickel::instance::DISABLE_SPECIAL_S);
    }

    smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_SPECIAL_S)(fighter)
}

// Prevents side special from being used again if it has already been used once in the current airtime
unsafe extern "C" fn should_use_special_s_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR)
    && VarModule::is_flag(fighter.battle_object, vars::pickel::instance::DISABLE_SPECIAL_S) {
        false.into()
    } else {
        true.into()
    }
}

// Re-enables the ability to use side special when connecting to ground or cliff
unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    let still_SideSpecial = fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_S_JUMP,
        *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_S_RIDE,
        *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_S_DRIVE
        ]
    );

    if (!fighter.is_situation(*SITUATION_KIND_AIR) && !still_SideSpecial) 
    || fighter.is_situation(*SITUATION_KIND_CLIFF)
    || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LANDING]) {
        VarModule::off_flag(fighter.battle_object, vars::pickel::instance::DISABLE_SPECIAL_S);
    }

    return true.into();
}

unsafe extern "C" fn special_s_init(fighter: &mut L2CFighterCommon) {
    fighter.global_table[globals::USE_SPECIAL_S_CALLBACK].assign(&L2CValue::Ptr(should_use_special_s_callback as *const () as _));
    fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));
}

pub fn install(agent: &mut Agent) {
    agent.on_start(special_s_init);
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_pre);
}