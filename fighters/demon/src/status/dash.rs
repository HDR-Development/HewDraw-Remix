use super::*;
use globals::*;

pub unsafe extern "C" fn status_dash(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Dash_Sub();
    fighter.sub_shift_status_main(L2CValue::Ptr(status_dash_main as *const () as _))
}

unsafe extern "C" fn status_dash_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.status_Dash_Main_common(L2CValue::I32(0)).get_bool() {
        if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
            if fighter.global_table[PREV_STATUS_KIND] == FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP {
                if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3 != 0 {
                    let attack_stand_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_attack_step"), hash40("dash_to_attack_stand1_frame"));
                    if fighter.global_table[CURRENT_FRAME].get_i32() <= attack_stand_frame {
                        fighter.change_status(
                            L2CValue::I32(*FIGHTER_DEMON_STATUS_KIND_ATTACK_STAND_1),
                            L2CValue::Bool(true)
                        );
                        return L2CValue::I32(0);
                    }
                }
            }
        }
    }
    return L2CValue::I32(0);
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_DASH, status_dash);
}