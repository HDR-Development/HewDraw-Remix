use super::*;

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
    let pos1 = Vector3f{x: -2.0, y: 0.0, z: 0.0};
    let pos2 = Vector3f{x: 2.0, y: 0.0, z: 0.5};
    let pos3 = Vector3f{x: 0.0, y: 0.0, z: -0.5};
    let pos4 = Vector3f{x: 2.0, y: 0.0, z: -0.5};
    EffectModule::req_follow(fighter.module_accessor, Hash40::new("samusd_win3_aura"), Hash40::new("hip"), &pos1, &Vector3f::zero(), 2.5, true, 0, 0, 0, 0, 0, false, false);
    EffectModule::req_follow(fighter.module_accessor, Hash40::new("samusd_win3_aura"), Hash40::new("clavicler"), &pos2, &Vector3f::zero(), 2.0, true, 0, 0, 0, 0, 0, false, false);
    EffectModule::req_follow(fighter.module_accessor, Hash40::new("samusd_win3_aura"), Hash40::new("kneer"), &pos3, &Vector3f::zero(), 1.70000005, true, 0, 0, 0, 0, 0, false, false);
    EffectModule::req_follow(fighter.module_accessor, Hash40::new("samusd_win3_aura"), Hash40::new("footr"), &Vector3f::zero(), &Vector3f::zero(), 2.0999999, true, 0, 0, 0, 0, 0, false, false);
    EffectModule::req_follow(fighter.module_accessor, Hash40::new("samusd_win3_aura"), Hash40::new("armr"), &Vector3f::zero(), &Vector3f::zero(), 1.89999998, true, 0, 0, 0, 0, 0, false, false);
    EffectModule::req_follow(fighter.module_accessor, Hash40::new("samusd_win3_aura"), Hash40::new("handr"), &Vector3f::zero(), &Vector3f::zero(), 2.0, true, 0, 0, 0, 0, 0, false, false);
    EffectModule::req_follow(fighter.module_accessor, Hash40::new("samusd_win3_aura"), Hash40::new("claviclel"), &pos4, &Vector3f::zero(), 2.0, true, 0, 0, 0, 0, 0, false, false);
    EffectModule::req_follow(fighter.module_accessor, Hash40::new("samusd_win3_aura"), Hash40::new("kneel"), &Vector3f::zero(), &Vector3f::zero(), 1.70000005, true, 0, 0, 0, 0, 0, false, false);
    EffectModule::req_follow(fighter.module_accessor, Hash40::new("samusd_win3_aura"), Hash40::new("footl"), &Vector3f::zero(), &Vector3f::zero(), 2.0999999, true, 0, 0, 0, 0, 0, false, false);
    EffectModule::req_follow(fighter.module_accessor, Hash40::new("samusd_win3_aura"), Hash40::new("arml"), &Vector3f::zero(), &Vector3f::zero(), 1.89999998, true, 0, 0, 0, 0, 0, false, false);
    EffectModule::req_follow(fighter.module_accessor, Hash40::new("samusd_win3_aura"), Hash40::new("handl"), &Vector3f::zero(), &Vector3f::zero(), 1.89999998, true, 0, 0, 0, 0, 0, false, false);
    float_main_common(fighter)
}

unsafe extern "C" fn float_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("samusd_win3_aura"), false, true);
    float_end_common(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, statuses::samusd::FLOAT, float_pre);
    agent.status(Main, statuses::samusd::FLOAT, float_main);
    agent.status(End, statuses::samusd::FLOAT, float_end);
}
