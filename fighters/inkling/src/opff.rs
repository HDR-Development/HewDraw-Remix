// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

const INKLING_COLORS: [Vector3f; 10] = [
    // used to tint the hitbox effects - have at least one value be 0, unless you want white ink. Values between 0 and 1.
    Vector3f {
        x: 0.758027,
        y: 0.115859,
        z: 0.04,
    }, // (orange)
    Vector3f {
        x: 0.04,
        y: 0.0608165,
        z: 0.758027,
    }, // (blue)
    Vector3f {
        x: 0.79,
        y: 0.504014,
        z: 0.04,
    }, // (yellow)
    Vector3f {
        x: 0.0,
        y: 0.7333,
        z: 0.003921,
    }, // (green)
    Vector3f {
        x: 0.758027,
        y: 0.0608165,
        z: 0.273385,
    }, // (pink)
    Vector3f {
        x: 0.08,
        y: 0.3,
        z: 0.65,
    }, // (sky blue)
    Vector3f {
        x: 1.0,
        y: 0.26667,
        z: 0.525,
    }, // (monika)
    Vector3f {
        x: 0.2549,
        y: 0.6549,
        z: 0.5098,
    }, // (splat tim)
    Vector3f {
        x: 0.79,
        y: 0.008,
        z: 0.09875,
    }, // (octoling 1)
    Vector3f {
        x: 0.76,
        y: 0.025,
        z: 0.025,
    }, // (octoling 2)
];

unsafe fn dair_splatter(boma: &mut BattleObjectModuleAccessor, motion_kind: u64, id: usize) {
    if motion_kind == hash40("attack_air_lw")
        && AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT)
    {
        let pos = Vector3f {
            x: 0.,
            y: -2.,
            z: 0.,
        };
        let rot = Vector3f {
            x: 0.,
            y: 90.,
            z: 0.,
        };
        let handle2 = EffectModule::req_on_joint(
            boma,
            Hash40::new("inkling_blaster_muzzle"),
            Hash40::new("top"),
            &pos,
            &rot,
            2.2,
            &Vector3f::zero(),
            &Vector3f::zero(),
            false,
            0,
            0,
            0,
        ) as u32;
        let costumenum =
            VarModule::get_int(boma.object(), vars::common::instance::COSTUME_SLOT_NUMBER) as usize;
        EffectModule::set_rgb(
            boma,
            handle2,
            INKLING_COLORS[costumenum].x,
            INKLING_COLORS[costumenum].y,
            INKLING_COLORS[costumenum].z,
        );
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
    if boma.is_motion(Hash40::new("special_air_s_jump_end"))
    && !StatusModule::is_changing(boma) {
        if MotionModule::frame(boma) > 6.0 {
            CancelModule::enable_cancel(boma);
        }
    }
}

unsafe fn special_cancel(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
    ]) && boma.status_frame() <= 5
        && boma.is_button_on(Buttons::Guard)
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

pub unsafe fn moveset(
    fighter: &mut L2CFighterCommon,
    boma: &mut BattleObjectModuleAccessor,
    id: usize,
    cat: [i32; 4],
    status_kind: i32,
    situation_kind: i32,
    motion_kind: u64,
    stick_x: f32,
    stick_y: f32,
    facing: f32,
    frame: f32,
) {
    dair_splatter(boma, motion_kind, id);
    roller_jump_cancel(boma);
    special_cancel(boma);
    fastfall_specials(fighter);
}

#[utils::macros::opff(FIGHTER_KIND_INKLING)]
pub fn inkling_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		inkling_frame(fighter);
    }
}

pub unsafe fn inkling_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(
            fighter,
            &mut *info.boma,
            info.id,
            info.cat,
            info.status_kind,
            info.situation_kind,
            info.motion_kind.hash,
            info.stick_x,
            info.stick_y,
            info.facing,
            info.frame,
        );
    }
}
