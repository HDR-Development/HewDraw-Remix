use super::*;

#[skyline::from_offset(0xb2f800)]
extern "C" fn jack_customizer(module_accessor: *mut BattleObjectModuleAccessor, customize_to: u32);

#[skyline::hook(offset = 0xb30934, inline)]
unsafe fn check_doyle_summon_dispatch_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let module_accessor = *ctx.registers[21].x.as_ref() as *mut BattleObjectModuleAccessor;
    WorkModule::off_flag(module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE_EXIST);
    jack_customizer(module_accessor, 0);
}

pub fn install() {
    skyline::install_hooks!(
        check_doyle_summon_dispatch_hook
    );
}