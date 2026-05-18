use super::*;

hitbox_templates!(
    pub KROOL_KROWN = {
        extends: PHYSICAL_PROJECTILE_HITBOX,
        effect: "collision_attr_normal",
        hit_sound: CollisionSound::Punch,
        region: AttackRegion::Object,
        clank: SetOff::Thru,
        sound_level: SoundLevel::L,
    };
);

unsafe extern "C" fn game_throw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        hitbox!(agent, { extends: KROOL_KROWN, id: 0, bone: "rot", dmg: 9.0, angle: 65, kbg: 79, bkb: 45, size: 3.5, x: 0.0, y: 0.0, z: 0.0, x2: -4.8, y2: -6.0, z2: 0.0, facing: LrCheck::F, rehit: 44, });
        AttackModule::enable_safe_pos(boma);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        hitbox!(agent, { extends: KROOL_KROWN, id: 0, bone: "rot", dmg: 9.0, angle: 65, kbg: 79, bkb: 45, size: 3.5, x: 0.0, y: 0.0, z: 0.0, facing: LrCheck::F, rehit: 44, });
    }
    frame(lua_state, 39.0);
    if is_excute(agent) {
        hitbox!(agent, { extends: KROOL_KROWN, id: 0, bone: "rot", dmg: 7.0, angle: 65, kbg: 79, bkb: 45, size: 3.5, x: 0.0, y: 0.0, z: 0.0, facing: LrCheck::B, rehit: 44, });
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_throw", game_throw, Priority::Low);
}