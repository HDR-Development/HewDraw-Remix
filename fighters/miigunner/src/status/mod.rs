use super::*;
use globals::*;
// status script import

mod attack_air;
//mod down;
//mod landing_attack_air;
mod landing_fall_special;

mod special_hi1;
mod special_hi3;

mod special_n1;
mod special_n2;
mod special_n3;

mod special_s2;
mod special_s3;

unsafe fn set_move_customizer(fighter: &mut L2CFighterCommon, customizer: unsafe extern "C" fn(&mut L2CFighterCommon) -> L2CValue) {
    if fighter.global_table["move_customizer_set"].get_bool() {
        return;
    }

    let clone = fighter.global_table[globals::WAZA_CUSTOMIZE_CONTROL].clone();
    fighter.global_table["move_customizer_set"].assign(&L2CValue::Bool(true));
    fighter.global_table["move_customizer_original"].assign(&clone);
    fighter.global_table[globals::WAZA_CUSTOMIZE_CONTROL].assign(&L2CValue::Ptr(customizer as *const () as _));
}

unsafe fn get_original_customizer(fighter: &mut L2CFighterCommon) -> Option<unsafe extern "C" fn(&mut L2CFighterCommon) -> L2CValue> {
    let ptr = fighter.global_table["move_customizer_original"].get_ptr();
    if !ptr.is_null() {
        Some(std::mem::transmute(ptr))
    } else {
        None
    }
}

unsafe extern "C" fn move_customizer(fighter: &mut L2CFighterCommon) -> L2CValue {
    let customize_to = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_WAZA_CUSTOMIZE_TO);
    if let Some(original) = get_original_customizer(fighter) {
        original(fighter);
    }
    if customize_to == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_N_2 {
        fighter.sv_set_status_func(
            FIGHTER_STATUS_KIND_SPECIAL_N.into(),
            LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(),
            std::mem::transmute(special_n2::special_n2_main as *const ())
        );
    }
    if customize_to == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_S_2 {
        fighter.sv_set_status_func(
            FIGHTER_STATUS_KIND_SPECIAL_S.into(),
            LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(),
            std::mem::transmute(special_s2::special_s2_main as *const ())
        );
        fighter.sv_set_status_func(
            FIGHTER_STATUS_KIND_SPECIAL_S.into(),
            LUA_SCRIPT_STATUS_FUNC_STATUS_END.into(),
            std::mem::transmute(special_s2::special_s2_end as *const ())
        );
    }
    else if customize_to == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_S_3 {
        fighter.sv_set_status_func(
            FIGHTER_STATUS_KIND_SPECIAL_S.into(),
            LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(),
            std::mem::transmute(special_s3::special_s3_pre as *const ())
        );
    }
    else if customize_to == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_HI_1 {
        fighter.sv_set_status_func(
            FIGHTER_STATUS_KIND_SPECIAL_HI.into(),
            LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(),
            std::mem::transmute(special_hi1::special_hi1_pre as *const ())
        );
        fighter.sv_set_status_func(
            FIGHTER_STATUS_KIND_SPECIAL_HI.into(),
            LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(),
            std::mem::transmute(special_hi1::special_hi1_main as *const ())
        );
        fighter.sv_set_status_func(
            FIGHTER_STATUS_KIND_SPECIAL_HI.into(),
            LUA_SCRIPT_STATUS_FUNC_STATUS_END.into(),
            std::mem::transmute(special_hi1::special_hi1_end as *const ())
        );
    }
    else if customize_to == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_HI_3 {
        fighter.sv_set_status_func(
            FIGHTER_STATUS_KIND_SPECIAL_HI.into(),
            LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(),
            std::mem::transmute(special_hi3::special_hi3_pre as *const ())
        );
        fighter.sv_set_status_func(
            FIGHTER_STATUS_KIND_SPECIAL_HI.into(),
            LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS.into(),
            std::mem::transmute(special_hi3::special_hi3_exit as *const ())
        );
    }
    
    return 0.into();
}

// Prevents side special from being used if a missile is present
unsafe extern "C" fn should_use_special_s_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_CUSTOMIZE_SPECIAL_S_NO) == 1 {
        if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_STEALTHBOMB)
        || ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_STEALTHBOMB_S) {
            return false.into();
        }
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_CUSTOMIZE_SPECIAL_S_NO) == 2 {
        // Grab the stored missile ID
        let missile_object_id = VarModule::get_int(fighter.battle_object, vars::miigunner::instance::SPECIAL_S3_MISSILE_OBJECT_ID) as u32;
        // Check if the stored object ID is *actually* a Gunner missile or not.
        if sv_battle_object::is_active(missile_object_id)
        && sv_battle_object::category(missile_object_id) == *BATTLE_OBJECT_CATEGORY_WEAPON
        && sv_battle_object::kind(missile_object_id) == *WEAPON_KIND_MIIGUNNER_SUPERMISSILE {
            return false.into();
        }
    }

    return true.into();
}

unsafe extern "C" fn should_use_special_lw_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_GROUNDBOMB) {
        return false.into();
    }
    return true.into();
}

unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_GROUND) || fighter.is_situation(*SITUATION_KIND_CLIFF)
    || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_GIMMICK_SPRING_JUMP]) {
        VarModule::off_flag(fighter.battle_object, vars::miigunner::instance::BOOSTED_ATTACK_AIR_LW_AIRTIME);
        VarModule::off_flag(fighter.battle_object, vars::miigunner::instance::SPECIAL_HI1_AIR_USED);
    }
    true.into()
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    set_move_customizer(fighter, move_customizer);
    move_customizer(fighter);
    fighter.global_table[globals::USE_SPECIAL_S_CALLBACK].assign(&L2CValue::Ptr(should_use_special_s_callback as *const () as _));
    fighter.global_table[globals::USE_SPECIAL_LW_CALLBACK].assign(&L2CValue::Ptr(should_use_special_lw_callback as *const () as _));
    fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));   
}

unsafe extern "C" fn damage_fly_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_STEALTHBOMB, ArticleOperationTarget(0));
    smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_DAMAGE_FLY)(fighter)
}

unsafe extern "C" fn damage_fly_roll_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_STEALTHBOMB, ArticleOperationTarget(0));
    smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL)(fighter)
}

unsafe extern "C" fn damage_fly_meteor_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_STEALTHBOMB, ArticleOperationTarget(0));
    smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR)(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);

    attack_air::install(agent);
    //down::install(agent);
    //landing_attack_air::install(agent);
    landing_fall_special::install(agent);
    special_hi3::install(agent);
    special_n1::install(agent);
    special_n3::install(agent);
    special_s3::install(agent);
    special_s2::install(agent);

    agent.status(Main, *FIGHTER_STATUS_KIND_DAMAGE_FLY, damage_fly_main);
    agent.status(Main, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, damage_fly_roll_main);
    agent.status(Main, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, damage_fly_meteor_main);
}