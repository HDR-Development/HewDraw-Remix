use smash::app::{
    self,
    *,
    lua_bind::*
};
use smash::lua2cpp::*;
use smash::lib::{
    *,
    lua_const::*
};
use smash::phx::*;
use bitflags::bitflags;

pub trait Vec2Ext {
    fn new(x: f32, y: f32) -> Self where Self: Sized;
    fn zero() -> Self where Self: Sized;
}

pub trait Vec3Ext {
    fn new(x: f32, y: f32, z: f32) -> Self where Self: Sized;
    fn zero() -> Self where Self: Sized;
}

pub trait Vec4Ext {
    fn new(x: f32, y: f32, z: f32, w: f32) -> Self where Self: Sized;
    fn zero() -> Self where Self: Sized;
}

impl Vec2Ext for Vector2f {
    fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y
        }
    }

    fn zero() -> Self {
        Self::new(0.0, 0.0)
    }
}

impl Vec3Ext for Vector3f {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y,
            z
        }
    }

    fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

impl Vec4Ext for Vector4f {
    fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self {
            x,
            y,
            z,
            w
        }
    }

    fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }
}

pub enum CommandCat {
    Cat1(Cat1),
    Cat2(Cat2),
    Cat3(Cat3),
    Cat4(Cat4)
}

impl Into<CommandCat> for Cat1 {
    fn into(self) -> CommandCat {
        CommandCat::Cat1(self)
    }
}

impl Into<CommandCat> for Cat2 {
    fn into(self) -> CommandCat {
        CommandCat::Cat2(self)
    }
}

impl Into<CommandCat> for Cat3 {
    fn into(self) -> CommandCat {
        CommandCat::Cat3(self)
    }
}

impl Into<CommandCat> for Cat4 {
    fn into(self) -> CommandCat {
        CommandCat::Cat4(self)
    }
}

bitflags! {
    pub struct Cat1: i32 {
        const AttackN       = 0x1;
        const AttackS3      = 0x2;
        const AttackHi3     = 0x4;
        const AttackLw3     = 0x8;
        const AttackS4      = 0x10;
        const AttackHi4     = 0x20;
        const AttackLw4     = 0x40;
        const AttackAirN    = 0x80;
        const AttackAirF    = 0x100;
        const AttackAirB    = 0x200;
        const AttackAirHi   = 0x400;
        const AttackAirLw   = 0x800;
        const SpecialN      = 0x1000;
        const SpecialS      = 0x2000;
        const SpecialHi     = 0x4000;
        const SpecialLw     = 0x8000;
        const SpecialAny    = 0xF000;
        const Walk          = 0x10000;
        const Dash          = 0x20000;
        const Turn          = 0x40000;
        const TurnDash      = 0x80000;
        const Jump          = 0x100000;
        const JumpButton    = 0x200000;
        const AirEscape     = 0x400000;
        const Squat         = 0x800000;
        const Escape        = 0x1000000;
        const EscapeF       = 0x2000000;
        const EscapeB       = 0x4000000;
        const WallJumpLeft  = 0x8000000;
        const WallJumpRight = 0x10000000;
        const Catch         = 0x20000000;
        const NoCmd         = 0x40000000;
    }

    pub struct Cat2: i32 {
        const AppealSL            = 0x1;
        const AppealSR            = 0x2;
        const AppealHi            = 0x4;
        const AppealLw            = 0x8;
        const AppealSmash         = 0x10;
        const AppealAll           = 0x1F;
        const AttackDashAttackHi4 = 0x20;
        const FallJump            = 0x40;
        const DashAttackS4        = 0x80;
        const DamageFallToFall    = 0x100;
        const DownToDownStandFB   = 0x200;
        const DownToDownStand     = 0x400;
        const GuardToPass         = 0x800;
        const SquatToSquatF       = 0x1000;
        const SquatToSquatB       = 0x2000;
        const TurnToEscapeF       = 0x4000;
        const TurnToEscapeB       = 0x8000;
        const StickEscapeF        = 0x10000;
        const StickEscapeB        = 0x20000;
        const StickEscape         = 0x40000;
        const SpecialNReverseLR   = 0x80000;
        const ThrowF              = 0x100000;
        const ThrowB              = 0x200000;
        const ThrowHi             = 0x400000;
        const ThrowLw             = 0x800000;
        const CommonGuard         = 0x1000000;
        const AirLasso            = 0x2000000;
        const AttackN2            = 0x4000000;
        const FinalReverseLR      = 0x8000000;
    }

    pub struct Cat3: i32 {
        const ItemLightThrowFB4    = 0x1;
        const ItemLightThrowHi4    = 0x2;
        const ItemLightThrowLw4    = 0x4;
        const ItemLightThrowHi     = 0x8;
        const ItemLightThrowLw     = 0x10;
        const ItemLightDrop        = 0x20;
        const ItemLightThrowFB     = 0x40;
        const ItemLightThrowAirFB  = 0x80;
        const ItemLightThrowAirFB4 = 0x100;
        const ItemLightThrowAirHi  = 0x200;
        const ItemLightThrowAirHi4 = 0x400;
        const ItemLightThrowAirLw  = 0x800;
        const ItemLightThrowAirLw4 = 0x1000;
        const ItemLightDropAir     = 0x2000;
        const ItemHeavyThrowFB     = 0x4000;
        const ItemGetAir           = 0x8000;
        const SpecialSSmash        = 0x10000;
        const SpecialSSmashDash    = 0x20000;

        const ItemLightThrow       = 0x58;
        const ItemLightThrowAir    = 0xA80;
        const ItemLightThrow4      = 0x7;
        const ItemLightThrow4Air   = 0x1500;
        const ItemLightThrowAll    = 0x5F;
        const ItemLightThrowAirAll = 0x1F80;
    }

    pub struct Cat4: i32 {
        const SpecialNCommand       = 0x1;
        const SpecialN2Command      = 0x2;
        const SpecialSCommand       = 0x4;
        const SpecialHiCommand      = 0x8;
        const Command6N6            = 0x10;
        const Command4N4            = 0x20;
        const AttackCommand1        = 0x40;
        const SpecialHi2Command     = 0x80;
        const SuperSpecialCommand   = 0x100;
        const SuperSpecialRCommand  = 0x200;
        const SuperSpecial2Command  = 0x400;
        const SuperSpecial2RCommand = 0x800;
        const Command623NB          = 0x1000;
        const Command623Strict      = 0x2000;
        const Command623ALong       = 0x4000;
        const Command623BLong       = 0x8000;
        const Command623A           = 0x10000;
        const Command2              = 0x20000;
        const Command3              = 0x40000;
        const Command1              = 0x80000;
        const Command6              = 0x100000;
        const Command4              = 0x200000;
        const Command8              = 0x400000;
        const Command9              = 0x800000;
        const Command7              = 0x1000000;
        const Command6N6AB          = 0x2000000;
        const Command323Catch       = 0x4000000;
    }

    pub struct PadFlag: i32 {
        const AttackTrigger  = 0x1;
        const AttrckRelease  = 0x2;
        const SpecialTrigger = 0x4;
        const SpecialRelease = 0x8;
        const JumpTrigger    = 0x10;
        const JumpRelease    = 0x20;
        const GuardTrigger   = 0x40;
        const GuardRelease   = 0x80;
    }

    pub struct Buttons: i32 {
        const Attack      = 0x1;
        const Special     = 0x2;
        const Jump        = 0x4;
        const Guard       = 0x8;
        const Catch       = 0x10;
        const Smash       = 0x20;
        const JumpMini    = 0x40;
        const CStickOn    = 0x80;
        const StockShare  = 0x100;
        const AttackRaw   = 0x200;
        const AppealHi    = 0x400;
        const SpecialRaw  = 0x800;
        const AppealLw    = 0x1000;
        const AppealSL    = 0x2000;
        const AppealSR    = 0x4000;
        const FlickJump   = 0x8000;
        const GuardHold   = 0x10000;
        const SpecialRaw2 = 0x20000;

        const SpecialAll = 0x20802;
        const AttackAll = 0x201;
    }
}

impl Cat1 {
    pub fn new(boma: *mut BattleObjectModuleAccessor) -> Self {
        unsafe {
            Cat1::from_bits_unchecked(ControlModule::get_command_flag_cat(boma, 0))
        }
    }
}

impl Cat2 {
    pub fn new(boma: *mut BattleObjectModuleAccessor) -> Self {
        unsafe {
            Cat2::from_bits_unchecked(ControlModule::get_command_flag_cat(boma, 1))
        }
    }
}

impl Cat3 {
    pub fn new(boma: *mut BattleObjectModuleAccessor) -> Self {
        unsafe {
            Cat3::from_bits_unchecked(ControlModule::get_command_flag_cat(boma, 2))
        }
    }
}

impl Cat4 {
    pub fn new(boma: *mut BattleObjectModuleAccessor) -> Self {
        unsafe {
            Cat4::from_bits_unchecked(ControlModule::get_command_flag_cat(boma, 3))
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AerialKind {
    Nair,
    Fair,
    Bair,
    Uair,
    Dair
}

pub trait MainShift {
    fn main_shift(&mut self, new_main: unsafe extern "C" fn(&mut L2CFighterCommon) -> L2CValue) -> L2CValue;
}

pub trait FastShift {
    fn fast_shift(&mut self, new_main: unsafe extern "C" fn(&mut L2CFighterBase) -> L2CValue) -> L2CValue;
}

impl MainShift for L2CFighterCommon {
    fn main_shift(&mut self, new_main: unsafe extern "C" fn(&mut L2CFighterCommon) -> L2CValue) -> L2CValue {
        unsafe {
            self.sub_shift_status_main(L2CValue::Ptr(new_main as *const () as _))
        }
    }
}

impl FastShift for L2CFighterBase {
    fn fast_shift(&mut self, new_main: unsafe extern "C" fn(&mut L2CFighterBase) -> L2CValue) -> L2CValue {
        unsafe {
            self.fastshift(L2CValue::Ptr(new_main as *const () as _))
        }
    }
}

pub trait BomaExt {
    // INPUTS
    unsafe fn is_cat_flag<T: Into<CommandCat>>(&mut self, fighter_pad_cmd_flag: T) -> bool;
    unsafe fn is_cat_flag_all<T: Into<CommandCat>>(&mut self, fighter_pad_cmd_flag: T) -> bool;
    unsafe fn is_pad_flag(&mut self, pad_flag: PadFlag) -> bool;
    unsafe fn is_button_on(&mut self, buttons: Buttons) -> bool;
    unsafe fn is_button_off(&mut self, buttons: Buttons) -> bool;
    unsafe fn is_button_trigger(&mut self, buttons: Buttons) -> bool;
    unsafe fn is_button_release(&mut self, buttons: Buttons) -> bool;
    unsafe fn was_prev_button_on(&mut self, buttons: Buttons) -> bool;
    unsafe fn was_prev_button_off(&mut self, buttons: Buttons) -> bool;
    unsafe fn stick_x(&mut self) -> f32;
    unsafe fn stick_y(&mut self) -> f32;
    unsafe fn prev_stick_x(&mut self) -> f32;
    unsafe fn prev_stick_y(&mut self) -> f32;
    unsafe fn is_flick_y(&mut self, sensitivity: f32) -> bool;
    unsafe fn is_input_jump(&mut self) -> bool;
    unsafe fn get_aerial(&mut self) -> Option<AerialKind>;
    /// returns whether or not the stick x is pointed in the "forwards" direction for
    /// a character
    unsafe fn is_stick_forward(&mut self) -> bool;

    /// returns whether or not the stick x is pointed in the "backwards" direction for
    /// a character
    unsafe fn is_stick_backward(&mut self) -> bool;

    // STATE
    unsafe fn is_status(&mut self, kind: i32) -> bool;
    unsafe fn is_status_one_of(&mut self, kinds: &[i32]) -> bool;
    unsafe fn is_prev_status(&mut self, kind: i32) -> bool;
    unsafe fn is_prev_status_one_of(&mut self, kinds: &[i32]) -> bool;
    unsafe fn is_situation(&mut self, kind: i32) -> bool;
    unsafe fn is_prev_situation(&mut self, kind: i32) -> bool;
    unsafe fn is_motion(&mut self, motion: Hash40) -> bool;
    unsafe fn is_motion_one_of(&mut self, motions: &[Hash40]) -> bool;
    unsafe fn get_jump_count(&mut self) -> i32;
    unsafe fn get_jump_count_max(&mut self) -> i32;
    unsafe fn motion_frame(&mut self) -> f32;


    unsafe fn change_status_req(&mut self, kind: i32, repeat: bool) -> i32;

    // INSTANCE
    unsafe fn is_fighter(&mut self) -> bool;
    unsafe fn is_weapon(&mut self) -> bool;
    unsafe fn kind(&mut self) -> i32;
}

impl BomaExt for BattleObjectModuleAccessor {
    unsafe fn is_cat_flag<T: Into<CommandCat>>(&mut self, fighter_pad_cmd_flag: T) -> bool {
        let cat = fighter_pad_cmd_flag.into();
        match cat {
            CommandCat::Cat1(cat) => Cat1::new(self).intersects(cat),
            CommandCat::Cat2(cat) => Cat2::new(self).intersects(cat),
            CommandCat::Cat3(cat) => Cat3::new(self).intersects(cat),
            CommandCat::Cat4(cat) => Cat4::new(self).intersects(cat)
        }
    }

    unsafe fn is_cat_flag_all<T: Into<CommandCat>>(&mut self, fighter_pad_cmd_flag: T) -> bool {
        let cat = fighter_pad_cmd_flag.into();
        match cat {
            CommandCat::Cat1(cat) => Cat1::new(self).contains(cat),
            CommandCat::Cat2(cat) => Cat2::new(self).contains(cat),
            CommandCat::Cat3(cat) => Cat3::new(self).contains(cat),
            CommandCat::Cat4(cat) => Cat4::new(self).contains(cat)
        }
    }

    unsafe fn is_pad_flag(&mut self, pad_flag: PadFlag) -> bool {
        PadFlag::from_bits_unchecked(ControlModule::get_pad_flag(self)).intersects(pad_flag)
    }

    unsafe fn is_button_on(&mut self, buttons: Buttons) -> bool {
        Buttons::from_bits_unchecked(ControlModule::get_button(self)).intersects(buttons)
    }

    unsafe fn is_button_off(&mut self, buttons: Buttons) -> bool {
        !self.is_button_on(buttons)
    }

    unsafe fn is_button_trigger(&mut self, buttons: Buttons) -> bool {
        Buttons::from_bits_unchecked(ControlModule::get_trigger(self)).intersects(buttons)
    }

    unsafe fn is_button_release(&mut self, buttons: Buttons) -> bool {
        Buttons::from_bits_unchecked(ControlModule::get_release(self)).intersects(buttons)
    }

    unsafe fn was_prev_button_on(&mut self, buttons: Buttons) -> bool {
        Buttons::from_bits_unchecked(ControlModule::get_button_prev(self)).intersects(buttons)
    }

    unsafe fn was_prev_button_off(&mut self, buttons: Buttons) -> bool {
        !self.was_prev_button_on(buttons)
    }

    unsafe fn stick_x(&mut self) -> f32 {
        return ControlModule::get_stick_x(self);
    }
    
    unsafe fn stick_y(&mut self) -> f32 {
        return ControlModule::get_stick_y(self);
    }
    
    unsafe fn prev_stick_x(&mut self) -> f32 {
        return ControlModule::get_stick_prev_x(self);
    }
    
    unsafe fn prev_stick_y(&mut self) -> f32 {
        return ControlModule::get_stick_prev_y(self);
    }

    unsafe fn is_input_jump(&mut self) -> bool {
        if self.is_cat_flag(Cat1::Jump) && ControlModule::is_enable_flick_jump(self) {
            WorkModule::set_int(self, 1, *FIGHTER_INSTANCE_WORK_ID_INT_STICK_JUMP_COMMAND_LIFE);
            return true;
        }

        return self.is_cat_flag(Cat1::JumpButton);
    }
        
    // TODO: Reimplement this check
    unsafe fn is_flick_y(&mut self, sensitivity: f32) -> bool {
        let stick = self.stick_y();
        let p_stick = self.prev_stick_y();

        if sensitivity < 0.0 && stick < sensitivity && (stick < p_stick || self.is_cat_flag(Cat2::FallJump)) {
            return true;
        }

        if sensitivity > 0.0 && stick > sensitivity && (stick > p_stick || self.is_cat_flag(Cat2::FallJump)) {
            return true;
        }

        return false;
    }

    /// returns whether or not the stick x is pointed in the "forwards" direction for
    /// a character
    unsafe fn is_stick_forward(&mut self) -> bool{
        let stick_value_x = ControlModule::get_stick_x(self);
        if stick_value_x != 0. {
            if stick_value_x*PostureModule::lr(self) > 0. {
                return true;
            }
        }
        return false;
    }

    /// returns whether or not the stick x is pointed in the "backwards" direction for
    /// a character
    unsafe fn is_stick_backward(&mut self) -> bool{
        let stick_value_x = ControlModule::get_stick_x(self);
        if stick_value_x != 0. {
            if stick_value_x*PostureModule::lr(self) < 0. {
                return true;
            }
        }
        return false;
    }

    unsafe fn get_aerial(&mut self) -> Option<AerialKind> {
        if self.is_cat_flag(Cat1::AttackHi3 | Cat1::AttackHi4) {
            Some(AerialKind::Uair)
        } else if self.is_cat_flag(Cat1::AttackLw3 | Cat1::AttackLw4) {
            Some(AerialKind::Dair)
        } else if self.is_cat_flag(Cat1::AttackS3 | Cat1::AttackS4) {
            if self.is_stick_backward() {
                Some(AerialKind::Bair)
            } else {
                Some(AerialKind::Fair)
            }
        } else if self.is_cat_flag(Cat1::AttackN | Cat1::AttackAirN) {
            Some(AerialKind::Nair)
        } else {
            None
        }
    }

    unsafe fn is_status(&mut self, kind: i32) -> bool {
        return StatusModule::status_kind(self) == kind;
    }

    unsafe fn is_status_one_of(&mut self, kinds: &[i32]) -> bool {
        let kind = StatusModule::status_kind(self);
        return kinds.contains(&kind);
    }

    unsafe fn is_prev_status(&mut self, kind: i32) -> bool {
        return StatusModule::prev_status_kind(self, 0) == kind;
    }

    unsafe fn is_prev_status_one_of(&mut self, kinds: &[i32]) -> bool {
        let kind = StatusModule::prev_status_kind(self, 0);
        return kinds.contains(&kind);
    }

    unsafe fn is_situation(&mut self, kind: i32) -> bool {
        return StatusModule::situation_kind(self) == kind;
    }

    unsafe fn is_prev_situation(&mut self, kind: i32) -> bool {
        return StatusModule::prev_situation_kind(self) == kind;
    }

    unsafe fn is_motion(&mut self, kind: Hash40) -> bool {
        return MotionModule::motion_kind(self) == kind.hash;
    }

    unsafe fn is_motion_one_of(&mut self, kinds: &[Hash40]) -> bool {
        let kind = MotionModule::motion_kind(self);
        return kinds.contains(&Hash40::new_raw(kind));
    }

    unsafe fn motion_frame(&mut self) -> f32 {
        return MotionModule::frame(self);
    }

    unsafe fn change_status_req(&mut self, kind: i32, repeat: bool) -> i32 {
        return StatusModule::change_status_request_from_script(self, kind, repeat) as i32;
    }

    unsafe fn is_fighter(&mut self) -> bool {
        return smash::app::utility::get_category(self) == *BATTLE_OBJECT_CATEGORY_FIGHTER;
    }

    unsafe fn is_weapon(&mut self) -> bool {
        return smash::app::utility::get_category(self) == *BATTLE_OBJECT_CATEGORY_WEAPON;
    }

    unsafe fn kind(&mut self) -> i32 {
        return smash::app::utility::get_kind(self);
    }

    unsafe fn get_jump_count(&mut self) -> i32 {
        return WorkModule::get_int(self, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    }

    unsafe fn get_jump_count_max(&mut self) -> i32 {
        return WorkModule::get_int(self, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
    }
}

pub trait LuaUtil {
    // kinetic
    unsafe fn get_speed_x(&mut self, kinetic_id: i32) -> f32;
    unsafe fn get_speed_y(&mut self, kinetic_id: i32) -> f32;
    unsafe fn set_speed(&mut self, speed: Vector2f, kinetic_id: i32);
}

impl LuaUtil for L2CAgentBase {
    unsafe fn get_speed_x(&mut self, kinetic_id: i32) -> f32 {
        self.clear_lua_stack();
        smash_script::lua_args!(self, kinetic_id);
        app::sv_kinetic_energy::get_speed_x(self.lua_state_agent)
    }

    unsafe fn get_speed_y(&mut self, kinetic_id: i32) -> f32 {
        self.clear_lua_stack();
        smash_script::lua_args!(self, kinetic_id);
        app::sv_kinetic_energy::get_speed_y(self.lua_state_agent)
    }

    unsafe fn set_speed(&mut self, speed: Vector2f, kinetic_id: i32) {
        self.clear_lua_stack();
        smash_script::lua_args!(self, kinetic_id, speed.x, speed.y);
        app::sv_kinetic_energy::set_speed(self.lua_state_agent);
    }
}

pub trait GetObjects {
    unsafe fn boma(&mut self) -> &'static mut BattleObjectModuleAccessor {
        Self::get_boma(self)
    }

    unsafe fn object(&mut self) -> &'static mut BattleObject {
        Self::get_object(self)
    }

    unsafe fn get_boma(this: &mut Self) -> &'static mut BattleObjectModuleAccessor;
    unsafe fn get_object(this: &mut Self) -> &'static mut BattleObject;
}

impl GetObjects for smash::lib::L2CAgent {
    unsafe fn get_boma(this: &mut Self) -> &'static mut BattleObjectModuleAccessor {
        std::mem::transmute(this.module_accessor)
    }

    unsafe fn get_object(this: &mut Self) -> &'static mut BattleObject {
        std::mem::transmute(this.battle_object)
    }
}

impl GetObjects for BattleObject {
    unsafe fn get_boma(this: &mut Self) -> &'static mut BattleObjectModuleAccessor {
        std::mem::transmute(this.module_accessor)
    }

    unsafe fn get_object(_: &mut Self) -> &'static mut BattleObject {
        panic!("Gannot call GetObjects::get_object on BattleObject!")
    }
}

impl GetObjects for BattleObjectModuleAccessor {
    unsafe fn get_boma(_: &mut Self) -> &'static mut BattleObjectModuleAccessor {
        panic!("Gannot call GetObjects::get_boma on BattleObjectModuleAccessor!")
    }

    unsafe fn get_object(this: &mut Self) -> &'static mut BattleObject {
        std::mem::transmute(super::util::get_battle_object_from_id(this.battle_object_id))
    }
}