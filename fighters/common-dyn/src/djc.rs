use smash::lua2cpp::*;
use smash::lib::*;

extern "C" {
    pub fn attack_air_main_status(fighter: &mut L2CFighterCommon) -> L2CValue;
    pub fn attack_air_main_status_loop(fighter: &mut L2CFighterCommon) -> L2CValue;
    pub fn sub_attack_air_inherit_jump_aerial_motion_uniq_process_init(fighter: &mut L2CFighterCommon) -> L2CValue;
    pub fn sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec(fighter: &mut L2CFighterCommon) -> L2CValue;
}