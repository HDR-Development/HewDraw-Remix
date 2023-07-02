use super::*;
pub mod energy;
pub mod effect;
pub mod edge_slipoffs;
pub mod ledges;
pub mod get_param;
pub mod change_motion;
pub mod transition;
pub mod djcancel;
pub mod init_settings;
pub mod momentum_transfer;
pub mod directional_influence;
pub mod hitstun;
pub mod change_status;
pub mod is_flag;
pub mod controls;
pub mod jumps;
pub mod stage_hazards;
pub mod set_fighter_status_data;
pub mod attack;
pub mod collision;
pub mod camera;


#[skyline::hook(offset = 0x3a85b4, inline)]
unsafe fn run_lua_status_hook(ctx: &skyline::hooks::InlineCtx) {
    let boma = *ctx.registers[22].x.as_ref() as *mut BattleObjectModuleAccessor;

    if (*boma).is_fighter()
    && StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR
    && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND
    {
        WorkModule::set_int(boma, 0, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR);
    }
}

pub fn install() {
    energy::install();
    effect::install();
    edge_slipoffs::install();
    ledges::install();
    get_param::install();
    change_motion::install();
    transition::install();
    djcancel::install();
    init_settings::install();
    hitstun::install();
    change_status::install();
    is_flag::install();
    controls::install();
    momentum_transfer::install();
    jumps::install();
    stage_hazards::install();
    set_fighter_status_data::install();
    attack::install();
    collision::install();
    camera::install();

    unsafe {
        // Handles getting rid of the kill zoom
        const KILL_ZOOM_DATA: u32 = 0xD503201Fu32;
        skyline::patching::Patch::in_text(utils::offsets::kill_zoom_regular()).nop();
        skyline::patching::Patch::in_text(utils::offsets::kill_zoom_throw()).data(KILL_ZOOM_DATA);
        // Changes full hops to calculate vertical velocity identically to short hops
        skyline::patching::Patch::in_text(0x6d2188).data(0x52800015u32);        

        // removes phantoms
        skyline::patching::Patch::in_text(0x3e6ce8).data(0x14000012u32);
    }
    skyline::install_hooks!(
        run_lua_status_hook
    );
}
