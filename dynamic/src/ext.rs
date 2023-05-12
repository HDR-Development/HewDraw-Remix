use smash::app::{
    self,
    *,
    lua_bind::*,
    FighterKineticEnergyMotion,
    FighterKineticEnergyController,
};
use smash::lua2cpp::*;
use smash::lib::{
    *,
    lua_const::*
};
use smash::phx::*;
use bitflags::bitflags;
use modular_bitfield::specifiers::*;
use crate::consts::globals::*;

pub trait Vec2Ext {
    fn new(x: f32, y: f32) -> Self where Self: Sized;
    fn zero() -> Self where Self: Sized;
}

pub trait Vec3Ext {
    fn new(x: f32, y: f32, z: f32) -> Self where Self: Sized;
    fn zero() -> Self where Self: Sized;
    fn mag(&self) -> f32;
    fn normalize(&self) -> Self;
}

pub trait Vec4Ext {
    fn new(x: f32, y: f32, z: f32, w: f32) -> Self where Self: Sized;
    fn zero() -> Self where Self: Sized;
}

pub trait Hash40Ext {
    fn to_hash(self) -> Hash40;
}

impl Hash40Ext for Hash40 {
    fn to_hash(self) -> Hash40 {
        self
    }
}

impl Hash40Ext for u64 {
    fn to_hash(self) -> Hash40 {
        Hash40::new_raw(self)
    }
}

impl Hash40Ext for &str {
    fn to_hash(self) -> Hash40 {
        Hash40::new(self)
    }
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

    fn mag(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    fn normalize(&self) -> Self {
        let mag = self.mag();
        Self {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag
        }
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

#[derive(Copy, Clone)]
pub enum CommandCat {
    Cat1(Cat1),
    Cat2(Cat2),
    Cat3(Cat3),
    Cat4(Cat4),
    CatHdr(CatHdr)
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

impl Into<CommandCat> for CatHdr {
    fn into(self) -> CommandCat {
        CommandCat::CatHdr(self)
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

    pub struct CatHdr: i32 {
        const TiltAttack = 0x1;
        const Wavedash = 0x2;
        const ShieldDrop = 0x4;
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
        // We leave a blank at 0x4000 because the internal control mapping will map 1 << InputKind to the button bitfield, and so our shorthop button
        // would get mapped to TiltAttack (issue #776)
        const TiltAttack  = 0x80000;
        const CStickOverride = 0x100000;

        const SpecialAll  = 0x20802;
        const AttackAll   = 0x201;
        const AppealAll   = 0x7400;
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

impl CatHdr {
    pub fn new(boma: *mut BattleObjectModuleAccessor) -> Self {
        unsafe {
            CatHdr::from_bits_unchecked(ControlModule::get_command_flag_cat(boma, 4))
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
    fn change_to_custom_status(&mut self, id: i32, clear_cat: bool, common: bool);
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

    fn change_to_custom_status(&mut self, id: i32, clear_cat: bool, common: bool) {
        use crate::CustomStatusModule;

        let kind = if common {
            CustomStatusModule::get_common_status_kind(self.battle_object, id)
        } else {
            CustomStatusModule::get_agent_status_kind(self.battle_object, id)
        };

        unsafe {
            self.change_status(kind.into(), clear_cat.into())
        }
    }
}

pub trait BomaExt {
    // INPUTS
    unsafe fn clear_commands<T: Into<CommandCat>>(&mut self, fighter_pad_cmd_flag: T);
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
    unsafe fn is_input_jump(&mut self) -> bool;
    unsafe fn get_aerial(&mut self) -> Option<AerialKind>;
    unsafe fn set_joint_rotate(&mut self, bone_name: &str, rotation: Vector3f);
    /// returns whether or not the stick x is pointed in the "forwards" direction for
    /// a character
    unsafe fn is_stick_forward(&mut self) -> bool;

    /// returns whether or not the stick x is pointed in the "backwards" direction for
    /// a character
    unsafe fn is_stick_backward(&mut self) -> bool;
    unsafe fn left_stick_x(&mut self) -> f32;
    unsafe fn left_stick_y(&mut self) -> f32;

    // STATE
    unsafe fn is_status(&mut self, kind: i32) -> bool;
    unsafe fn is_status_one_of(&mut self, kinds: &[i32]) -> bool;
    unsafe fn is_prev_status(&mut self, kind: i32) -> bool;
    unsafe fn is_prev_status_one_of(&mut self, kinds: &[i32]) -> bool;
    unsafe fn is_situation(&mut self, kind: i32) -> bool;
    unsafe fn is_prev_situation(&mut self, kind: i32) -> bool;
    unsafe fn is_motion(&mut self, motion: Hash40) -> bool;
    unsafe fn is_motion_one_of(&mut self, motions: &[Hash40]) -> bool;
    unsafe fn status(&mut self) -> i32;

    /// gets the number of jumps that have been used
    unsafe fn get_num_used_jumps(&mut self) -> i32;

    /// gets the max allowed number of jumps for this character
    unsafe fn get_jump_count_max(&mut self) -> i32;
    unsafe fn motion_frame(&mut self) -> f32;
    unsafe fn set_rate(&mut self, motion_rate: f32);
    unsafe fn is_in_hitlag(&mut self) -> bool;
    unsafe fn status_frame(&mut self) -> i32;


    unsafe fn change_status_req(&mut self, kind: i32, repeat: bool) -> i32;

    // INSTANCE
    unsafe fn is_fighter(&mut self) -> bool;
    unsafe fn is_weapon(&mut self) -> bool;
    unsafe fn kind(&mut self) -> i32;
    // gets the boma of the player who you are grabbing
    unsafe fn get_grabbed_opponent_boma(&mut self) -> &mut BattleObjectModuleAccessor;
    // gets the boma of the player who is grabbing you
    unsafe fn get_grabber_boma(&mut self) -> &mut BattleObjectModuleAccessor;

    // WORK
    unsafe fn get_int(&mut self, what: i32) -> i32;
    unsafe fn get_float(&mut self, what: i32) -> f32;
    unsafe fn get_int64(&mut self, what: i32) -> u64;
    unsafe fn is_flag(&mut self, what: i32) -> bool;
    unsafe fn set_int(&mut self, value: i32, what: i32);
    unsafe fn set_float(&mut self, value: f32, what: i32);
    unsafe fn set_int64(&mut self, value: i64, what: i32);
    unsafe fn on_flag(&mut self, what: i32);
    unsafe fn off_flag(&mut self, what: i32);
    unsafe fn get_param_int(&mut self, obj: &str, field: &str) -> i32;
    unsafe fn get_param_float(&mut self, obj: &str, field: &str) -> f32;
    unsafe fn get_param_int64(&mut self, obj: &str, field: &str) -> u64;
    unsafe fn set_int_from_param(&mut self, what: i32, object: impl Hash40Ext, param: impl Hash40Ext);
    unsafe fn set_float_from_param(&mut self, what: i32, object: impl Hash40Ext, param: impl Hash40Ext);
    unsafe fn set_int64_from_param(&mut self, what: i32, object: impl Hash40Ext, param: impl Hash40Ext);

    // ENERGY
    unsafe fn get_motion_energy(&mut self) -> &mut FighterKineticEnergyMotion;
    unsafe fn get_controller_energy(&mut self) -> &mut FighterKineticEnergyController;
    // tech/general subroutine
    unsafe fn handle_waveland(&mut self, require_airdodge: bool) -> bool;
    unsafe fn set_front_cliff_hangdata(&mut self, x: f32, y: f32);
    unsafe fn set_back_cliff_hangdata(&mut self, x: f32, y: f32);
    unsafe fn set_center_cliff_hangdata(&mut self, x: f32, y: f32);
    unsafe fn select_cliff_hangdata_from_name(&mut self, cliff_hangdata_type: &str);

    // Checks for status and enables transition to jump
    unsafe fn check_jump_cancel(&mut self, update_lr: bool) -> bool;
    // Checks for status and enables transition to airdodge
    unsafe fn check_airdodge_cancel(&mut self) -> bool;
    // Checks for status and enables transition to dash
    unsafe fn check_dash_cancel(&mut self) -> bool;

    /// check for hitfall (should be called once per frame)
    unsafe fn check_hitfall(&mut self);
}

impl BomaExt for BattleObjectModuleAccessor {
    unsafe fn clear_commands<T: Into<CommandCat>>(&mut self, fighter_pad_cmd_flag: T) {
        let cat = fighter_pad_cmd_flag.into();
        let (cat, bits) = match cat {
            CommandCat::Cat1(cat) => (0, cat.bits()),
            CommandCat::Cat2(cat) => (1, cat.bits()),
            CommandCat::Cat3(cat) => (2, cat.bits()),
            CommandCat::Cat4(cat) => (3, cat.bits()),
            CommandCat::CatHdr(cat) => (4, cat.bits())
        };

        crate::modules::InputModule::clear_commands(self.object(), cat, bits);
    }

    unsafe fn is_cat_flag<T: Into<CommandCat>>(&mut self, fighter_pad_cmd_flag: T) -> bool {
        let cat = fighter_pad_cmd_flag.into();
        match cat {
            CommandCat::Cat1(cat) => Cat1::new(self).intersects(cat),
            CommandCat::Cat2(cat) => Cat2::new(self).intersects(cat),
            CommandCat::Cat3(cat) => Cat3::new(self).intersects(cat),
            CommandCat::Cat4(cat) => Cat4::new(self).intersects(cat),
            CommandCat::CatHdr(cat) => CatHdr::new(self).intersects(cat)
        }
    }

    unsafe fn is_cat_flag_all<T: Into<CommandCat>>(&mut self, fighter_pad_cmd_flag: T) -> bool {
        let cat = fighter_pad_cmd_flag.into();
        match cat {
            CommandCat::Cat1(cat) => Cat1::new(self).contains(cat),
            CommandCat::Cat2(cat) => Cat2::new(self).contains(cat),
            CommandCat::Cat3(cat) => Cat3::new(self).contains(cat),
            CommandCat::Cat4(cat) => Cat4::new(self).contains(cat),
            CommandCat::CatHdr(cat) => CatHdr::new(self).intersects(cat)
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

    unsafe fn left_stick_x(&mut self) -> f32 {
        if self.is_button_on(Buttons::CStickOverride) {
            return ControlModule::get_sub_stick_x(self);
        } else {
            return ControlModule::get_stick_x(self);
        }
    }

    unsafe fn left_stick_y(&mut self) -> f32 {
        if self.is_button_on(Buttons::CStickOverride) {
            return ControlModule::get_sub_stick_y(self);
        } else {
            return ControlModule::get_stick_y(self);
        }
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

    unsafe fn set_rate(&mut self, motion_rate: f32) {
        MotionModule::set_rate(self, motion_rate);
    }

    unsafe fn is_motion_one_of(&mut self, kinds: &[Hash40]) -> bool {
        let kind = MotionModule::motion_kind(self);
        return kinds.contains(&Hash40::new_raw(kind));
    }

    unsafe fn motion_frame(&mut self) -> f32 {
        return MotionModule::frame(self);
    }

    unsafe fn is_in_hitlag(&mut self) -> bool{
        let hitlag_frame = WorkModule::get_int(self, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_ATTACK_SUSPEND_FRAME);
        if hitlag_frame > 0 {
            return true;
        }
        return false;
    }

    unsafe fn status_frame(&mut self) -> i32 {
        return crate::util::get_fighter_common_from_accessor(self).global_table[CURRENT_FRAME].get_i32();
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

    unsafe fn get_grabbed_opponent_boma(&mut self) -> &mut BattleObjectModuleAccessor {
        let opponent_id = LinkModule::get_node_object_id(self, *LINK_NO_CAPTURE) as u32;
        let opponent_object = super::util::get_battle_object_from_id(opponent_id);
        &mut *(*opponent_object).module_accessor
    }

    unsafe fn get_grabber_boma(&mut self) -> &mut BattleObjectModuleAccessor {
        let opponent_id = LinkModule::get_parent_object_id(self, *LINK_NO_CAPTURE) as u32;
        let opponent_object = super::util::get_battle_object_from_id(opponent_id);
        &mut *(*opponent_object).module_accessor
    }

    unsafe fn get_num_used_jumps(&mut self) -> i32 {
        return WorkModule::get_int(self, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    }

    unsafe fn get_jump_count_max(&mut self) -> i32 {
        return WorkModule::get_int(self, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
    }

    unsafe fn get_int(&mut self, what: i32) -> i32 {
        WorkModule::get_int(self, what)
    }

    unsafe fn get_float(&mut self, what: i32) -> f32 {
        WorkModule::get_float(self, what)
    }

    unsafe fn get_int64(&mut self, what: i32) -> u64 {
        WorkModule::get_int64(self, what)
    }

    unsafe fn is_flag(&mut self, what: i32) -> bool {
        WorkModule::is_flag(self, what)
    }

    unsafe fn set_int(&mut self, value: i32, what: i32) {
        WorkModule::set_int(self, value, what)
    }

    unsafe fn set_int_from_param(&mut self, what: i32, object: impl Hash40Ext, param: impl Hash40Ext) {
        let int = WorkModule::get_param_int(self, object.to_hash().hash, param.to_hash().hash);
        WorkModule::set_int(self, int, what);
    }

    unsafe fn set_float(&mut self, value: f32, what: i32) {
        WorkModule::set_float(self, value, what)
    }

    unsafe fn set_float_from_param(&mut self, what: i32, object: impl Hash40Ext, param: impl Hash40Ext) {
        let float = WorkModule::get_param_float(self, object.to_hash().hash, param.to_hash().hash);
        WorkModule::set_float(self, float, what);
    }

    unsafe fn set_int64(&mut self, value: i64, what: i32) {
        WorkModule::set_int64(self, value, what)
    }

    unsafe fn set_int64_from_param(&mut self, what: i32, object: impl Hash40Ext, param: impl Hash40Ext) {
        let int = WorkModule::get_param_int64(self, object.to_hash().hash, param.to_hash().hash);
        WorkModule::set_int64(self, int as i64, what);
    }

    unsafe fn on_flag(&mut self, what: i32) {
        WorkModule::on_flag(self, what)
    }

    unsafe fn off_flag(&mut self, what: i32) {
        WorkModule::off_flag(self, what)
    }

    unsafe fn get_param_int(&mut self, obj: &str, field: &str) -> i32 {
        WorkModule::get_param_int(self, Hash40::new(obj).hash, Hash40::new(field).hash)
    }

    unsafe fn get_param_float(&mut self, obj: &str, field: &str) -> f32 {
        let obj = obj.into();
        let field = field.into();
        WorkModule::get_param_float(self, Hash40::new(obj).hash, Hash40::new(field).hash)
    }

    unsafe fn get_param_int64(&mut self, obj: &str, field: &str) -> u64 {
        let obj = obj.into();
        let field = field.into();
        WorkModule::get_param_int64(self, Hash40::new(obj).hash, Hash40::new(field).hash)
    }


    unsafe fn set_joint_rotate(&mut self, bone_name: &str, rotation: Vector3f) {
        ModelModule::set_joint_rotate(self, Hash40::new(&bone_name), &rotation, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8})
    }


    /// gets the FighterKineticEnergyMotion object
    unsafe fn get_motion_energy(&mut self) -> &mut FighterKineticEnergyMotion {
        std::mem::transmute::<u64, &mut app::FighterKineticEnergyMotion>(KineticModule::get_energy(self, *FIGHTER_KINETIC_ENERGY_ID_MOTION))
    }

    /// gets the FighterKineticEnergyController object
    unsafe fn get_controller_energy(&mut self) -> &mut FighterKineticEnergyController {
        std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(self, *FIGHTER_KINETIC_ENERGY_ID_CONTROL))
    }

    unsafe fn handle_waveland(&mut self, require_airdodge: bool) -> bool {
        // MotionModule::frame(self) > 5.0 && !WorkModule::is_flag(self, *FIGHTER_STATUS_ESCAPE_FLAG_HIT_XLU);
        if (require_airdodge && !self.is_status_one_of(&[*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE])) {
            return false;
        }
    
        // must check this because it is for allowing the player to screw up a perfect WD and be punished with a non-perfect WD (otherwise they'd have like, 8 frames for perfect WD lol)
        if !crate::VarModule::is_flag(self.object(), crate::consts::vars::common::instance::ENABLE_AIR_ESCAPE_MAGNET) {
            return false;
        }
    
        if self.is_prev_status(*FIGHTER_STATUS_KIND_JUMP_SQUAT) {
            return false;
        }
    
        // The distance from your ECB center to your base position is your waveland snap threshold
        let pos = *PostureModule::pos(self);
        let upper_bound_offset_y = if StatusModule::is_changing(self) && !self.is_prev_status(*FIGHTER_STATUS_KIND_PASS) {
            crate::VarModule::get_float(self.object(), crate::consts::vars::common::instance::ECB_CENTER_Y_OFFSET)
        } else {
            crate::VarModule::get_float(self.object(), crate::consts::vars::common::instance::ECB_BOTTOM_Y_OFFSET)
        };
        let upper_bound_y = pos.y + upper_bound_offset_y;
        let snap_leniency = if WorkModule::get_float(self, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_Y) <= 0.0 {
                // For a downwards/horizontal airdodge, waveland snap threshold = the distance from your ECB center to your base position
                upper_bound_offset_y
            } else {
                // For an upwards airdodge, waveland snap threshold = 6 units below ECB center, if the distance from your ECB center to your base position is less than 6 units long
                (upper_bound_offset_y).max(crate::ParamModule::get_float(self.object(), crate::ParamType::Common, "waveland_distance_threshold"))
            };
        let lower_bound = Vector2f::new(pos.x, upper_bound_y - snap_leniency);
        let ground_pos_any = &mut Vector2f::zero();
        let ground_pos_stage = &mut Vector2f::zero();
        let is_touch_any = GroundModule::line_segment_check(self, &Vector2f::new(pos.x, upper_bound_y), &lower_bound, &Vector2f::zero(), ground_pos_any, true);
        let is_touch_stage = GroundModule::line_segment_check(self, &Vector2f::new(pos.x, upper_bound_y), &lower_bound, &Vector2f::zero(), ground_pos_stage, false);
        let can_snap = !( 
            is_touch_any == 0 as *const *const u64
            || (is_touch_stage != 0 as *const *const u64
                && WorkModule::get_float(self, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_Y) > 0.0)
        );
        if can_snap { // pretty sure it returns a pointer, at least it defo returns a non-0 value if success
            crate::VarModule::on_flag(self.object(), crate::consts::vars::common::status::DISABLE_ECB_SHIFT);
            PostureModule::set_pos(self, &Vector3f::new(pos.x, ground_pos_any.y + 0.1, pos.z));
            GroundModule::attach_ground(self, false);
            true
        } else {
            false
        }
    }
    
    /// gets the current status kind for the fighter
    unsafe fn status(&mut self) -> i32 {
        return StatusModule::status_kind(self);
    }

    /// If update_lr is true, we set your facing direction based on your stick position
    unsafe fn check_jump_cancel(&mut self, update_lr: bool) -> bool {
        let fighter = crate::util::get_fighter_common_from_accessor(self);
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
            if fighter.sub_transition_group_check_ground_jump_mini_attack().get_bool()
            || fighter.sub_transition_group_check_ground_jump().get_bool() {
                if update_lr {
                    PostureModule::set_stick_lr(self, 0.0);
                    PostureModule::update_rot_y_lr(self);
                }
                return true;
            }
        }
        else {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_FLY);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_FLY_BUTTON);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_FLY_NEXT);
            if fighter.sub_transition_group_check_air_jump_aerial().get_bool() {
                return true;
            }
        }
        false
    }

    unsafe fn check_airdodge_cancel(&mut self) -> bool {
        let fighter = crate::util::get_fighter_common_from_accessor(self);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);
        if fighter.sub_transition_group_check_air_escape().get_bool() {
            return true;
        }
        false
    }

    unsafe fn check_dash_cancel(&mut self) -> bool {
        if self.is_situation(*SITUATION_KIND_GROUND) {
            if self.is_cat_flag(Cat1::Dash) {
                self.change_status_req(*FIGHTER_STATUS_KIND_DASH, false);
                return true;
            } else if self.is_cat_flag(Cat1::TurnDash) {
                self.change_status_req(*FIGHTER_STATUS_KIND_TURN_DASH, false);
                return true;
            }
        }
        false
    }

    /// Sets the position of the front/red ledge-grab box (see [`set_center_cliff_hangdata`](BomaExt::set_center_cliff_hangdata) for more information)
    /// 
    /// # Arguments
    /// * `x` - The x coordinate, relative to the [position](smash::app::lua_bind::PostureModule::pos) of the fighter
    /// * `y` - The y coordinate, relative to the [position](smash::app::lua_bind::PostureModule::pos) of the fighter
    unsafe fn set_front_cliff_hangdata(&mut self, x: f32, y: f32) {
        let ground_module = *(self as *mut BattleObjectModuleAccessor as *const u64).add(0x58 / 8);
        let ground_data = *((ground_module + 0x28) as *mut *mut f32);
        *ground_data.add(0x530 / 4) = x;
        *ground_data.add(0x534 / 4) = y;
    }

    /// Sets the position of the back/blue ledge-grab box (see [`set_center_cliff_hangdata`](BomaExt::set_center_cliff_hangdata) for more information)
    /// 
    /// # Arguments
    /// * `x` - The x coordinate, relative to the [position](smash::app::lua_bind::PostureModule::pos) of the fighter
    /// * `y` - The y coordinate, relative to the [position](smash::app::lua_bind::PostureModule::pos) of the fighter
    unsafe fn set_back_cliff_hangdata(&mut self, x: f32, y: f32) {
        let ground_module = *(self as *mut BattleObjectModuleAccessor as *const u64).add(0x58 / 8);
        let ground_data = *((ground_module + 0x28) as *mut *mut f32);
        *ground_data.add(0x540 / 4) = x;
        *ground_data.add(0x544 / 4) = y;
    }

    /// Sets the center position of the two ledge-grab boxes
    /// 
    /// # Information about hang data
    /// There are two rectangles which represent the ledge-grab data for a fighter. One of them is usually
    /// placed behind the fighter and the other in front. For the purposes of explanation, I will refer to
    /// the one in front as "red" and the one in the back as "blue", as those are the colors chosen
    /// for the visualizer. 
    /// 
    /// The center position for ledge-grab boxes is a point which both the red and blue boxes have as a corner.
    /// Both boxes meet at this location. This is usually located near the center of the fighter on the x-axis. The
    /// location on the y-axis of the fighter depends on the fighter.
    /// 
    /// The front and back positions (set by [`BomaExt::set_front_cliff_hangdata`] and [`BomaExt::set_back_cliff_hangdata`] respectively)
    /// are the corners that oppose this center.
    /// 
    /// # Arguments
    /// * `x` - The x coordinate, relative to the [position](smash::app::lua_bind::PostureModule::pos) of the fighter
    /// * `y` - The y coordinate, relative to the [position](smash::app::lua_bind::PostureModule::pos) of the fighter
    unsafe fn set_center_cliff_hangdata(&mut self, x: f32, y: f32) {
        let ground_module = *(self as *mut BattleObjectModuleAccessor as *const u64).add(0x58 / 8);
        let ground_data = *((ground_module + 0x28) as *mut *mut f32);
        *ground_data.add(0x520 / 4) = x;
        *ground_data.add(0x524 / 4) = y;
    }

    unsafe fn select_cliff_hangdata_from_name(&mut self, name: &str) {
        let p1_x = crate::ParamModule::get_float(self.object(), crate::ParamType::Agent, &format!("cliff_hang_data.{}.p1_x", name));
        let p1_y = crate::ParamModule::get_float(self.object(), crate::ParamType::Agent, &format!("cliff_hang_data.{}.p1_y", name));
        let p2_x = crate::ParamModule::get_float(self.object(), crate::ParamType::Agent, &format!("cliff_hang_data.{}.p2_x", name));
        let p2_y = crate::ParamModule::get_float(self.object(), crate::ParamType::Agent, &format!("cliff_hang_data.{}.p2_y", name));

        // Can uncomment and test hardcoded values here, while working on a character
        // so you don't have to rebuild hdr.prc every time
        //let p1_x = 16.0;
        //let p1_y = 18.0;
        //let p2_x = -9.6;
        //let p2_y = 9.0;

        self.set_front_cliff_hangdata(p1_x, (p1_y - p2_y));
        self.set_back_cliff_hangdata((p2_x * -1.0), (p1_y - p2_y));
        self.set_center_cliff_hangdata(0.0, p2_y);
    }

    /// checks whether you should hitfall (call this once per frame)
    unsafe fn check_hitfall(&mut self) {
        if self.is_situation(*SITUATION_KIND_AIR)
        && self.is_status(*FIGHTER_STATUS_KIND_ATTACK_AIR)
        {
            /* this is written this way because stick_y_flick wont update during
                hitlag, which means we need a flag to allow you to hitfall 1 frame
                after the end of hitlag as well, and we need to check previous 
                stick y directly to detect hitfall. That way, with the 5 frame buffer,
                if you input a fastfall during hitlag, it will get registered after
                the hitlag is over. Without the HITFALL_BUFFER flag, you have to
                input the fastfall BEFORE you hit the move, only.
            */
            if !AttackModule::is_infliction_status(self, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD)
            || AttackModule::is_infliction(self, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD)
            {
               crate::VarModule::set_int(self.object(), crate::consts::vars::common::instance::HITFALL_BUFFER, 0);
            }

            if AttackModule::is_infliction_status(self, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) {
               crate::VarModule::inc_int(self.object(), crate::consts::vars::common::instance::HITFALL_BUFFER);
            }

            let buffer =crate::VarModule::get_int(self.object(), crate::consts::vars::common::instance::HITFALL_BUFFER);

            if self.is_cat_flag(Cat2::FallJump)
            && 0 < buffer && buffer <= 5 
            {
                WorkModule::on_flag(self, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
            }
        }
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

/// Enum for the kinds of controls that are mapped
/// Can map any of these over any button
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum InputKind {
    Attack = 0x0,
    Special = 0x1,
    Jump = 0x2,
    Guard = 0x3,
    Grab = 0x4,
    SmashAttack = 0x5,
    AppealHi = 0xA,
    AppealS = 0xB,
    AppealLw = 0xC,
    Unset = 0xD,
    JumpMini = 0x12, // this is ours :), also start at 0x12 to avoid masking errors
    TiltAttack = 0x13, // also custom, this one is for tilts!
}

/// 0x50 Byte struct containing the information for controller mappings
#[derive(Debug)]
#[repr(C)]
pub struct ControllerMapping {
    pub gc_l: InputKind,
    pub gc_r: InputKind,
    pub gc_z: InputKind,
    pub gc_dup: InputKind,
    pub gc_dlr: InputKind,
    pub gc_ddown: InputKind,
    pub gc_a: InputKind,
    pub gc_b: InputKind,
    pub gc_cstick: InputKind,
    pub gc_y: InputKind,
    pub gc_x: InputKind,
    pub gc_rumble: bool,
    pub gc_absmash: bool,
    pub gc_tapjump: bool,
    pub gc_sensitivity: u8,
    // 0xF
    pub pro_l: InputKind,
    pub pro_r: InputKind,
    pub pro_zl: InputKind,
    pub pro_zr: InputKind,
    pub pro_dup: InputKind,
    pub pro_dlr: InputKind,
    pub pro_ddown: InputKind,
    pub pro_a: InputKind,
    pub pro_b: InputKind,
    pub pro_cstick: InputKind,
    pub pro_x: InputKind,
    pub pro_y: InputKind,
    pub pro_rumble: bool,
    pub pro_absmash: bool,
    pub pro_tapjump: bool,
    pub pro_sensitivity: u8,
    // 0x1F
    pub joy_shoulder: InputKind,
    pub joy_zshoulder: InputKind,
    pub joy_sl: InputKind,
    pub joy_sr: InputKind,
    pub joy_up: InputKind,
    pub joy_right: InputKind,
    pub joy_left: InputKind,
    pub joy_down: InputKind,
    pub joy_rumble: bool,
    pub joy_absmash: bool,
    pub joy_tapjump: bool,
    pub joy_sensitivity: u8,
    // 0x2B
    pub _2b: u8,
    pub _2c: u8,
    pub _2d: u8,
    pub _2e: u8,
    pub _2f: u8,
    pub _30: u8,
    pub _31: u8,
    pub _32: u8,
    pub is_absmash: bool,
    pub _34: [u8; 0x1C]
}

/// Controller class used internally by the game
#[repr(C)]
pub struct Controller {
    pub vtable: *const u64,
    pub current_buttons: ButtonBitfield,
    pub previous_buttons: ButtonBitfield,
    pub left_stick_x: f32,
    pub left_stick_y: f32,
    pub left_trigger: f32,
    pub _left_padding: u32,
    pub right_stick_x: f32,
    pub right_stick_y: f32,
    pub right_trigger: f32,
    pub _right_padding: u32,
    pub gyro: [f32; 4],
    pub button_timespan: AutorepeatInfo,
    pub lstick_timespan: AutorepeatInfo,
    pub rstick_timespan: AutorepeatInfo,
    pub just_down: ButtonBitfield,
    pub just_release: ButtonBitfield,
    pub autorepeat_keys: u32,
    pub autorepeat_threshold: u32,
    pub autorepeat_initial_press_threshold: u32,
    pub style: ControllerStyle,
    pub controller_id: u32,
    pub primary_controller_color1: u32,
    pub primary_controller_color2: u32,
    pub secondary_controller_color1: u32,
    pub secondary_controller_color2: u32,
    pub led_pattern: u8,
    pub button_autorepeat_initial_press: bool,
    pub lstick_autorepeat_initial_press: bool,
    pub rstick_autorepeat_initial_press: bool,
    pub is_valid_controller: bool,
    pub _xB9: [u8; 2],
    pub is_connected: bool,
    pub is_left_connected: bool,
    pub is_right_connected: bool,
    pub is_wired: bool,
    pub is_left_wired: bool,
    pub is_right_wired: bool,
    pub _xC1: [u8; 3],
    pub npad_number: u32,
    pub _xC8: [u8; 8]
}

/// Re-ordered bitfield the game uses for buttons
#[bitfield]
#[derive(Debug, Default, Copy, Clone)]
pub struct ButtonBitfield {
    pub dpad_up: bool,
    pub dpad_right: bool,
    pub dpad_down: bool,
    pub dpad_left: bool,
    pub x: bool,
    pub a: bool,
    pub b: bool,
    pub y: bool,
    pub l: bool,
    pub r: bool,
    pub zl: bool,
    pub zr: bool,
    pub left_sl: bool,
    pub left_sr: bool,
    pub right_sl: bool,
    pub right_sr: bool,
    pub stick_l: bool,
    pub stick_r: bool,
    pub plus: bool,
    pub minus: bool,
    pub l_up: bool,
    pub l_right: bool,
    pub l_down: bool,
    pub l_left: bool,
    pub r_up: bool,
    pub r_right: bool,
    pub r_down: bool,
    pub r_left: bool,
    pub unused: B4
}

#[repr(C)]
pub struct AutorepeatInfo {
    field: [u8; 0x18]
}

/// Controller style declaring what kind of controller is being used
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
#[repr(u32)]
pub enum ControllerStyle {
    Handheld = 0x1,
    DualJoycon = 0x2,
    LeftJoycon = 0x3,
    RightJoycon = 0x4,
    ProController = 0x5,
    DebugPag = 0x6, // I assume
    GCController = 0x7
}

/// 8 byte struct containig all button inputs
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MappedInputs {
    pub buttons: Buttons,
    pub lstick_x: i8,
    pub lstick_y: i8,
    pub rstick_x: i8,
    pub rstick_y: i8
}

pub type StatusFunc = unsafe extern "C" fn(&mut L2CFighterCommon) -> L2CValue;

pub struct StatusInfo {
    pub pre: Option<StatusFunc>,
    pub main: Option<StatusFunc>,
    pub end: Option<StatusFunc>,
    pub init: Option<StatusFunc>,
    pub exec: Option<StatusFunc>,
    pub exec_stop: Option<StatusFunc>,
    pub exec_post: Option<StatusFunc>,
    pub exit: Option<StatusFunc>,
    pub map_correction: Option<StatusFunc>,
    pub fix_camera: Option<StatusFunc>,
    pub fix_pos_slow: Option<StatusFunc>,
    pub check_damage: Option<StatusFunc>,
    pub check_attack: Option<StatusFunc>,
    pub on_change_lr: Option<StatusFunc>,
    pub leave_stop: Option<StatusFunc>,
    pub notify_event_gimmick: Option<StatusFunc>,
    pub calc_param: Option<StatusFunc>
}

impl StatusInfo {
    pub fn new() -> StatusInfo {
        StatusInfo {
            pre: None,
            main: None,
            end: None,
            init: None,
            exec: None,
            exec_stop: None,
            exec_post: None,
            exit: None,
            map_correction: None,
            fix_camera: None,
            fix_pos_slow: None,
            check_damage: None,
            check_attack: None,
            on_change_lr: None,
            leave_stop: None,
            notify_event_gimmick: None,
            calc_param: None,
        }
    }

    pub fn with_pre(mut self, pre: StatusFunc) -> Self {
        self.pre = Some(pre);
        self
    }

    pub fn with_main(mut self, main: StatusFunc) -> Self {
        self.main = Some(main);
        self
    }

    pub fn with_end(mut self, end: StatusFunc) -> Self {
        self.end = Some(end);
        self
    }

    pub fn with_init(mut self, init: StatusFunc) -> Self {
        self.init = Some(init);
        self
    }

    pub fn with_exec(mut self, exec: StatusFunc) -> Self {
        self.exec = Some(exec);
        self
    }

    pub fn with_exec_stop(mut self, exec_stop: StatusFunc) -> Self {
        self.exec_stop = Some(exec_stop);
        self
    }

    pub fn with_exec_post(mut self, exec_post: StatusFunc) -> Self {
        self.exec_post = Some(exec_post);
        self
    }

    pub fn with_exit(mut self, exit: StatusFunc) -> Self {
        self.exit = Some(exit);
        self
    }

    pub fn with_map_correction(mut self, map_correction: StatusFunc) -> Self {
        self.map_correction = Some(map_correction);
        self
    }

    pub fn with_fix_camera(mut self, fix_camera: StatusFunc) -> Self {
        self.fix_camera = Some(fix_camera);
        self
    }

    pub fn with_fix_pos_slow(mut self, fix_pos_slow: StatusFunc) -> Self {
        self.fix_pos_slow = Some(fix_pos_slow);
        self
    }

    pub fn with_check_damage(mut self, check_damage: StatusFunc) -> Self {
        self.check_damage = Some(check_damage);
        self
    }

    pub fn with_check_attack(mut self, check_attack: StatusFunc) -> Self {
        self.check_attack = Some(check_attack);
        self
    }

    pub fn with_on_change_lr(mut self, on_change_lr: StatusFunc) -> Self {
        self.on_change_lr = Some(on_change_lr);
        self
    }

    pub fn with_leave_stop(mut self, leave_stop: StatusFunc) -> Self {
        self.leave_stop = Some(leave_stop);
        self
    }

    pub fn with_notify_event_gimmick(mut self, notify_event_gimmick: StatusFunc) -> Self {
        self.notify_event_gimmick = Some(notify_event_gimmick);
        self
    }

    pub fn with_calc_param(mut self, calc_param: StatusFunc) -> Self {
        self.calc_param = Some(calc_param);
        self
    }

}

pub fn is_hdr_available() -> bool {
    let mut symbol = 0usize;
    unsafe {
        skyline::nn::ro::LookupSymbol(&mut symbol, "hdr_is_available\0".as_ptr());
    }
    symbol != 0
}
