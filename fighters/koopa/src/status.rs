use super::*;
use globals::*;

mod special_n;
mod special_s;
 
// AGENT INIT AND CALLBACKS
unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    //Remove fireball ready effect
    if fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_ENTRY,*FIGHTER_STATUS_KIND_DEAD,*FIGHTER_STATUS_KIND_REBIRTH,
        *FIGHTER_STATUS_KIND_WIN,*FIGHTER_STATUS_KIND_LOSE]) || !sv_information::is_ready_go() {
        EFFECT_OFF_KIND(fighter,Hash40::new("koopa_breath_m_fire"),false,false);
        VarModule::set_int(fighter.battle_object, vars::koopa::instance::FIREBALL_EFFECT_ID,0);
        VarModule::set_int(fighter.battle_object, vars::koopa::instance::FIREBALL_COOLDOWN_FRAME,MAX_COOLDOWN);
    }
    true.into()
}

extern "C" fn koopa_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        if fighter.kind() == *FIGHTER_KIND_KOOPA {
            fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));   

            //Grant fireball during training mode
            if is_training_mode() {
                VarModule::set_int(fighter.battle_object, vars::koopa::instance::FIREBALL_COOLDOWN_FRAME,0);
            }
            else{
                VarModule::set_int(fighter.battle_object, vars::koopa::instance::FIREBALL_COOLDOWN_FRAME,MAX_COOLDOWN);
            }
        }
    }
}

unsafe extern "C" fn attack_s4_charge_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    EFFECT_OFF_KIND(fighter, Hash40::new("sys_explosion_sign"), false, false);
    return smashline::original_status(Exit, fighter, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD)(fighter);
}

// FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_A

pub unsafe extern "C" fn exec_special_hi_a(fighter: &mut L2CFighterCommon) -> L2CValue {
    if KineticModule::get_kinetic_type(fighter.module_accessor) != *FIGHTER_KINETIC_TYPE_FALL && fighter.global_table[PREV_STATUS_KIND] == FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_G {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        GroundModule::set_cliff_check(fighter.module_accessor, app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES));
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into()
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG1) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG1SET) {
            WorkModule::inc_int(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_WORK_INT_F);
            return 0.into()
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG1SET);
        }
    }
    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_WORK_INT_F);
    return 0.into()
}

pub fn install() {
    smashline::Agent::new("koopa")
        .on_start(koopa_init)
        .status(Exit, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, attack_s4_charge_exit)
        .status(Exec, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_A, exec_special_hi_a)
        .install();

    special_n::install();
    special_s::install();
}