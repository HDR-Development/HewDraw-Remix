use std::ops::{Deref, DerefMut};

use crate::*;

use app::*;

#[repr(C)]
#[derive(TypeAssert)]
#[size = 0x190]
pub struct ModuleAccessor {
    #[offset = 0x000]
    event_manager: lib::EventManager,
    #[offset = 0x028]
    posture_module: *mut PostureModule,
    #[offset = 0x030]
    status_module: *mut StatusModule,
    #[offset = 0x038]
    control_module: *mut Module,
    #[offset = 0x040]
    work_module: *mut WorkModule,
    #[offset = 0x048]
    ground_module: *mut Module,
    #[offset = 0x050]
    camera_module: *mut Module,
    #[offset = 0x058]
    kinetic_module: *mut Module,
    #[offset = 0x060]
    color_blend_module: *mut Module,
    #[offset = 0x068]
    model_module: *mut Module,
    #[offset = 0x070]
    physics_module: *mut Module,
    #[offset = 0x078]
    motion_module: *mut Module,
    #[offset = 0x080]
    stop_module: *mut Module,
    #[offset = 0x088]
    article_module: *mut Module,
    #[offset = 0x090]
    attack_module: *mut Module,
    #[offset = 0x098]
    damage_module: *mut Module,
    #[offset = 0x0A0]
    hit_module: *mut Module,
    #[offset = 0x0A8]
    combo_module: *mut Module,
    #[offset = 0x0B0]
    area_module: *mut Module,
    #[offset = 0x0B8]
    item_module: *mut Module,
    #[offset = 0x0C0]
    link_module: *mut Module,
    #[offset = 0x0C8]
    team_module: *mut Module,
    #[offset = 0x0D0]
    search_module: *mut Module,
    #[offset = 0x0D8]
    unk_module_1: *mut Module,
    #[offset = 0x0E0]
    turn_module: *mut Module,
    #[offset = 0x0E8]
    reflect_module: *mut Module,
    #[offset = 0x0F0]
    shield_module: *mut Module,
    #[offset = 0x0F8]
    reflector_module: *mut Module,
    #[offset = 0x100]
    absorber_module: *mut Module,
    #[offset = 0x108]
    jostle_module: *mut Module,
    #[offset = 0x110]
    catch_module: *mut Module,
    #[offset = 0x118]
    cancel_module: *mut CancelModule,
    #[offset = 0x120]
    unk_module_2: *mut Module, // appears to have to do with spirits
    #[offset = 0x128]
    capture_module: *mut Module,
    #[offset = 0x130]
    effect_module: *mut Module,
    #[offset = 0x138]
    sound_module: *mut Module,
    #[offset = 0x140]
    visibility_module: *mut Module,
    #[offset = 0x148]
    grab_module: *mut Module,
    #[offset = 0x150]
    slope_module: *mut Module,
    #[offset = 0x158]
    shake_module: *mut Module,
    #[offset = 0x160]
    slow_module: *mut Module,
    #[offset = 0x168]
    unk_module_3: *mut Module,
    #[offset = 0x170]
    shadow_module: *mut Module,
    #[offset = 0x178]
    motion_animcmd_module: *mut Module,
    #[offset = 0x180]
    lua_module: *mut LuaModule,
    #[offset = 0x188]
    ink_paint_module: *mut Module,
}

impl ModuleAccessor {
    pub fn event_manager(&mut self) -> &mut lib::EventManager {
        &mut self.event_manager
    }

    pub fn posture<'a>(&'a self) -> &'a mut PostureModule {
        unsafe { &mut *self.posture_module }
    }

    pub fn work<'a>(&'a self) -> &'a mut WorkModule {
        unsafe { &mut *self.work_module }
    }

    pub fn cancel<'a>(&'a self) -> &'a mut CancelModule {
        unsafe { &mut *self.cancel_module }
    }

    pub fn status<'a>(&'a self) -> &'a mut StatusModule {
        unsafe { &mut *self.status_module }
    }

    pub fn lua<'a>(&'a self) -> &'a mut LuaModule {
        unsafe { &mut *self.lua_module }
    }
}

#[repr(C)]
#[vtable_impl(BattleObjectModuleAccessor)]
#[derive(TypeAssert)]
#[size = 0x38]
pub(crate) struct BattleObjectModuleAccessorVTable {
    /// Checks if this module accessor is implemented
    ///
    /// ### Returns
    /// * `true` - The module accessor is **not** implemented
    /// * `false` - The module accessor **is** implemented
    ///
    /// ### Notes
    /// If the module accessor is not implemented, there should be no attempt to
    /// use any of its modules as if they were implemented, and there should be
    /// not attempt to cast it to an implementor's type.
    #[offset = 0x0]
    pub is_virtual: extern "C" fn(this: &BattleObjectModuleAccessor) -> bool,

    #[offset = 0x8]
    destructor: extern "C" fn(this: &mut BattleObjectModuleAccessor),
    #[offset = 0x10]
    deleter: extern "C" fn(this: &mut BattleObjectModuleAccessor),

    #[offset = 0x18]
    initialize_modules: extern "C" fn(this: &mut BattleObjectModuleAccessor, init_args: *const u64),

    #[offset = 0x20]
    finalize_modules: extern "C" fn(this: &mut BattleObjectModuleAccessor),

    #[offset = 0x28]
    start_modules: extern "C" fn(this: &mut BattleObjectModuleAccessor),

    #[offset = 0x30]
    end_modules: extern "C" fn(this: &mut BattleObjectModuleAccessor),
}

#[repr(C)]
pub struct BattleObjectModuleAccessor {
    vtable: &'static BattleObjectModuleAccessorVTable,
    battle_object_id: u32,
    padding: u32,
    module_accessor: ModuleAccessor,
}

impl Deref for BattleObjectModuleAccessor {
    type Target = ModuleAccessor;

    fn deref(&self) -> &Self::Target {
        &self.module_accessor
    }
}

impl DerefMut for BattleObjectModuleAccessor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.module_accessor
    }
}

// Temporary...

/** Module allocation sizes for Fighters
 * PostureModule => 0xD0
 * StatusModule => 0x188
 * ControlModule => 0xAF0
 * WorkModule => 0x98
 * GroundModule => 0x170
 * CameraModule => 0x58
 * KineticModule => 0xD70
 * ColorBlendModule => 0x620
 * ModelModule => 0x1D0
 * PhysicsModule => 0xD0
 * MotionModule => 0x310
 * StopModule => 0x50
 * ArticleModule => 0x260
 * AttackModule => 0x260
 * DamageModule => 0x3D0
 * HitModule => 0xC8
 * ComboModule => 0x78
 * AreaModule => 0x310
 * ItemModule => 0x70
 * LinkModule => 0x150
 * TeamModule => 0x50
 * SearchModule => 0x3A0
 * UnknownModule1 => DefaultImpl
 * TurnModule => 0x68
 * ReflectModule => 0x38
 * ShieldModule => 0xB8
 * ReflectorModule => 0xB8
 * AbsorberModule => 0xB8
 * JostleModule => 0x80
 * CatchModule => 0xA0
 * CancelModule => 0x20
 * UnknownModule2 => DefaultImpl
 * CaptureModule => 0x90
 * EffectModule => 0x108
 * SoundModule => 0x860
 * VisibilityModule => 0x88
 * GrabModule => 0x380
 * SlopeModule => 0xC8
 * ShakeModule => 0x80
 * SlowModule => 0x48
 * UnknownModule3 => 0x2b0
 * ShadowModule => 0x40
 * MotionAnimcmdModule => 0x10
 * LuaModule => 0x228
 * InkPaintModule => 0xE0
 */

#[repr(C)]
pub struct FighterModuleAccessor {
    parent: BattleObjectModuleAccessor,
    // ...
}

#[repr(C)]
pub struct WeaponModuleAccessor {
    parent: BattleObjectModuleAccessor,
    // ...
}

#[repr(C)]
pub struct ItemModuleAccessor {
    parent: BattleObjectModuleAccessor,
    // ...
}
