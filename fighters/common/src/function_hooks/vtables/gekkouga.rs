use super::*;

extern "C" {
    #[link_name = "gekkouga_get_sub_id"]
    fn gekkouga_get_sub_id(battle_object: *mut BattleObject) -> u32;
}

extern "Rust" {
    fn gimmick_flash(boma: &mut BattleObjectModuleAccessor);
}

unsafe extern "C" fn gekkouga_on_init(vtable: u64, fighter: &mut Fighter) {
    let object = &mut fighter.battle_object;
    let module_accessor = (*object).module_accessor;
    WorkModule::set_int(module_accessor, *BATTLE_OBJECT_ID_INVALID, 0x100000C1);
    WorkModule::set_int(module_accessor, *BATTLE_OBJECT_ID_INVALID, 0x100000C2);
    VarModule::set_int(object, vars::gekkouga::instance::SPECIAL_LW_SUMMON_SUB_COOLDOWN, 0);
}

#[skyline::hook(offset = 0xadca30)]
unsafe extern "C" fn gekkouga_frame(vtable: u64, fighter: &mut Fighter) {
    let object = &mut fighter.battle_object;
    let battle_object_slow = singletons::BattleObjectSlow() as *mut u8;
    if (*battle_object_slow.add(0x8) == 0 || *(battle_object_slow as *const u32) == 0) {
        if VarModule::countdown_int(object, vars::gekkouga::instance::SPECIAL_LW_SUMMON_SUB_COOLDOWN, 0) {
            gimmick_flash(object.boma());
        }
    }

    original!()(vtable, fighter)
}

unsafe extern "C" fn gekkouga_on_search(vtable: u64, fighter: &mut Fighter, log: u64) {
    let object = &mut fighter.battle_object;
    let module_accessor = (*object).module_accessor;
    let collision_log = *(log as *const u64).add(0x10 / 0x8);
    let collision_log = collision_log as *const CollisionLog;
    let status = StatusModule::status_kind(module_accessor);
    if status == *FIGHTER_STATUS_KIND_SPECIAL_LW {
        let opponent_id = (*collision_log).opponent_battle_object_id;
        let doll_id = gekkouga_get_sub_id(object);
        if doll_id == opponent_id {
            VarModule::on_flag(object, vars::gekkouga::status::SPECIAL_LW_TELEPORT_OK);
        }
    }
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x4fc02d8).data(gekkouga_on_init as u64);

    let _ = skyline::patching::Patch::in_text(0x4fc0438).data(gekkouga_on_search as u64);

    skyline::install_hooks!(
        gekkouga_frame
    );
}