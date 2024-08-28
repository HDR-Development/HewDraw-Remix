// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;
use skyline::hooks::InlineCtx;

unsafe fn dair_splatter(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_motion(Hash40::new("attack_air_lw"))
    && boma.motion_frame() < 17.0
    && AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) {
        let pos = Vector3f{ x: 0.0, y: -6.5, z: 0.0 };
        let rot = Vector3f{ x: 0.0, y: 90.0, z: 0.0 };
        let handle = EffectModule::req_on_joint(boma, Hash40::new("inkling_blaster_muzzle"), Hash40::new("top"), &pos, &rot, 1.5, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0) as u32;
        let r = boma.get_float(*FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_R);
        let g = boma.get_float(*FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_G);
        let b = boma.get_float(*FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_B);
        EffectModule::set_rgb(boma, handle, r, g, b);
        EffectModule::set_rate_last(boma, 0.5);
    }
}

unsafe fn roller_jump_cancel(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status(*FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_END)
        && boma.is_situation(*SITUATION_KIND_GROUND)
        && boma.status_frame() > 10
    {
        boma.check_jump_cancel(true, false);
    }
    if boma.is_status(*FIGHTER_STATUS_KIND_SPECIAL_S)
    && boma.is_situation(*SITUATION_KIND_AIR)
    && boma.status_frame() <= 5
    && boma.is_cat_flag(Cat1::AirEscape) {
        ControlModule::reset_trigger(boma);
        StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_FALL, true);
        ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
    }
    if boma.is_motion(Hash40::new("special_air_s_jump_end"))
    && !StatusModule::is_changing(boma) {
        if MotionModule::frame(boma) > 6.0 {
            CancelModule::enable_cancel(boma);
        }
    }
}

unsafe fn ink_charge_cancel(boma: &mut BattleObjectModuleAccessor) {
    if (boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_INKLING_STATUS_KIND_SPECIAL_N_SHOOT])
    && boma.is_button_on(Buttons::Guard))
    && boma.is_situation(*SITUATION_KIND_GROUND)
    {
        boma.change_status_req(*FIGHTER_INKLING_STATUS_KIND_CHARGE_INK_START, false);
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_INKLING_STATUS_KIND_SPECIAL_N_END,
        *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_JUMP_END,
        *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_END,
        *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_STOP_WALL,
        *FIGHTER_INKLING_STATUS_KIND_SPECIAL_HI_JUMP,
        *FIGHTER_INKLING_STATUS_KIND_SPECIAL_HI_FALL,
        *FIGHTER_INKLING_STATUS_KIND_SPECIAL_HI_STOP_CEIL,
        *FIGHTER_INKLING_STATUS_KIND_SPECIAL_LW_EMPTY,
        *FIGHTER_INKLING_STATUS_KIND_SPECIAL_LW_THROW
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
        if fighter.is_flag(*FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
            if [*FIGHTER_KINETIC_TYPE_MOTION_AIR, *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE].contains(&KineticModule::get_kinetic_type(fighter.module_accessor)) {
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
                let speed_y = app::sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);

                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y, 0.0, 0.0, 0.0);
                app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
                
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                app::sv_kinetic_energy::enable(fighter.lua_state_agent);

                KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
            }
        }
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, nstick_x: f32, stick_y: f32, facing: f32, frame: f32,) {
    dair_splatter(boma);
    roller_jump_cancel(boma);
    ink_charge_cancel(boma);
    fastfall_specials(fighter);
}

pub extern "C" fn inkling_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		inkling_frame(fighter);
    }
}

pub unsafe fn inkling_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, inkling_frame_wrapper);
}
