use crate::{DynamicModule, Module};
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use smash::{
    app::{BattleObject, BattleObjectModuleAccessor},
    phx::Hash40,
};
use std::collections::HashMap;
use virtmanip::Vtable;

const BOMA_MAGIC: [u8; 8] = [b'B', b'O', b'M', b'A', b'D', b'E', b'E', b'Z'];
const OBJECT_MAGIC: [u8; 8] = [b'O', b'B', b'J', b'E', b'C', b'T', b' ', b' '];

type BoxedInit = Box<
    dyn Fn(*mut BattleObjectModuleAccessor, InitArgs) -> Option<Box<dyn DynamicModule>>
        + 'static
        + Send
        + Sync,
>;

static REGISTERED_MODULES: Lazy<RwLock<HashMap<&'static str, BoxedInit>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

/// Adds a module to be registered on a [`BattleObjectModuleAccessor`] at runtime
pub fn add_module<T: Module + DynamicModule>() {
    let func = Box::new(
        |module_accessor: *mut BattleObjectModuleAccessor, init_args: InitArgs| {
            T::new(module_accessor, init_args)
                .map(|module| Box::new(module) as Box<dyn DynamicModule>)
        },
    );

    REGISTERED_MODULES.write().insert(T::NAME, func);
}

struct DynamicModuleInfo {
    name: &'static str,
    module: *mut u8,
}

impl DynamicModuleInfo {
    pub fn as_dyn_mod(&self) -> &'static mut dyn DynamicModule {
        unsafe { (*(self.module as *mut Box<dyn DynamicModule>)).as_mut() }
    }

    pub fn as_mod<T>(&self) -> &'static mut T {
        super::cast(self.as_dyn_mod())
    }
}

impl Drop for DynamicModuleInfo {
    fn drop(&mut self) {
        unsafe {
            drop(Box::from_raw(self.module as *mut Box<dyn DynamicModule>));
        }
    }
}

pub(crate) struct ModuleTable(&'static mut Vec<DynamicModuleInfo>);

impl ModuleTable {
    /// Gets the module table from this accessor
    ///
    /// # Panicking
    /// Panics if the module accessor does not have an associated module table
    pub fn from_accessor(module_accessor: *const BattleObjectModuleAccessor) -> Self {
        let vtable = unsafe { &((*module_accessor).vtable as Vtable) };
        let entries = virtmanip::entries(BOMA_MAGIC, vtable).unwrap();

        Self(unsafe { &mut *(entries[0] as *mut Vec<DynamicModuleInfo>) })
    }

    /// Attempts to get the module table from this accessor
    pub fn try_from_accessor(module_accessor: *const BattleObjectModuleAccessor) -> Option<Self> {
        let vtable = unsafe { &((*module_accessor).vtable as Vtable) };
        virtmanip::entries(BOMA_MAGIC, vtable)
            .map(|entries| Self(unsafe { &mut *(entries[0] as *mut Vec<DynamicModuleInfo>) }))
            .ok()
    }

    /// Find the dynamic module associated of the provided type
    ///
    /// # Panicking
    /// Panics if the module table does not have a module of the associated type
    pub fn find<T: Module + DynamicModule>(&mut self) -> &'static mut T {
        for info in self.0.iter() {
            if info.name == T::NAME {
                return info.as_mod();
            }
        }

        panic!(
            "BattleObjectModuleAccessor has no module with the name {}",
            T::NAME
        );
    }

    /// Runs [`DynamicModule::start`] on all modules in this table
    pub fn start(&mut self, args: StartArgs) {
        for info in self.0.iter_mut() {
            info.as_dyn_mod().start(args);
        }
    }

    /// Runs [`DynamicModule::end`] on all modules in this table
    pub fn end(&mut self) {
        for info in self.0.iter_mut() {
            info.as_dyn_mod().end();
        }
    }

    /// Runs [`DynamicModule::finalize`] on all modules in this table
    pub fn finalize(&mut self) {
        for info in self.0.iter_mut() {
            info.as_dyn_mod().finalize();
        }
    }
}

#[derive(Copy, Clone)]
#[repr(transparent)]
struct ModuleInitArgs(*const u8);

impl ModuleInitArgs {
    pub fn at<T: Copy>(self, offset: usize) -> T {
        unsafe { *(self.0.add(offset) as *const T) }
    }
}

#[derive(Copy, Clone)]
#[repr(transparent)]
struct ModuleStartArgs(*const u8);

impl ModuleStartArgs {
    #[allow(dead_code)]
    pub fn at<T: Copy>(self, offset: usize) -> T {
        unsafe { *(self.0.add(offset) as *const T) }
    }
}

const INITIALIZE_MODULES_OFFSET: usize = 0x3af2e0;
const START_MODULES_OFFSET: usize = 0x3af9f0;
const END_MODULES_OFFSET: usize = 0x3afde0;
const FINALIZE_MODULES_OFFSET: usize = 0x3af700;

/// Arguments provided to modules on [`Module::new`]
#[derive(Debug, Copy, Clone)]
pub struct InitArgs {
    /// The hash of the agent
    pub agent_kind_hash: Hash40,

    /// The type-erased agent kind
    pub agent_kind: i32,

    /// The ID of the battle object
    pub battle_object_id: u32,

    /// The category of the battle object
    pub battle_object_category: u8,
}

impl From<ModuleInitArgs> for InitArgs {
    fn from(value: ModuleInitArgs) -> Self {
        Self {
            agent_kind_hash: value.at(160),
            agent_kind: value.at(156),
            battle_object_id: value.at(8),
            battle_object_category: value.at(408),
        }
    }
}

/// Arguments provided to modules on [`DynamicModule::start`]
#[derive(Debug, Copy, Clone)]
pub struct StartArgs {}

impl From<ModuleStartArgs> for StartArgs {
    fn from(_value: ModuleStartArgs) -> Self {
        Self {}
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct ModuleAccessorVTable {
    is_virtual: extern "C" fn(*const BattleObjectModuleAccessor) -> bool,
    destructor: extern "C" fn(*mut BattleObjectModuleAccessor),
    deleter: extern "C" fn(*mut BattleObjectModuleAccessor),
    initialize_modules: extern "C" fn(*mut BattleObjectModuleAccessor, args: ModuleInitArgs),
    finalize_modules: extern "C" fn(*mut BattleObjectModuleAccessor),
    start_modules:
        extern "C" fn(*mut BattleObjectModuleAccessor, args: ModuleStartArgs, entry_id: u32),
    end_modules: extern "C" fn(*mut BattleObjectModuleAccessor),
}

#[repr(C)]
struct NewVTable {
    p_old: u64,
    modules: &'static mut Vec<DynamicModuleInfo>,
    magic: u64,
    old: ModuleAccessorVTable,
}

#[skyline::from_offset(0x3ac540)]
fn get_battle_object_from_id(id: u32) -> Option<&'static mut BattleObject>;

#[skyline::hook(offset = INITIALIZE_MODULES_OFFSET)]
fn boma_init(module_accessor: *mut BattleObjectModuleAccessor, args: ModuleInitArgs) {
    call_original!(module_accessor, args);

    let init_args = InitArgs::from(args);
    let modules = REGISTERED_MODULES.read();

    let mut init_modules = vec![];

    for (name, init) in modules.iter() {
        if let Some(module) = init(module_accessor, init_args) {
            let module = Box::leak(Box::new(module));
            module.listen_events(module_accessor);
            init_modules.push(DynamicModuleInfo {
                name,
                module: module as *mut _ as *mut u8,
            });
        }
    }

    if init_modules.is_empty() {
        return;
    }

    let vtable = unsafe { &mut ((*module_accessor).vtable as Vtable) };

    virtmanip::rebuild(BOMA_MAGIC, None, 1, vtable).unwrap();
    let entries = virtmanip::entries(BOMA_MAGIC, vtable).unwrap();

    entries[0] = Box::leak(Box::new(init_modules)) as *mut _ as u64;

    if let Some(object) = unsafe { get_battle_object_from_id((*module_accessor).battle_object_id) }
    {
        patch_object_vtable(object);
    }
}

#[skyline::hook(offset = START_MODULES_OFFSET)]
fn boma_start(
    module_accessor: *mut BattleObjectModuleAccessor,
    args: ModuleStartArgs,
    entry_id: u32,
) {
    call_original!(module_accessor, args, entry_id);
    let start_args = StartArgs::from(args);
    if let Some(mut table) = ModuleTable::try_from_accessor(module_accessor) {
        table.start(start_args);
    }
}

#[skyline::hook(offset = END_MODULES_OFFSET)]
fn boma_end(module_accessor: *mut BattleObjectModuleAccessor) {
    call_original!(module_accessor);
    if let Some(mut table) = ModuleTable::try_from_accessor(module_accessor) {
        table.end();
    }
}

#[skyline::hook(offset = FINALIZE_MODULES_OFFSET)]
fn boma_fini(module_accessor: *mut BattleObjectModuleAccessor) {
    call_original!(module_accessor);
    if let Some(mut table) = ModuleTable::try_from_accessor(module_accessor) {
        table.finalize();

        unsafe {
            drop(Box::from_raw(table.0));
            let vtable = Box::from_raw((*module_accessor).vtable as *mut NewVTable);
            (*module_accessor).vtable = vtable.p_old;
            drop(vtable);
            if let Some(object) = get_battle_object_from_id((*module_accessor).battle_object_id) {
                restore_object_vtable(object);
            }
        }
    }
}

const PROCESS_BEGIN_VIRT_OFFSET: usize = 0x6A;
const PROCESS_BEGIN2_VIRT_OFFSET: usize = 0x6B;
const PROCESS_GROUND_VIRT_OFFSET: usize = 0x6C;
const PROCESS_MAP_COLLISION_VIRT_OFFSET: usize = 0x6D;
const PROCESS_FIX_POSITION_VIRT_OFFSET: usize = 0x6E;
const PROCESS_PRE_COLLISION_VIRT_OFFSET: usize = 0x6F;
const PROCESS_COLLISION_VIRT_OFFSET: usize = 0x70;
const PROCESS_HIT_VIRT_OFFSET: usize = 0x71;
const PROCESS_FIX_CAMERA_VIRT_OFFSET: usize = 0x72;
const PROCESS_END_VIRT_OFFSET: usize = 0x73;
const PROCESS_END2_VIRT_OFFSET: usize = 0x74;

macro_rules! original {
    ($object:ident, $off:expr) => {{
        unsafe {
            let vtable = (*$object).vtable;
            let original_vtable = *(vtable as *const u64).sub(2);
            let function = *(original_vtable as *const u64).add($off / 8);
            let callable: extern "C" fn(*mut BattleObject) = std::mem::transmute(function);
            callable($object);
        }
    }};
}

macro_rules! define_replace {
    ($(($operation:ident, $offset:expr)),*) => {
        $(
            paste::paste! {
                extern "C" fn [<$operation _replace>](object: *mut BattleObject) {
                    let module_accessor = unsafe {
                        (*object).module_accessor
                    };
                    if let Some(mut table) = ModuleTable::try_from_accessor(module_accessor) {
                        table.$operation();
                    }
                    original!(object, $offset);
                }
            }
        )*

        impl ModuleTable {
            $(
                pub fn $operation(&mut self) {
                    for info in self.0.iter_mut() {
                        info.as_dyn_mod().$operation();
                    }
                }
            )*
        }

        fn patch_object_vtable(object: *mut BattleObject) {
            let vtable = unsafe { &mut ((*object).vtable as Vtable) };
            virtmanip::rebuild(OBJECT_MAGIC, None, 0, vtable).unwrap();

            paste::paste! {
                $(
                    virtmanip::replace_function(OBJECT_MAGIC, $offset, [<$operation _replace>] as *const (), vtable).unwrap();
                )*
            }
        }

        fn restore_object_vtable(object: *mut BattleObject) {
            let vtable = unsafe { &mut ((*object).vtable as Vtable) };
            virtmanip::restore(OBJECT_MAGIC, vtable).unwrap();
        }
    }
}

define_replace! {
    (process_begin, PROCESS_BEGIN_VIRT_OFFSET),
    (process_begin2, PROCESS_BEGIN2_VIRT_OFFSET),
    (process_ground, PROCESS_GROUND_VIRT_OFFSET),
    (process_map_collision, PROCESS_MAP_COLLISION_VIRT_OFFSET),
    (process_fix_position, PROCESS_FIX_POSITION_VIRT_OFFSET),
    (process_pre_collision, PROCESS_PRE_COLLISION_VIRT_OFFSET),
    (process_collision, PROCESS_COLLISION_VIRT_OFFSET),
    (process_hit, PROCESS_HIT_VIRT_OFFSET),
    (process_fix_camera, PROCESS_FIX_CAMERA_VIRT_OFFSET),
    (process_end, PROCESS_END_VIRT_OFFSET),
    (process_end2, PROCESS_END2_VIRT_OFFSET)
}

pub fn install() {
    skyline::install_hooks!(boma_init, boma_start, boma_end, boma_fini);
}
