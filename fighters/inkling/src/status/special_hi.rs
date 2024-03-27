use super::*;
use globals::*;

pub unsafe extern "C" fn special_hi_rot_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.battle_object,vars::inkling::status::SPECIAL_HI_ATTACK);
    0.into()
}
pub unsafe extern "C" fn special_hi_rot_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let attacked = VarModule::is_flag(fighter.battle_object,vars::inkling::status::SPECIAL_HI_ATTACK);
    let sum_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if sum_y < 1.0 && !attacked{
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            let fighter_ptr = fighter.global_table[0x4].get_ptr() as *mut Fighter;
            let ink_cost = FighterSpecializer_Inkling::get_sub_ink_special_lw(fighter_ptr);
            let can_max_shot = super::spend_ink(fighter,ink_cost);
            if can_max_shot {
                VarModule::on_flag(fighter.battle_object,vars::inkling::status::SPECIAL_HI_ATTACK);
                MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_GAME, Hash40::new("game_specialhiattack"), -1);
                MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EFFECT, Hash40::new("effect_specialhiattack"), -1);
                MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_SOUND, Hash40::new("sound_specialhiattack"), -1);
                MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EXPRESSION, Hash40::new("expression_specialhiattack"), -1); 
            }
        } 
    }

    smashline::original_status(Exec, fighter, *FIGHTER_INKLING_STATUS_KIND_SPECIAL_HI_ROT)(fighter)
}

pub fn install() {
    smashline::Agent::new("inkling")
        .status(
            Exec,
            *FIGHTER_INKLING_STATUS_KIND_SPECIAL_HI_ROT,
            special_hi_rot_exec,
        )
        .status(
            Init,
            *FIGHTER_INKLING_STATUS_KIND_SPECIAL_HI_ROT,
            special_hi_rot_init,
        )
        .install();
}
