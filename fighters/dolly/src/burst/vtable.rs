use super::*;

// thanks wuboy

// WeaponVtableSlot::WeaponGenerate2
unsafe extern "C" fn burst_init(_vtable: u64, weapon: *mut app::Weapon, something: u64) {
    let module_accessor = (*weapon).battle_object.module_accessor;

    let motion = *(something as *const u64).add(0x88 / 0x8);
    // println!("motion: {:#x}", motion);
    WorkModule::set_int64(module_accessor, motion, *WEAPON_DOLLY_BURST_INSTANCE_WORK_ID_INT_MOTION_KIND);

    let is_air = *(something as *const bool).add(0xb0);
    // println!("is_air: {}", is_air);
    WorkModule::set_flag(module_accessor, is_air, *WEAPON_DOLLY_BURST_INSTANCE_WORK_ID_FLAG_AIR);

    if [
        hash40("final2"),
        hash40("final3"),
        hash40("super_special_triple_2"),
        hash40("super_special_triple_3")
    ].contains(&motion) {
        let pos = &mut *(something as *mut smash2::cpp::simd::Vector2).add(0x98 / 0x8);
        // println!("pos: {}, {}", pos.x(), pos.y());
        GroundModule::set_shape_safe_pos(module_accessor, &Vector2f{x: pos.x(), y: pos.y()});
    }

    let atack_mul = *(something as *const f32).add(0xa8 / 0x4);
    let reaction_mul = *(something as *const f32).add(0xac / 0x4);
    WorkModule::set_float(module_accessor, atack_mul, 0x1); // WEAPON_INSTANCE_WORK_ID_FLOAT_TEAM_OWNER_ATTACK_MUL
    WorkModule::set_float(module_accessor, reaction_mul, 0x2); // WEAPON_INSTANCE_WORK_ID_FLOAT_TEAM_OWNER_ATTACK_REACTION_MUL

    let stage = smash::app::stage::get_stage_id();
    if stage == *StageID::Dolly_Stadium {
        WorkModule::on_flag(module_accessor, *WEAPON_INSTANCE_WORK_ID_FLAG_NO_DEAD);
    }
}

// WeaponVtableSlot::WeaponOnAttackMask
unsafe extern "C" fn burst_on_hit(_vtable: u64, weapon: *mut app::Weapon) -> u64 {
    let module_accessor = (*weapon).battle_object.module_accessor;

    let motion = MotionModule::motion_kind(module_accessor);

    let event = if motion == hash40("final") {
        0x1e7886b711
    }
    else if motion == hash40("final2") {
        0x1f701d1848
    }
    else if motion == hash40("final3") {
        0x1f071a28de
    }
    else if [
        hash40("super_special"),
        hash40("super_special_triple_3")
    ].contains(&motion) {
        if WorkModule::is_flag(module_accessor, 0x20000006) {
            // WEAPON_DOLLY_BURST_INSTANCE_WORK_ID_FLAG_HIT_SUPER_SPECIAL
            return 0;
        }

        let hit_slow_mag = WorkModule::get_param_int(module_accessor, hash40("param_burst"), hash40("hit_slow_mag"));
        let hit_slow_frame = WorkModule::get_param_int(module_accessor, hash40("param_burst"), hash40("hit_slow_frame"));
        SlowModule::set(module_accessor, 0, hit_slow_mag, hit_slow_frame, true, 0x50000000);
        WorkModule::on_flag(module_accessor, 0x20000006);

        0x265df8a12e
    }
    else {
        0
    };

    LinkModule::send_event_parents(module_accessor, 3, Hash40::new_raw(event));

    return 0;
}

static mut BURST_BOMA_PTR: u64 = 0;

// Inline hook at 0x97569c (installed from common).
#[no_mangle]
pub unsafe extern "C" fn burst_check_status_inner(ctx: &mut skyline::hooks::InlineCtx) {
    let module_accessor = ctx.registers[22].x() as *mut BattleObjectModuleAccessor;
    let battle_object: *mut BattleObject = utils::util::get_battle_object_from_id((*module_accessor).battle_object_id);
    BURST_BOMA_PTR = module_accessor as u64;
    let status = ctx.registers[0].w() as i32;
    if status == *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL
    && VarModule::is_flag(battle_object, vars::dolly::status::SUPER_SPECIAL_TRIPLE) {
        ctx.registers[0].set_w(0);
    }
}

// Inline hook at 0x975b70 (installed from common).
#[no_mangle]
pub unsafe extern "C" fn burst_set_motion_inner(ctx: &mut skyline::hooks::InlineCtx) {
    let module_accessor = BURST_BOMA_PTR as *mut BattleObjectModuleAccessor;
    let battle_object: *mut BattleObject = utils::util::get_battle_object_from_id((*module_accessor).battle_object_id);
    let mut motion = ctx.registers[8].x();
    // println!("motion: {:#x}", motion);
    if StatusModule::status_kind(module_accessor) == *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL
    && VarModule::is_flag(battle_object, vars::dolly::status::SUPER_SPECIAL_TRIPLE) {
        let num = VarModule::get_int(battle_object, vars::dolly::status::SUPER_SPECIAL_TRIPLE_COUNT);
        motion = match num {
            1 => hash40("super_special_triple_1"),
            2 => hash40("super_special_triple_2"),
            3 => hash40("super_special_triple_3"),
            _ => hash40("super_special")
        };
    }
    ctx.registers[8].set_x(motion);
}

pub fn install() {
    replace_weapon_vtable(
        *WEAPON_KIND_DOLLY_BURST,
        WeaponVtableSlot::WeaponGenerate2,
        burst_init as *const () as _
    );
    replace_weapon_vtable(
        *WEAPON_KIND_DOLLY_BURST,
        WeaponVtableSlot::WeaponOnAttackMask,
        burst_on_hit as *const () as _
    );
}
