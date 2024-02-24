use super::*;
use crate::consts::*;
use crate::consts::globals::*;
use std::ops::{Deref, DerefMut};

#[repr(C)]
pub struct FighterKineticEnergyStop {
    parent: super::energy::KineticEnergy,
    padding: u64,
    damage_target_speed: PaddedVec2,
    reset_type: EnergyStopResetType,
    elapsed_hitstop_frames: f32,
    hitstop_frames: f32,
    _xAC: f32,
    _xB0: f32,
    should_sync_damage_speed: bool,
    needs_to_sync_damage_speed: bool,
    should_start_interpolation: bool,
    interpolation_frames_remaining: u8,
    _xB8: u8,
    is_target_pos: bool,
    _xBA: bool,
    _xBB: bool,
    _xBC: u32,
    _xC0: PaddedVec2
    // ...
}

impl Deref for FighterKineticEnergyStop {
    type Target = super::energy::KineticEnergy;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for FighterKineticEnergyStop {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl FighterKineticEnergyStop {
    pub fn get_parent_sum_speed_correct(boma: &mut BattleObjectModuleAccessor, link_no: i32, arg: i32) -> PaddedVec2 {
        unsafe {
            let func: extern "C" fn(&mut BattleObjectModuleAccessor, i32, i32) -> energy::Vec3 = std::mem::transmute(LinkModule::get_parent_sum_speed as *const ());
            let vec = func(boma, link_no, arg);
            PaddedVec2::new(vec.x, vec.y)
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum EnergyStopResetType {
    Ground = 0x0,
    DamageGround,
    DamageAir,
    DamageAirIce,
    DamageOther,
    DamageKnockBack,
    GlidLanding,
    Air,
    AirXNormalMax,
    AirEscape,
    AirBrake,
    AirBrakeAlways,
    GuardDamage,
    Capture,
    CatchCut,
    ItemSwingDash,
    ItemDashThrow,
    SwimBrake,
    Run,
    RunBrake,
    GlidStart,
    CatchDash,
    ShieldRebound,
    Free,
    CaptureBeetle,
    AirLassoHang,
    AirLassoRewind,
    EscapeAirSlide,
    DamageGroundOrbit,
    DamageAirOrbit,
}

#[skyline::hook(offset = 0x6d6650)]
unsafe fn update_stop(energy: &mut FighterKineticEnergyStop, boma: &mut BattleObjectModuleAccessor) {
    use EnergyStopResetType::*;

    let backup_brake = energy.speed_brake;

    // <HDR>

    // Double traction while above max walk speed
    if StatusModule::status_kind(boma) <= 0x1DB  // only affects common statuses
    && boma.is_situation(*SITUATION_KIND_GROUND)
    && !boma.is_prev_situation(*SITUATION_KIND_AIR) {
        let mut damage_energy = KineticModule::get_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE) as *mut app::KineticEnergy;
        let damage_speed_x = app::lua_bind::KineticEnergy::get_speed_x(damage_energy);
        // If our speed is being influenced by knockback, we handle double traction elsewhere
        if damage_speed_x == 0.0 {
            let walk_speed_max =  WorkModule::get_param_float(boma, smash::hash40("walk_speed_max"), 0);
            if matches!(energy.reset_type, Ground | CatchDash | DamageGround | GuardDamage | DamageGroundOrbit | ShieldRebound) {
                let speed = energy.get_speed();
                let adjusted_speed = energy::KineticEnergy::adjust_speed_for_ground_normal(speed, boma);

                let magnitude = (adjusted_speed.x.powi(2) + adjusted_speed.y.powi(2)).sqrt();

                if magnitude >= walk_speed_max {
                    energy.speed_brake.x *= 2.0;
                }
            }
        }
        
    }

    // </HDR>

    call_original!(energy, boma);
    
    energy.speed_brake = backup_brake;
}

#[skyline::hook(offset = 0x6d8560)]
pub unsafe extern "Rust" fn setup_stop(energy: &mut FighterKineticEnergyStop, reset_type: EnergyStopResetType, initial_speed: &PaddedVec2, unk: u64, boma: &mut BattleObjectModuleAccessor) {
    if ( boma.is_fighter()
    &&     (boma.kind() == *FIGHTER_KIND_MEWTWO && boma.is_status(*FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_2))
        || (boma.kind() == *FIGHTER_KIND_PALUTENA && boma.is_status(*FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_2))
        || (boma.kind() == *FIGHTER_KIND_SHEIK && boma.is_status(*FIGHTER_SHEIK_STATUS_KIND_SPECIAL_HI_MOVE))
        || (boma.kind() == *FIGHTER_KIND_ZELDA && boma.is_status(*FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_2)) )
    {
        VarModule::set_float(boma.object(), vars::common::status::TELEPORT_INITIAL_SPEED_Y, initial_speed.y);
    }
    call_original!(energy, reset_type, initial_speed, unk, boma);
}

pub fn install() {
    skyline::install_hooks!(
        update_stop,
        setup_stop
    );
}