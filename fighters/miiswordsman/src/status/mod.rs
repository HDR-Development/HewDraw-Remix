use super::*;
use globals::*;
// status script import

mod special_n2;
mod special_n3;

mod special_s1;
mod special_s2;
mod special_s3;

mod special_hi;
mod special_hi2;
mod special_hi3;

mod special_lw1;
mod special_lw2;
mod special_lw3;

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
    else if customize_to == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_N_3 {
        fighter.sv_set_status_func(
            FIGHTER_STATUS_KIND_SPECIAL_N.into(),
            LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(),
            std::mem::transmute(special_n3::special_n3_pre as *const ())
        );
        fighter.sv_set_status_func(
            FIGHTER_STATUS_KIND_SPECIAL_N.into(),
            LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(),
            std::mem::transmute(special_n3::special_n3_main as *const ())
        );
        fighter.sv_set_status_func(
            FIGHTER_STATUS_KIND_SPECIAL_N.into(),
            LUA_SCRIPT_STATUS_FUNC_STATUS_END.into(),
            std::mem::transmute(special_n3::special_n3_end as *const ())
        );
    }
    else if customize_to == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_S_2 {
        fighter.sv_set_status_func(
            FIGHTER_STATUS_KIND_SPECIAL_S.into(),
            LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(),
            std::mem::transmute(special_s2::special_s2_pre as *const ())
        );
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
        fighter.sv_set_status_func(
            FIGHTER_STATUS_KIND_SPECIAL_S.into(),
            LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(),
            std::mem::transmute(special_s3::special_s3_main as *const ())
        );
        fighter.sv_set_status_func(
            FIGHTER_STATUS_KIND_SPECIAL_S.into(),
            LUA_SCRIPT_STATUS_FUNC_STATUS_END.into(),
            std::mem::transmute(special_s3::special_s3_end as *const ())
        );
        fighter.sv_set_status_func(
            FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_LW3_END.into(),
            LUA_SCRIPT_STATUS_FUNC_INIT_STATUS.into(),
            std::mem::transmute(special_s3::special_s3_end_init as *const ())
        );
    }
    else if customize_to == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_LW_1 {
        fighter.sv_set_status_func(
            FIGHTER_STATUS_KIND_SPECIAL_LW.into(),
            LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(),
            std::mem::transmute(special_lw1::special_lw1_pre as *const ())
        );
        fighter.sv_set_status_func(
            FIGHTER_STATUS_KIND_SPECIAL_LW.into(),
            LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(),
            std::mem::transmute(special_lw1::special_lw1_main as *const ())
        );
        fighter.sv_set_status_func(
            FIGHTER_STATUS_KIND_SPECIAL_LW.into(),
            LUA_SCRIPT_STATUS_FUNC_CHECK_ATTACK.into(),
            std::mem::transmute(special_lw1::special_lw1_check_attack as *const ())
        );
        fighter.sv_set_status_func(
            FIGHTER_STATUS_KIND_SPECIAL_LW.into(),
            LUA_SCRIPT_STATUS_FUNC_STATUS_END.into(),
            std::mem::transmute(special_lw1::special_lw1_end as *const ())
        );
    }
    else if customize_to == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_LW_2 {
        fighter.sv_set_status_func(
            FIGHTER_STATUS_KIND_SPECIAL_LW.into(),
            LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS.into(),
            std::mem::transmute(special_lw2::special_lw2_exec as *const ())
        );
    }
    else if customize_to == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_LW_3 {
        fighter.sv_set_status_func(
            FIGHTER_STATUS_KIND_SPECIAL_LW.into(),
            LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(),
            std::mem::transmute(special_lw3::special_lw3_pre as *const ())
        );
        fighter.sv_set_status_func(
            FIGHTER_STATUS_KIND_SPECIAL_LW.into(),
            LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(),
            std::mem::transmute(special_lw3::special_lw3_main as *const ())
        );
        fighter.sv_set_status_func(
            FIGHTER_STATUS_KIND_SPECIAL_LW.into(),
            LUA_SCRIPT_STATUS_FUNC_STATUS_END.into(),
            std::mem::transmute(special_lw3::special_lw3_end as *const ())
        );
    }
    0.into()
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    set_move_customizer(fighter, move_customizer);
    move_customizer(fighter);
}

pub fn install(agent: &mut Agent) {
    special_s1::install(agent);
    special_s2::install(agent);
    special_s3::install(agent);

    special_hi::install(agent);
    special_hi2::install(agent);
    special_hi3::install(agent);

    special_lw3::install(agent);

    agent.on_start(on_start);
}