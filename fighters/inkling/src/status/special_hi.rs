use super::*;

pub unsafe extern "C" fn special_hi_rot_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.battle_object,vars::inkling::status::SPECIAL_HI_ATTACK);
    0.into()
}
pub unsafe extern "C" fn special_hi_rot_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let attacked = VarModule::is_flag(fighter.battle_object,vars::inkling::status::SPECIAL_HI_ATTACK);
    let sum_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if sum_y < 1.0 && !attacked{
        if VarModule::is_flag(fighter.battle_object,vars::inkling::instance::SPECIAL_HI_CAN_ATTACK){
            let fighter_ptr = fighter.global_table[0x4].get_ptr() as *mut Fighter;
            let ink_cost =  WorkModule::get_param_float(fighter.module_accessor, hash40("param_private"), hash40("ink_max"))/2.0;
            let can_max_shot = super::spend_ink(fighter,ink_cost);
            if can_max_shot {
                VarModule::on_flag(fighter.battle_object,vars::inkling::status::SPECIAL_HI_ATTACK);
                MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_GAME, Hash40::new("game_specialhiattack"), -1);
                MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EFFECT, Hash40::new("effect_specialhiattack"), -1);
                MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_SOUND, Hash40::new("sound_specialhiattack"), -1);
                MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EXPRESSION, Hash40::new("expression_specialhiattack"), -1); 
            }
        } 
        VarModule::off_flag(fighter.battle_object, vars::inkling::instance::SPECIAL_HI_CAN_ATTACK);
    }

    smashline::original_status(Exec, fighter, *FIGHTER_INKLING_STATUS_KIND_SPECIAL_HI_ROT)(fighter)
}

pub fn install(agent: &mut Agent)  {
    agent.status(Exec, *FIGHTER_INKLING_STATUS_KIND_SPECIAL_HI_ROT, special_hi_rot_exec);
    agent.status(Init, *FIGHTER_INKLING_STATUS_KIND_SPECIAL_HI_ROT, special_hi_rot_init);
}
