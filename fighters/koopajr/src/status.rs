use super::*;
use globals::*;
// status script import
 
pub fn install() {
    install_status_scripts!(
        pre_special_hi_escape,
        special_hi_escape,
        end_special_hi_escape,
        special_s_jump_init,
        special_hi_damage_end_main
    );
}

// FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_ESCAPE //

#[status_script(agent = "koopajr", status = FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_ESCAPE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn pre_special_hi_escape(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR);
    return 1.into()
}

// FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_S_JUMP

#[status_script(agent = "koopajr", status = FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_S_JUMP, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
pub unsafe fn special_s_jump_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Burn double jump when jumping out of Clown Kart Dash
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR
    && fighter.get_num_used_jumps() < fighter.get_jump_count_max() {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    }
    0.into()
}

// FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_DAMAGE_END

#[status_script(agent = "koopajr", status = FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_DAMAGE_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_hi_damage_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    original!(fighter);
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_WAIT, false);
    }
    else {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KOOPAJR_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DAMAGE_FLY) {
            fighter.change_status_req(*FIGHTER_STATUS_KIND_DAMAGE_FALL, false);
        }
        else {
            fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL, false);
        }
    }
    1.into()
}