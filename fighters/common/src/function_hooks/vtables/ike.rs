use super::*;
use utils::ext::*;

#[skyline::hook(offset = 0xaf9350)]
pub unsafe extern "C" fn ike_on_attack(_vtable: u64, fighter: &mut Fighter, log: u64) {
    let battle_object = &mut fighter.battle_object;
    let module_accessor = battle_object.module_accessor;
    let kind = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND);
    let status = StatusModule::status_kind(module_accessor);
    if [
        *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_END_MAX,
        *FIGHTER_KIRBY_STATUS_KIND_IKE_SPECIAL_N_END_MAX,
    ].contains(&status) {
        let hash = if kind == 6 {
            hash40("param_special_n_kirby")
        }
        else {
            hash40("param_special_n")
        };
        call_critical(module_accessor, log, *FIGHTER_KIND_IKE, hash, 1, 0, 0, 0, 0);
    }

    // new
    if kind == *FIGHTER_KIND_IKE {
        if status == *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_ATTACK
        && VarModule::is_flag(battle_object, vars::ike::status::SPECIAL_S_INSTAKILL) {
            let collision_log: &mut CollisionLog = std::mem::transmute(log as *mut u64);
            let kind = collision_log.collision_kind;
            if kind == *COLLISION_KIND_HIT as u8 {
                VarModule::on_flag(battle_object, vars::ike::status::SPECIAL_S_INSTAKILL_HIT);
                call_critical(module_accessor, log, *FIGHTER_KIND_IKE, hash40("param_special_n"), 1, 0, 0, 0, 0);
            }
        }
    }
}

#[skyline::from_offset(0x696720)]
pub fn call_critical(
    module_accessor: *mut BattleObjectModuleAccessor,
    unk: u64,
    unk2: i32,
    param_struct: u64,
    unk3: i32,
    unk4: i32,
    unk5: i32,
    unk6: i32,
    unk7: i32
) -> u64;

pub fn install() {
    skyline::install_hooks!(
        ike_on_attack,
    );
}
