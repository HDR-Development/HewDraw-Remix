use smash::app::{
    lua_bind::ControlModule, lua_bind::WorkModule, BattleObject, BattleObjectModuleAccessor,
};
use utils_dyn::ext::*;

use crate::consts::globals::*;
use crate::consts::*;
use crate::modules::*;
use crate::offsets;
use crate::util::get_battle_object_from_id;
use crate::util::get_fighter_common_from_accessor;
use smash::hash40;

use super::INPUT_MODULE_OFFSET;

use globals::*;

macro_rules! get_input_module {
    ($object:ident) => {{
        unsafe {
            let vtable = *($object as *mut *mut *mut u64);
            &mut *super::get_entry::<InputModule>(vtable, INPUT_MODULE_OFFSET)
                .expect("Did not find InputModule!")
        }
    }};
}

macro_rules! has_input_module {
    ($object:ident) => {{
        unsafe {
            if $object.is_null() {
                false
            } else {
                let vtable = *($object as *mut *mut *mut u64);
                super::is_hdr_object(vtable as _)
                    && !super::get_entry::<InputModule>(vtable, INPUT_MODULE_OFFSET).is_none()
            }
        }
    }};
}

macro_rules! require_input_module {
    ($object:ident) => {{
        if !has_input_module!($object) {
            panic!("BattleObject does not contain reference to InputModule!");
        }
        get_input_module!($object)
    }};
}

#[repr(C)]
struct HdrCat {
    /// Represents the number of remaining buffered frames that each input will be considered active.
    /// A value of zero for any entry means that input is not considered to be in the buffer.
    pub valid_frames: [u8; 32],
}

#[repr(C)]
struct BufferState {
    pub on_last_frame: u32,
    pub should_hold: [bool; 32],
    pub hold_frame: [i32; 32],
    pub hold_frame_max: [i32; 32],
}

impl BufferState {
    pub fn new() -> Self {
        Self {
            on_last_frame: 0,
            should_hold: [false; 32],
            hold_frame: [0; 32],
            hold_frame_max: [-1; 32],
        }
    }

    pub fn clear(&mut self) {
        self.on_last_frame = 0;
        self.should_hold = [false; 32];
        self.hold_frame = [0; 32];
        self.hold_frame_max = [-1; 32];
    }

    pub fn update(
        &mut self,
        game_held: &mut [u8],
        max_hold_frame: i32,
        press_frame: i32,
        should_hold: bool,
    ) {
        self.on_last_frame = 0;
        for (idx, x) in game_held.iter_mut().enumerate() {
            if *x != 0
                && (self.hold_frame[idx] < press_frame
                    || self.should_hold[idx]
                    || should_hold
                    || *x != 1)
            {
                self.hold_frame[idx] += 1;
                if self.hold_frame[idx] < press_frame {
                    continue;
                }
                if *x == 1 {
                    if self.should_hold[idx] {
                        if self.hold_frame_max[idx] != -1
                            && self.hold_frame_max[idx] < self.hold_frame[idx]
                        {
                            *x = 0;
                            self.hold_frame[idx] = 0;
                            continue;
                        }
                    }
                } else if should_hold {
                    if max_hold_frame != -1 && max_hold_frame < self.hold_frame[idx] {
                        *x = 0;
                        self.hold_frame[idx] = 0;
                        continue;
                    }
                }
                self.on_last_frame |= 1 << idx;
            } else {
                self.hold_frame[idx] = 0;
                *x = 0;
            }
        }
    }
}

/// An additional module to be used with Smash's `BattleObject` class. This handles manipulating and adjusting hold buffer
/// depending on the situation to encourage more precise inputs with some exceptions to allow for overall better game health and feel.
/// You can reference all of these calls from just passing the `BattleObject` into function. If a function is called on a `BattleObject` that does not have
/// `InputModule` set up, it will panic.
#[repr(C)]
pub struct InputModule {
    owner: *mut BattleObject,
    cats: [BufferState; 5],
    hdr_cat: HdrCat,
    hold_all: bool,
    hold_all_frame_max: i32,
}

impl InputModule {
    /// Constructs a new `InputModule` instance with the specified `BattleObject` as its own
    /// # Arguments
    /// * `owner` - Owning `BattleObject` instance
    /// # Returns
    /// New, constructed `InputModule` with all settings set to default
    pub(crate) fn new(owner: *mut BattleObject) -> Self {
        Self {
            owner,
            cats: [
                BufferState::new(),
                BufferState::new(),
                BufferState::new(),
                BufferState::new(),
                BufferState::new(),
            ],
            hdr_cat: HdrCat {
                valid_frames: [0; 32],
            },
            hold_all: false,
            hold_all_frame_max: -1,
        }
    }

    /// Enables the hold buffer on one specific input.
    /// # Arguments
    /// * `object` - Owning `BattleObject` instance
    /// * `category` - Which command flag category the input is under (valid values are 0-3)
    /// * `flag` - Which flag in the category you are enabling hold buffer for
    #[export_name = "InputModule__persist_command_one"]
    pub fn persist_command_one(object: *mut BattleObject, category: i32, flag: i32) {
        let module = require_input_module!(object);

        let flag = flag & 0x1F;
        module.cats[category as usize].should_hold[(flag & 0x1F) as usize] = true;
        module.cats[category as usize].hold_frame_max[(flag & 0x1F) as usize] = -1;
    }

    /// Enables the hold buffer on one specific input with a specified lifetime
    /// # Arguments
    /// * `object` - Owning `BattleObject` instance
    /// * `category` - Which command flag category the input is under (valid values are 0-3)
    /// * `flag` - Which flag in the category you are enabling hold buffer for
    /// * `lifetime` - The maximum number of frames hold buffer is enabled for (-1 is infinite). This lifetime includes tap buffer frames.
    #[export_name = "InputModule__persist_command_one_with_lifetime"]
    pub fn persist_command_one_with_lifetime(
        object: *mut BattleObject,
        category: i32,
        flag: i32,
        lifetime: i32,
    ) {
        let module = require_input_module!(object);

        Self::persist_command_one(object, category, flag);
        module.cats[category as usize].hold_frame_max[(flag & 0x1F) as usize] = lifetime;
    }

    /// Sets the global hold buffer lifetime
    /// # Arguments
    /// * `object` - Owning `BattleObject` instance
    /// * `lifetime` - The maximum number of frames hold buffer is enabled for (-1 is infinite). This lifetime includes tap buffer frames.
    #[export_name = "InputModule__set_persist_lifetime"]
    pub fn set_persist_lifetime(object: *mut BattleObject, lifetime: i32) {
        require_input_module!(object).hold_all_frame_max = lifetime;
    }

    /// Enables global hold buffer on all inputs for this `BattleObject`
    /// # Arguments
    /// * `object` - Owning `BattleObject` instance
    #[export_name = "InputModule__enable_persist"]
    pub fn enable_persist(object: *mut BattleObject) {
        require_input_module!(object).hold_all = true;
    }

    /// Disables global hold buffer for this `BattleObject`
    /// # Arguments
    /// * `object` - Owning `BattleObject` instance
    /// # Note
    /// If specific inputs have hold buffer enabled, calling `disable_persist` will not disable those,
    /// only the global flag which enabled hold buffer on all inputs will be disabled
    #[export_name = "InputModule__disable_persist"]
    pub fn disable_persist(object: *mut BattleObject) {
        require_input_module!(object).hold_all = false;
    }

    /// Clears all of the hold buffer information for every input
    /// # Arguments
    /// * `object` - Owning `BattleObject` instance
    /// # Note
    /// This function is similar to `ControlModule::clear_command_flag_cat` in that it resets all information regarding holding those inputs.
    /// This does not impact anything in the `ControlModule` command information, only the `InputModule` implementation
    #[export_name = "InputModule__clear_persist"]
    pub fn clear_persist(object: *mut BattleObject) {
        let module = require_input_module!(object);

        module.cats[0].clear();
        module.cats[1].clear();
        module.cats[2].clear();
        module.cats[3].clear();
        module.cats[4].clear();
    }

    /// Clears the hold buffer information for one input
    /// # Arguments
    /// * `object` - Owning `BattleObject` instance
    /// * `category` - Which command flag category the input is under (valid values are 0-3)
    /// * `flag` - Which flag in the category you are clearing hold buffer for
    #[export_name = "InputModule__clear_persist_one"]
    pub fn clear_persist_one(object: *mut BattleObject, category: i32, flag: i32) {
        let module = require_input_module!(object);
        let cat = &mut module.cats[category as usize];
        cat.on_last_frame &= !(1 << (flag as usize));
        cat.should_hold[flag as usize] = false;
        cat.hold_frame[flag as usize] = 0;
        cat.hold_frame_max[flag as usize] = -1;
    }

    /// Updates the hold buffer information
    /// # Arguments
    /// * `object` - Owning `BattleObject` instance
    /// * `cats` - `ControlModule` command flag information to update `InputModule` with.
    /// # Note
    /// This method is not intended to be used by users of `InputModule`. It is instead used internally with a hook to update every frame.
    #[export_name = "InputModule__exec"]
    pub fn exec(object: *mut BattleObject, cats: &mut [&mut [u8]; 5]) {
        let module = require_input_module!(object);

        let press_frame = unsafe {
            ControlModule::get_command_life_count_max((*module.owner).module_accessor) as i32
        };

        for x in 0..5 {
            module.cats[x].update(
                cats[x],
                module.hold_all_frame_max,
                press_frame - 1,
                module.hold_all,
            );
        }
    }

    /// Checks whether or not global hold buffer is enabled
    /// # Arguments
    /// * `object` - Owning `BattleObject` instance
    /// # Returns
    /// A boolean representing whether or not global hold buffer is enabled.
    #[export_name = "InputModule__is_persist"]
    pub fn is_persist(object: *mut BattleObject) -> bool {
        require_input_module!(object).hold_all
    }

    /// Checks whether or not hold buffer is enabled for a specific input
    /// # Arguments
    /// * `object` - Owning `BattleObject` instance
    /// * `category` - Which command flag category the input is under (valid values are 0-3)
    /// * `flag` - Which flag in the category you are checking hold buffer for
    /// # Returns
    /// A boolean representing whether or not hold buffer is enabled for a specific input.
    #[export_name = "InputModule__is_persist_one"]
    pub fn is_persist_one(object: *mut BattleObject, category: i32, flag: i32) -> bool {
        require_input_module!(object).cats[category as usize].should_hold[flag as usize]
    }

    /// Gets the max amount of global hold buffer frames (can vary depending on input)
    /// # Arguments
    /// * `object` - Owning `BattleObject` instance
    /// # Returns
    /// The number of frames hold buffer is enabled for globally.
    /// #Note
    /// This returns whatever value was last last set with `set_persist_lifetime` and
    /// is a valid value even when `is_persist` is false.
    #[export_name = "InputModule__persist_lifetime"]
    pub fn persist_lifetime(object: *mut BattleObject) -> i32 {
        require_input_module!(object).hold_all_frame_max
    }

    /// Gets the current amount of frames an object has been holding an input for
    /// # Arguments
    /// * `object` - Owning `BattleObject` instance
    /// * `category` - Which command flag category the input is under (valid values are 0-3)
    /// * `flag` - Which flag in the category you are checking hold buffer for
    /// # Returns
    /// The number of frames the input has been held
    #[export_name = "InputModule__persist_lifetime_one"]
    pub fn persist_lifetime_one(object: *mut BattleObject, category: i32, flag: i32) -> i32 {
        require_input_module!(object).cats[category as usize].hold_frame[flag as usize]
    }

    /// Gets the max amount of hold buffer frames for a specified input
    /// # Arguments
    /// * `object` - Owning `BattleObject` instance
    /// * `category` - Which command flag category the input is under (valid values are 0-3)
    /// * `flag` - Which flag in the category you are checking hold buffer for
    /// # Returns
    /// The max amount of frames a specific input can have hold buffer for.
    #[export_name = "InputModule__persist_lifetime_max_one"]
    pub fn persist_lifetime_max_one(object: *mut BattleObject, category: i32, flag: i32) -> i32 {
        require_input_module!(object).cats[category as usize].hold_frame_max[flag as usize]
    }

    #[export_name = "InputModule__clear_command_one_proper"]
    pub fn clear_commands(object: *mut BattleObject, category: i32, flags: i32) {
        if category == 4 {
            if !has_input_module!(object) {
                return;
            }

            let module = require_input_module!(object);

            for x in 0..32 {
                if flags & (1 << x) != 0 {
                    module.hdr_cat.valid_frames[x] = 0;
                }
            }

            return;
        }

        let cats = unsafe {
            let control_module = *((*object).module_accessor as *const u64).add(0x48 / 8);
            std::slice::from_raw_parts_mut((control_module + 0x568) as *mut CommandFlagCat, 4)
        };

        for x in 0..cats[category as usize].count {
            if flags & (1 << x) != 0 {
                cats[category as usize].lifetimes_mut()[x] = 0;
            }
        }
    }

    #[export_name = "InputModule__get_analog_for_guard"]
    pub fn get_analog_for_guard(object: *mut BattleObject) -> f32 {
        unsafe {
            let control_module = *((*object).module_accessor as *const u64).add(0x48 / 8);
            let inner = *(control_module as *const u64).add(0x110 / 8);
            *(inner as *const f32).add(0x48 / 4)
        }
    }
}

#[repr(C)]
struct CommandFlagCat {
    flags: u32,
    _x4: u32,
    count: usize,
    lifetimes: *mut u8,
    lifetimes2: *mut u8,
    lifetimes3: *mut u64,
}

impl CommandFlagCat {
    fn lifetimes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.lifetimes, self.count) }
    }

    fn lifetimes_mut(&self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.lifetimes, self.count) }
    }

    fn lifetimes2(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.lifetimes2, self.count) }
    }
}

#[skyline::hook(offset = offsets::get_command_flag_cat())]
fn get_command_flag_cat_replace(control_module: u64, cat: i32) -> u32 {
    let boma = unsafe { *(control_module as *mut *mut BattleObjectModuleAccessor).add(1) };
    let battle_object = unsafe { get_battle_object_from_id((*boma).battle_object_id) };

    if cat == 4 {
        if !has_input_module!(battle_object) {
            return 0;
        }

        let im = require_input_module!(battle_object);

        let mut output = 0;
        // this iterates across all 32 bits of the output bitmask, where valid_frames represents how many frames
        // left any given custom input may have left in its internal buffer state.
        for x in 0..32 {
            if im.hdr_cat.valid_frames[x] != 0 {
                output |= 1 << x;
            }
        }

        return output;
    }

    let cats =
        unsafe { std::slice::from_raw_parts((control_module + 0x568) as *mut CommandFlagCat, 4) };

    let mut output = 0;
    let lifetimes = cats[cat as usize].lifetimes();
    let lifetimes2 = cats[cat as usize].lifetimes2();
    for x in 0..cats[cat as usize].count {
        if lifetimes[x] > 0 && lifetimes2[x] <= 1 {
            output |= 1 << x;
        }
    }
    output
}

fn exec_internal(input_module: &mut InputModule, control_module: u64, call_original: impl Fn()) {
    let triggered_buttons: Buttons = unsafe {
        Buttons::from_bits_unchecked(
            ControlModule::get_button((*input_module.owner).module_accessor)
                & !ControlModule::get_button_prev((*input_module.owner).module_accessor),
        )
    };

    let buttons: Buttons = unsafe {
        Buttons::from_bits_unchecked(ControlModule::get_button(
            (*input_module.owner).module_accessor,
        ))
    };

    unsafe {
        // Allow Aidou with only A button held
        // Also extends directional inputs for Tilt Stick Aidou
        if (*input_module.owner).was_prev_button_on(Buttons::AttackAll)
            && triggered_buttons.intersects(Buttons::CStickOn)
        {
            // smash stick input
            let stick_x = ControlModule::get_stick_x((*input_module.owner).module_accessor);
            let stick_y = ControlModule::get_stick_y((*input_module.owner).module_accessor);
            let dash_stick_x = WorkModule::get_param_float(
                (*input_module.owner).module_accessor,
                hash40("common"),
                hash40("dash_stick_x"),
            );
            let squat_stick_y = WorkModule::get_param_float(
                (*input_module.owner).module_accessor,
                hash40("common"),
                hash40("squat_stick_y"),
            ) * -1.0;
            if stick_x != 0.0 && stick_x.abs() < dash_stick_x {
                ControlModule::set_main_stick_x(
                    (*input_module.owner).module_accessor,
                    dash_stick_x * stick_x.signum(),
                );
            }
            if stick_y != 0.0 && stick_y.abs() < squat_stick_y {
                ControlModule::set_main_stick_y(
                    (*input_module.owner).module_accessor,
                    squat_stick_y * stick_y.signum(),
                );
            }
            ControlModule::reset_trigger((*input_module.owner).module_accessor);
        }
        // Prevent game from thinking you are inputting a flick on the frame the cstick stops overriding left stick
        if (*input_module.owner).is_button_release(Buttons::CStickOverride) {
            ControlModule::reset_flick_x((*input_module.owner).module_accessor);
            ControlModule::reset_flick_y((*input_module.owner).module_accessor);
        }
    }

    // TiltAttack cat flag
    let tilt_attack_offset = CatHdr::TiltAttack.bits().trailing_zeros() as usize;
    if triggered_buttons.intersects(Buttons::TiltAttack) {
        if input_module.hdr_cat.valid_frames[tilt_attack_offset] == 0 {
            input_module.hdr_cat.valid_frames[tilt_attack_offset] = unsafe {
                ControlModule::get_command_life_count_max((*input_module.owner).module_accessor)
                    as u8
            };
        }
    }
    if input_module.hdr_cat.valid_frames[tilt_attack_offset] != 0
        && !(input_module.hdr_cat.valid_frames[tilt_attack_offset] == 1
            && buttons.intersects(Buttons::TiltAttack))
    {
        input_module.hdr_cat.valid_frames[tilt_attack_offset] -= 1;
    }

    // Wavedash cat flag
    let wavedash_offset = CatHdr::Wavedash.bits().trailing_zeros() as usize;
    let wavedash_input = (triggered_buttons.intersects(Buttons::Jump)
        || triggered_buttons.intersects(Buttons::FlickJump))
        && triggered_buttons.intersects(Buttons::Guard);
    if wavedash_input {
        if input_module.hdr_cat.valid_frames[wavedash_offset] == 0 {
            input_module.hdr_cat.valid_frames[wavedash_offset] = unsafe {
                ControlModule::get_command_life_count_max((*input_module.owner).module_accessor)
                    as u8
            };
        }
    }
    if input_module.hdr_cat.valid_frames[wavedash_offset] != 0
        && !(input_module.hdr_cat.valid_frames[wavedash_offset] == 1 && wavedash_input)
    {
        input_module.hdr_cat.valid_frames[wavedash_offset] -= 1;
    }

    // ShieldDrop cat flag
    let shielddrop_offset = CatHdr::ShieldDrop.bits().trailing_zeros() as usize;
    let pass_flick_y = unsafe {
        WorkModule::get_param_int(
            (*input_module.owner).module_accessor,
            hash40("common"),
            hash40("pass_flick_y"),
        )
    };
    let pass_stick_y = unsafe {
        if VarModule::is_flag(&mut (*input_module.owner), vars::common::status::ENABLE_UCF) {
            ParamModule::get_float(
                &mut (*input_module.owner),
                ParamType::Common,
                "ucf_pass_stick_y",
            )
        } else {
            WorkModule::get_param_float(
                (*input_module.owner).module_accessor,
                hash40("common"),
                hash40("pass_stick_y"),
            )
        }
    };
    let escape_stick_y = unsafe {
        if VarModule::is_flag(&mut (*input_module.owner), vars::common::status::ENABLE_UCF) {
            ParamModule::get_float(
                &mut (*input_module.owner),
                ParamType::Common,
                "ucf_escape_stick_y",
            )
        } else {
            WorkModule::get_param_float(
                (*input_module.owner).module_accessor,
                hash40("common"),
                hash40("escape_stick_y"),
            )
        }
    };
    let shielddrop_input = unsafe {
        buttons.intersects(Buttons::Guard)
            && ControlModule::get_flick_y((*input_module.owner).module_accessor) < pass_flick_y
            && ControlModule::get_stick_y((*input_module.owner).module_accessor) <= pass_stick_y
            && ControlModule::get_stick_y((*input_module.owner).module_accessor) > escape_stick_y
    };
    if shielddrop_input {
        if input_module.hdr_cat.valid_frames[shielddrop_offset] == 0 {
            input_module.hdr_cat.valid_frames[shielddrop_offset] = unsafe {
                ControlModule::get_command_life_count_max((*input_module.owner).module_accessor)
                    as u8
            };
        }
    }
    if input_module.hdr_cat.valid_frames[shielddrop_offset] != 0
        && !(input_module.hdr_cat.valid_frames[shielddrop_offset] == 1 && shielddrop_input)
    {
        input_module.hdr_cat.valid_frames[shielddrop_offset] -= 1;
    }
    call_original();

    let cats = unsafe {
        std::slice::from_raw_parts_mut((control_module + 0x568) as *mut CommandFlagCat, 4)
    };

    let mut lifetimes = [
        cats[0].lifetimes_mut(),
        cats[1].lifetimes_mut(),
        cats[2].lifetimes_mut(),
        cats[3].lifetimes_mut(),
        &mut input_module.hdr_cat.valid_frames,
    ];

    let boma = unsafe { *(control_module as *mut *mut BattleObjectModuleAccessor).add(1) };

    if triggered_buttons.intersects(Buttons::TiltAttack) {
        unsafe {
            (*input_module.owner).clear_commands(Cat1::AttackS4);
            (*input_module.owner).clear_commands(Cat1::AttackHi4);
            (*input_module.owner).clear_commands(Cat1::AttackLw4);
        }
    }

    InputModule::exec(input_module.owner, &mut lifetimes);
}

#[skyline::hook(offset = offsets::exec_command())]
fn exec_command_hook(control_module: u64, flag: bool) {
    let boma = unsafe { *(control_module as *mut *mut BattleObjectModuleAccessor).add(1) };
    let battle_object = unsafe { get_battle_object_from_id((*boma).battle_object_id) };

    if has_input_module!(battle_object) {
        exec_internal(require_input_module!(battle_object), control_module, || {
            call_original!(control_module, flag)
        });
    } else {
        call_original!(control_module, flag);
    }
}

pub(crate) fn init() {
    skyline::install_hooks!(get_command_flag_cat_replace, exec_command_hook);
}
