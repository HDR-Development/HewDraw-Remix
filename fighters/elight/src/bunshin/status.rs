use super::*;

unsafe extern "C" fn status_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NORMAL,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(0),
        false,
        0,
        0,
        0,
        0
    );
    0.into()
}

unsafe extern "C" fn status_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    weapon.fastshift(L2CValue::Ptr(status_fastshift as *const () as _))
}

unsafe extern "C" fn status_fastshift(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if WorkModule::is_flag(weapon.module_accessor, 0x21000000) {
        WorkModule::off_flag(weapon.module_accessor, 0x21000000);
        let owner = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
        let owner_module_accessor = sv_battle_object::module_accessor(owner);
        let pos_x = PostureModule::pos_x(owner_module_accessor);
        let lr = PostureModule::lr(weapon.module_accessor);
        let pos_y = PostureModule::pos_y(owner_module_accessor);
        let scale = PostureModule::scale(weapon.module_accessor);
        let pos_z = PostureModule::pos_z(weapon.module_accessor);
        PostureModule::set_pos(
            weapon.module_accessor,
            &Vector3f::new(
                pos_x + (-40.0 * scale * lr),
                pos_y,
                pos_z
            )
        );
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, 0, status_pre);
    agent.status(Main, 0, status_main);
}
