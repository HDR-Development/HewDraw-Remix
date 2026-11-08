#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

//pub mod acmd;

//pub mod opff;
//pub mod status;

pub mod ptrainer;

use smash::{
    lib::{
        L2CValue,
        LuaConst,
    },
    app::{
        *,
        self,
        sv_animcmd::{
            frame,
            wait
        },
        lua_bind::*
    },
    hash40,
    lib::lua_const::*,
    lua2cpp::*,
    phx::*
};
use smash_script::{
    *,
    macros::*
};
use smash_script::macros::ATTACK_ABS;
use utils::{
    *,
    util::*,
    ext::*,
    consts::*,
};
use smashline::*;

pub unsafe extern "C" fn kill_pledge_effects(poke_object: *mut BattleObject) {
    if poke_object.is_null() {
        return;
    }
    let poke_boma = &mut *(*poke_object).module_accessor;
    let poke_boma_kind = poke_boma.kind();
    if poke_boma_kind == *FIGHTER_KIND_PZENIGAME {
        let handle = VarModule::get_int(poke_object, vars::pzenigame::instance::SPECIAL_N_PLEDGE_EFFECT_HANDLE) as u32;
        EffectModule::kill(poke_boma, handle, false, false);
        VarModule::set_int(poke_object, vars::pzenigame::instance::SPECIAL_N_PLEDGE_EFFECT_HANDLE, -1);
    }
    else if poke_boma_kind == *FIGHTER_KIND_PFUSHIGISOU {
        let handle = VarModule::get_int(poke_object, vars::pfushigisou::instance::SPECIAL_N_PLEDGE_EFFECT_HANDLE) as u32;
        EffectModule::kill(poke_boma, handle, false, false);
        VarModule::set_int(poke_object, vars::pfushigisou::instance::SPECIAL_N_PLEDGE_EFFECT_HANDLE, -1);
    }
    else {
        let handle = VarModule::get_int(poke_object, vars::plizardon::instance::SPECIAL_N_PLEDGE_EFFECT_HANDLE) as u32;
        EffectModule::kill(poke_boma, handle, false, false);
        VarModule::set_int(poke_object, vars::plizardon::instance::SPECIAL_N_PLEDGE_EFFECT_HANDLE, -1);
    }
}

pub unsafe extern "C" fn update_pledge_ui(weapon: &mut L2CFighterBase, poke_object: *mut BattleObject) {
    if poke_object.is_null() {
        return;
    }
    let poke_boma = &mut *(*poke_object).module_accessor;
    if !sv_information::is_ready_go() && poke_boma.status_frame() < 1 {
        return;
    }
    let entry_id = poke_boma.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    let pledge_timer = VarModule::get_int(weapon.battle_object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_TIMER);
    let pledge_timer_max = ParamModule::get_int(poke_boma.object(), ParamType::Agent, "param_special_lw.pledge_duration_frame");
    let swap_timer = VarModule::get_int(weapon.battle_object, vars::ptrainer::instance::SPECIAL_LW_SWAP_TIMER);
    let swap_timer_max = ParamModule::get_int(poke_boma.object(), ParamType::Agent, "param_special_lw.swap_lockout_frame");
    let pledge_state = VarModule::get_int(weapon.battle_object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_STATE);
    let poke_boma_kind = poke_boma.kind();
    let disabled = (poke_boma_kind == *FIGHTER_KIND_PZENIGAME   && pledge_state == *PLEDGE_STATE_WATER)
                      || (poke_boma_kind == *FIGHTER_KIND_PFUSHIGISOU && pledge_state == *PLEDGE_STATE_GRASS)
                      || (poke_boma_kind == *FIGHTER_KIND_PLIZARDON   && pledge_state == *PLEDGE_STATE_FIRE);
    utils::ui::UiManager::set_ptrainer_meter_enable(entry_id, true);
    utils::ui::UiManager::set_ptrainer_meter_info(
        entry_id,
        pledge_timer as f32,
        pledge_timer_max as f32,
        swap_timer as f32,
        swap_timer_max as f32,
        pledge_state,
        disabled
    );
}

pub fn install() {
    let agent = &mut Agent::new("ptrainer");
    agent.install();

    ptrainer::install();
}