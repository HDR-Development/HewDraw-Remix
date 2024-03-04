use super::*;
use globals::*;

extern "Rust" {
    #[link_name = "float_pre_common"]
    fn float_pre_common(fighter: &mut L2CFighterCommon) -> L2CValue;
    #[link_name = "float_main_common"]
    fn float_main_common(fighter: &mut L2CFighterCommon) -> L2CValue;
    #[link_name = "float_end_common"]
    fn float_end_common(fighter: &mut L2CFighterCommon) -> L2CValue;
}

unsafe extern "C" fn float_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    float_pre_common(fighter)
}

unsafe extern "C" fn float_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    EffectModule::req_follow(fighter.module_accessor, Hash40::new("mewtwo_final_aura"), Hash40::new("hip"), &Vector3f::zero(), &Vector3f::zero(), 1.25, true, 0, 0, 0, 0, 0, false, false);

    if fighter.get_num_used_jumps() < fighter.get_jump_count_max() {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    }

    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MEWTWO_INSTANCE_WORK_ID_FLAG_SPECIAL_S_BUOYANCY);
    
    float_main_common(fighter)
}

unsafe extern "C" fn float_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("mewtwo_final_aura"), false, true);
    float_end_common(fighter)
}

pub fn install() {
    smashline::Agent::new("mewtwo")
        .status(Pre, statuses::mewtwo::FLOAT, float_pre)
        .status(Main, statuses::mewtwo::FLOAT, float_main)
        .status(End, statuses::mewtwo::FLOAT, float_end)
        .install();
}
