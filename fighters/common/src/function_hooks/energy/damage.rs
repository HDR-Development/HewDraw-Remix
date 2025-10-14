use super::*;
use crate::consts::*;
use crate::consts::globals::*;


#[skyline::hook(offset = 0x6d2498, inline)]
unsafe fn hitstun_gravity_1(ctx: &mut skyline::hooks::InlineCtx) {
    let work_module = ctx.registers[19].x();
    let boma = *(work_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let air_accel_y = WorkModule::get_param_float(boma, hash40("air_accel_y"), 0);
    let hitstun_gravity_min = ParamModule::get_float((*boma).object(), ParamType::Common, "hitstun_gravity_min");
    let hitstun_gravity_max = ParamModule::get_float((*boma).object(), ParamType::Common, "hitstun_gravity_max");

    let hitstun_gravity = air_accel_y.clamp(hitstun_gravity_min, hitstun_gravity_max);

    ctx.registers_f[0].set_s(hitstun_gravity)
}

#[skyline::hook(offset = 0x6d24c0, inline)]
unsafe fn hitstun_fall_speed_1(ctx: &mut skyline::hooks::InlineCtx) {
    let work_module = ctx.registers[19].x();
    let boma = *(work_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let air_speed_y_stable = WorkModule::get_param_float(boma, hash40("air_speed_y_stable"), 0);

    ctx.registers_f[0].set_s(air_speed_y_stable)
}

#[skyline::hook(offset = 0x6c399c, inline)]
unsafe fn hitstun_gravity_2(ctx: &mut skyline::hooks::InlineCtx) {
    let boma = ctx.registers[1].x() as *mut smash::app::BattleObjectModuleAccessor;
    let air_accel_y = WorkModule::get_param_float(boma, hash40("air_accel_y"), 0);
    let hitstun_gravity_min = ParamModule::get_float((*boma).object(), ParamType::Common, "hitstun_gravity_min");
    let hitstun_gravity_max = ParamModule::get_float((*boma).object(), ParamType::Common, "hitstun_gravity_max");

    let hitstun_gravity = air_accel_y.clamp(hitstun_gravity_min, hitstun_gravity_max);

    ctx.registers_f[0].set_s(hitstun_gravity)
}

#[skyline::hook(offset = 0x6c39c4, inline)]
unsafe fn hitstun_fall_speed_2(ctx: &mut skyline::hooks::InlineCtx) {
    let boma = ctx.registers[1].x() as *mut smash::app::BattleObjectModuleAccessor;
    let air_speed_y_stable = WorkModule::get_param_float(boma, hash40("air_speed_y_stable"), 0);

    ctx.registers_f[0].set_s(air_speed_y_stable)
}

#[skyline::hook(offset = 0x6d5920, inline)]
unsafe fn hitstun_gravity_3(ctx: &mut skyline::hooks::InlineCtx) {
    let work_module = ctx.registers[20].x();
    let boma = *(work_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let air_accel_y = WorkModule::get_param_float(boma, hash40("air_accel_y"), 0);
    let hitstun_gravity_min = ParamModule::get_float((*boma).object(), ParamType::Common, "hitstun_gravity_min");
    let hitstun_gravity_max = ParamModule::get_float((*boma).object(), ParamType::Common, "hitstun_gravity_max");

    let hitstun_gravity = air_accel_y.clamp(hitstun_gravity_min, hitstun_gravity_max);

    ctx.registers_f[0].set_s(hitstun_gravity)
}

#[skyline::hook(offset = 0x6d5948, inline)]
unsafe fn hitstun_fall_speed_3(ctx: &mut skyline::hooks::InlineCtx) {
    let work_module = ctx.registers[20].x();
    let boma = *(work_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let air_speed_y_stable = WorkModule::get_param_float(boma, hash40("air_speed_y_stable"), 0);
    
    ctx.registers_f[0].set_s(air_speed_y_stable)
}

pub fn install() {
    unsafe {
        // Stubs damage_fly_top_air_accel_y and
        // damage_fly_top_speed_y_stable param pulls
        skyline::patching::Patch::in_text(0x6d2498).nop();
        skyline::patching::Patch::in_text(0x6d24c0).nop();
        skyline::patching::Patch::in_text(0x6c399c).nop();
        skyline::patching::Patch::in_text(0x6c39c4).nop();
        skyline::patching::Patch::in_text(0x6d5920).nop();
        skyline::patching::Patch::in_text(0x6d5948).nop();

        // Allows custom hitstun gravity to apply to all knockback angles
        // rather than just vertical angles
        skyline::patching::Patch::in_text(0x6d1f20).nop();
        skyline::patching::Patch::in_text(0x6d1f24).nop();
        skyline::patching::Patch::in_text(0x6d1f48).nop();
        skyline::patching::Patch::in_text(0x6d1f4c).nop();
        skyline::patching::Patch::in_text(0x6cf574).data(0x320003F8);
        skyline::patching::Patch::in_text(0x6d1624).data(0x320003F8);
        skyline::patching::Patch::in_text(0x6c3568).data(0x320003E8);
        skyline::patching::Patch::in_text(0x6c3764).data(0x320003E8);
    }
    skyline::install_hooks!(
        hitstun_gravity_1,
        hitstun_fall_speed_1,
        hitstun_gravity_2,
        hitstun_fall_speed_2,
        hitstun_gravity_3,
        hitstun_fall_speed_3
    );
}