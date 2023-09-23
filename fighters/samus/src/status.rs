use super::*;
mod speciallw;
mod crawl;
mod ice_attacks;
mod specialn;
mod specials;
mod missiles;

unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let current_status = StatusModule::status_kind(boma);
    if FighterStopModuleImpl::is_damage_stop(boma){
        EFFECT_OFF_KIND(fighter,Hash40::new("sys_ice"),false,false);
        if VarModule::is_flag(fighter.battle_object, vars::samus::status::ATTACK_HAS_ICE){
            VarModule::off_flag(fighter.battle_object, vars::samus::status::ATTACK_HAS_ICE);
        }
    }
    if ![
        *FIGHTER_STATUS_KIND_SQUAT_F,*FIGHTER_STATUS_KIND_SQUAT_B,*FIGHTER_STATUS_KIND_SQUAT_WAIT,
    *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP,*FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_A,*FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_G,
    *FIGHTER_STATUS_KIND_SPECIAL_LW,*FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW,
    ].contains(&current_status){
        VarModule::off_flag(fighter.battle_object, vars::samus::instance::SPECIAL_LW_CRAWL);
    }
    true.into()
}

#[smashline::fighter_init]
fn samus_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        if fighter.kind() != *FIGHTER_KIND_SAMUS {
            return;
        }
        VarModule::set_flag(fighter.battle_object, vars::samus::instance::ICE_MODE, false);
        VarModule::set_int(fighter.battle_object, vars::samus::instance::BOMB_COOLDOWN, 0);
        super::suit_effect(fighter.module_accessor, fighter.battle_object);
        fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));   
    }
}

pub fn install() {
    speciallw::install();
    specials::install();
    specialn::install();
    crawl::install();
    missiles::install();
    ice_attacks::install();
    
    smashline::install_agent_init_callbacks!(samus_init);
}