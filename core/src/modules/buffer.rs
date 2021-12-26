use smash::app::{BattleObject, lua_bind::ControlModule, BattleObjectModuleAccessor};

use crate::offsets;
use crate::utils::get_battle_object_from_id;

use super::BUFFER_MODULE_OFFSET;

macro_rules! get_buffer_module {
    ($object:ident) => {{
        unsafe {
            let vtable = *($object as *mut *mut *mut u64);
            &mut *super::get_entry::<BufferModule>(vtable, BUFFER_MODULE_OFFSET).expect("Did not find BufferModule!")
        }
    }}
}

macro_rules! has_buffer_module {
    ($object:ident) => {{
        unsafe {
            if $object.is_null() {
                false
            } else {
                let vtable = *($object as *mut *mut *mut u64);
                super::is_hdr_object(vtable as _) && !super::get_entry::<BufferModule>(vtable, BUFFER_MODULE_OFFSET).is_none()
            }
        }
    }}
}

macro_rules! require_buffer_module {
    ($object:ident) => {{
        if !has_buffer_module!($object) {
            panic!("BattleObject does not contain reference to BufferModule!");
        }
        get_buffer_module!($object)
    }}
}

struct BufferState {
    pub on_last_frame: u32,
    pub should_hold: [bool; 32],
    pub hold_frame: [i32; 32],
    pub hold_frame_max: [i32; 32]
}

impl BufferState {
    pub fn new() -> Self {
        Self {
            on_last_frame: 0,
            should_hold: [false; 32],
            hold_frame: [0; 32],
            hold_frame_max: [-1; 32]
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
        should_hold: bool
    ) {
        self.on_last_frame = 0;
        for (idx, x) in game_held.iter_mut().enumerate() {
            if *x != 0
            && (self.hold_frame[idx] < press_frame || self.should_hold[idx] || should_hold || *x != 1) {
                self.hold_frame[idx] += 1;
                if self.hold_frame[idx] < press_frame {
                    continue;
                }
                if *x == 1 {
                    if self.should_hold[idx] {
                        if self.hold_frame_max[idx] != -1 && self.hold_frame_max[idx] < self.hold_frame[idx] {
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
/// `BufferModule` set up, it will panic.
pub struct BufferModule {
    owner: *mut BattleObject,
    cats: [BufferState; 4],
    hold_all: bool,
    hold_all_frame_max: i32
}

impl BufferModule {
    /// Constructs a new `BufferModule` instance with the specified `BattleObject` as its own
    /// # Arguments
    /// * `owner` - Owning `BattleObject` instance
    /// # Returns
    /// New, constructed `BufferModule` with all settings set to default
    pub(crate) fn new(owner: *mut BattleObject) -> Self {
        Self {
            owner,
            cats: [
                BufferState::new(),
                BufferState::new(),
                BufferState::new(),
                BufferState::new()
            ],
            hold_all: false,
            hold_all_frame_max: -1
        }
    }

    /// Enables the hold buffer on one specific input.
    /// # Arguments
    /// * `object` - Owning `BattleObject` instance
    /// * `category` - Which command flag category the input is under (valid values are 0-3)
    /// * `flag` - Which flag in the category you are enabling hold buffer for
    #[export_name = "BufferModule__persist_command_one"]
    pub extern "Rust" fn persist_command_one(object: *mut BattleObject, category: i32, flag: i32) {
        let module = require_buffer_module!(object);

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
    #[export_name = "BufferModule__persist_command_one_with_lifetime"]
    pub extern "Rust" fn persist_command_one_with_lifetime(object: *mut BattleObject, category: i32, flag: i32, lifetime: i32) {
        let module = require_buffer_module!(object);

        Self::persist_command_one(object, category, flag);
        module.cats[category as usize].hold_frame_max[(flag & 0x1F) as usize] = lifetime;
    }

    /// Sets the global hold buffer lifetime
    /// # Arguments
    /// * `object` - Owning `BattleObject` instance
    /// * `lifetime` - The maximum number of frames hold buffer is enabled for (-1 is infinite). This lifetime includes tap buffer frames.
    #[export_name = "BufferModule__set_persist_lifetime"]
    pub extern "Rust" fn set_persist_lifetime(object: *mut BattleObject, lifetime: i32) {
        require_buffer_module!(object).hold_all_frame_max = lifetime;
    }

    /// Enables global hold buffer on all inputs for this `BattleObject`
    /// # Arguments
    /// * `object` - Owning `BattleObject` instance
    #[export_name = "BufferModule__enable_persist"]
    pub extern "Rust" fn enable_persist(object: *mut BattleObject) {
        require_buffer_module!(object).hold_all = true;
    }

    /// Disables global hold buffer for this `BattleObject`
    /// # Arguments
    /// * `object` - Owning `BattleObject` instance
    /// # Note
    /// If specific inputs have hold buffer enabled, calling `disable_persist` will not disable those,
    /// only the global flag which enabled hold buffer on all inputs will be disabled
    #[export_name = "BufferModule__disable_persist"]
    pub extern "Rust" fn disable_persist(object: *mut BattleObject) {
        require_buffer_module!(object).hold_all = false;
    }

    /// Clears all of the hold buffer information for every input
    /// # Arguments
    /// * `object` - Owning `BattleObject` instance
    /// # Note
    /// This function is similar to `ControlModule::clear_command_flag_cat` in that it resets all information regarding holding those inputs.
    /// This does not impact anything in the `ControlModule` command information, only the `BufferModule` implementation
    #[export_name = "BufferModule__clear_persist"]
    pub extern "Rust" fn clear_persist(object: *mut BattleObject) {
        let module = require_buffer_module!(object);

        module.cats[0].clear();
        module.cats[1].clear();
        module.cats[2].clear();
        module.cats[3].clear();
    }

    /// Clears the hold buffer information for one input
    /// # Arguments
    /// * `object` - Owning `BattleObject` instance
    /// * `category` - Which command flag category the input is under (valid values are 0-3)
    /// * `flag` - Which flag in the category you are clearing hold buffer for
    #[export_name = "BufferModule__clear_persist_one"]
    pub extern "Rust" fn clear_persist_one(object: *mut BattleObject, category: i32, flag: i32) {
        let module = require_buffer_module!(object);
        let cat = &mut module.cats[category as usize];
        cat.on_last_frame &= !(1 << (flag as usize));
        cat.should_hold[flag as usize] = false;
        cat.hold_frame[flag as usize] = 0;
        cat.hold_frame_max[flag as usize] = -1;
    }

    /// Updates the hold buffer information
    /// # Arguments
    /// * `object` - Owning `BattleObject` instance
    /// * `cats` - `ControlModule` command flag information to update `BufferModule` with.
    /// # Note
    /// This method is not intended to be used by users of `BufferModule`. It is instead used internally with a hook to update every frame.
    #[export_name = "BufferModule__exec"]
    pub extern "Rust" fn exec(object: *mut BattleObject, cats: &mut [&mut [u8]; 4]) {
        let module = require_buffer_module!(object);

        let press_frame = unsafe {
            ControlModule::get_command_life_count_max((*module.owner).module_accessor) as i32
        };

        for x in 0..4 {
            module.cats[x].update(cats[x], module.hold_all_frame_max, press_frame -1, module.hold_all);
        }
    }

    /// Checks whether or not global hold buffer is enabled
    /// # Arguments
    /// * `object` - Owning `BattleObject` instance
    /// # Returns
    /// A boolean representing whether or not global hold buffer is enabled.
    #[export_name = "BufferModule__is_persist"]
    pub extern "Rust" fn is_persist(object: *mut BattleObject) -> bool {
        require_buffer_module!(object).hold_all
    }

    /// Checks whether or not hold buffer is enabled for a specific input
    /// # Arguments
    /// * `object` - Owning `BattleObject` instance
    /// * `category` - Which command flag category the input is under (valid values are 0-3)
    /// * `flag` - Which flag in the category you are checking hold buffer for
    /// # Returns
    /// A boolean representing whether or not hold buffer is enabled for a specific input.
    #[export_name = "BufferModule__is_persist_one"]
    pub extern "Rust" fn is_persist_one(object: *mut BattleObject, category: i32, flag: i32) -> bool {
        require_buffer_module!(object).cats[category as usize].should_hold[flag as usize]
    }

    /// Gets the max amount of global hold buffer frames (can vary depending on input)
    /// # Arguments
    /// * `object` - Owning `BattleObject` instance
    /// # Returns
    /// The number of frames hold buffer is enabled for globally.
    /// #Note
    /// This returns whatever value was last last set with `set_persist_lifetime` and
    /// is a valid value even when `is_persist` is false.
    #[export_name = "BufferModule__persist_lifetime"]
    pub extern "Rust" fn persist_lifetime(object: *mut BattleObject) -> i32 {
        require_buffer_module!(object).hold_all_frame_max
    }

    /// Gets the current amount of frames an object has been holding an input for
    /// # Arguments
    /// * `object` - Owning `BattleObject` instance
    /// * `category` - Which command flag category the input is under (valid values are 0-3)
    /// * `flag` - Which flag in the category you are checking hold buffer for
    /// # Returns
    /// The number of frames the input has been held
    #[export_name = "BufferModule__persist_lifetime_one"]
    pub fn persist_lifetime_one(object: *mut BattleObject, category: i32, flag: i32) -> i32 {
        require_buffer_module!(object).cats[category as usize].hold_frame[flag as usize]
    }

    /// Gets the max amount of hold buffer frames for a specified input
    /// # Arguments
    /// * `object` - Owning `BattleObject` instance
    /// * `category` - Which command flag category the input is under (valid values are 0-3)
    /// * `flag` - Which flag in the category you are checking hold buffer for
    /// # Returns
    /// The max amount of frames a specific input can have hold buffer for.
    #[export_name = "BufferModule__persist_lifetime_max_one"]
    pub fn persist_lifetime_max_one(object: *mut BattleObject, category: i32, flag: i32) -> i32 {
        require_buffer_module!(object).cats[category as usize].hold_frame_max[flag as usize]
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
        unsafe {
            std::slice::from_raw_parts(self.lifetimes, self.count)
        }
    }

    fn lifetimes_mut(&self) -> &mut [u8] {
        unsafe {
            std::slice::from_raw_parts_mut(self.lifetimes, self.count)
        }
    }

    fn lifetimes2(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(self.lifetimes2, self.count)
        }
    }
}

#[skyline::hook(offset = offsets::get_command_flag_cat())]
fn get_command_flag_cat_replace(control_module: u64, cat: i32) -> u32 {
    let cats = unsafe {
        std::slice::from_raw_parts((control_module + 0x568) as *mut CommandFlagCat, 4)
    };

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

#[skyline::hook(offset = offsets::exec_command())]
fn exec_command_hook(control_module: u64, flag: bool) {
    call_original!(control_module, flag);
    
    let cats = unsafe {
        std::slice::from_raw_parts_mut((control_module + 0x568) as *mut CommandFlagCat, 4)
    };

    let mut lifetimes = [
        cats[0].lifetimes_mut(),
        cats[1].lifetimes_mut(),
        cats[2].lifetimes_mut(),
        cats[3].lifetimes_mut(),
    ];

    let boma = unsafe {
        *(control_module as *mut *mut BattleObjectModuleAccessor).add(1)
    };

    let battle_object = unsafe {
        get_battle_object_from_id((*boma).battle_object_id)
    };

    if has_buffer_module!(battle_object) {
        BufferModule::exec(battle_object, &mut lifetimes);
    }
}

pub(crate) fn init() {
    skyline::install_hooks!(
        get_command_flag_cat_replace,
        exec_command_hook
    );
}