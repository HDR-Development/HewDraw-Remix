use super::*;

extern "Rust" {
    #[link_name = "float_pre_common"]
    fn float_pre_common(fighter: &mut L2CFighterCommon) -> L2CValue;
    #[link_name = "float_main_common"]
    fn float_main_common(fighter: &mut L2CFighterCommon) -> L2CValue;
    #[link_name = "float_end_common"]
    fn float_end_common(fighter: &mut L2CFighterCommon) -> L2CValue;
}

// statuses::reflet::FLOAT

unsafe extern "C" fn float_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    float_pre_common(fighter)
}

unsafe extern "C" fn float_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let eff_handle = EffectModule::req_follow(fighter.module_accessor, Hash40::new("reflet_catch"), Hash40::new("top"), &Vector3f{x: 0.0, y: -4.6, z: -4.8}, &Vector3f::zero(), 0.7, true, 0, 0, 0, 0, 0, false, false);
    EffectModule::set_rgb(fighter.module_accessor, eff_handle as u32, 0.0, 1.0, 0.0);  // elwind green
    let eff_handle = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_aura_light"), Hash40::new("bookc"), &Vector3f::zero(), &Vector3f::zero(), 1.5, true, 0, 0, 0, 0, 0, false, false);
    EffectModule::set_rgb(fighter.module_accessor, eff_handle as u32, 0.0, 1.0, 0.078);  // elwind green
    float_main_common(fighter)
}

unsafe extern "C" fn float_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    STOP_SE(fighter, Hash40::new("se_common_spirits_wind_loop"));
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("reflet_catch"), true, true);
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_aura_light"), true, true);
    float_end_common(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, statuses::reflet::FLOAT, float_pre);
    agent.status(Main, statuses::reflet::FLOAT, float_main);
    agent.status(End, statuses::reflet::FLOAT, float_end);
}