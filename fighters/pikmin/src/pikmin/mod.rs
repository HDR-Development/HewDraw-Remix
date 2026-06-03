use super::*;
pub mod acmd;
pub mod status;

#[repr(C)]
pub struct PikminInfo {
    dmg: f32, // applied in common staling damage hook - does not affect knockback
    kbg: f32,
    shield_dmg: f32,
    hitlag: f32,
    attr: Hash40,
    attr_special: Hash40,
    sound: i32,
    angle: u64,
    color: Vector3f,
    cling_frame: i32
}

impl From<&mut BattleObjectModuleAccessor> for PikminInfo {
    fn from(weapon_boma: &mut BattleObjectModuleAccessor) -> Self {
        unsafe {
            let variation = WorkModule::get_int(weapon_boma, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
            let prefix = format!("param_pikmin_particular.{}.", variation);
            let param = |name: &str| -> String { format!("{}{}", prefix, name) };
            let battle_object = weapon_boma.get_owner_boma().object(); // olimar's battle object
            return PikminInfo {
                dmg:        ParamModule::get_float(battle_object, ParamType::Agent, &param("damage_mul")),
                kbg:        ParamModule::get_float(battle_object, ParamType::Agent, &param("kbg_mul")),
                shield_dmg: ParamModule::get_float(battle_object, ParamType::Agent, &param("shield_damage_mul")),
                angle:      ParamModule::get_int(battle_object, ParamType::Agent, &param("angle_mod")) as u64,
                hitlag:     ParamModule::get_float(battle_object, ParamType::Agent, &param("hitlag_mul")),
                attr:         Hash40::new(&ParamModule::get_string(battle_object, ParamType::Agent, &param("attr"))),
                attr_special: Hash40::new(&ParamModule::get_string(battle_object, ParamType::Agent, &param("attr_special"))),
                sound: ParamModule::get_int(battle_object, ParamType::Agent, &param("sound")),
                color: Vector3f {
                    x: ParamModule::get_float(battle_object, ParamType::Agent, &param("color_r")),
                    y: ParamModule::get_float(battle_object, ParamType::Agent, &param("color_g")),
                    z: ParamModule::get_float(battle_object, ParamType::Agent, &param("color_b"))
                },
                cling_frame: ParamModule::get_int(battle_object, ParamType::Agent, &param("cling_counter"))
            };
        }
    }
}

pub fn install() {
    let agent = &mut Agent::new("pikmin_pikmin");
    acmd::install(agent);
    status::install(agent);
    agent.install();
}
