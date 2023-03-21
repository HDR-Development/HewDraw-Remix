use crate::{DynamicModule, Module};
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use smash::{app::BattleObjectModuleAccessor, phx::Hash40};
use std::collections::HashMap;

const HDR_MAGIC: u64 = u64::from_le_bytes([b'H', b'D', b'R', b'M', b'A', b'G', b'I', b'C']);

type BoxedInit = Box<dyn Fn(InitArgs) -> Option<Box<dyn DynamicModule>> + 'static + Send + Sync>;

static REGISTERED_MODULES: Lazy<RwLock<HashMap<&'static str, BoxedInit>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

/// Adds a module to be registered on a [`BattleObjectModuleAccessor`] at runtime
pub fn add_module<T: Module + DynamicModule>() {
    let func = Box::new(|init_args: InitArgs| {
        T::new(init_args).map(|module| Box::new(module) as Box<dyn DynamicModule>)
    });

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
        Self::try_from_accessor(module_accessor)
            .expect("BattleObjectModuleAccessor does not have module table")
    }

    /// Attempts to get the module table from this accessor
    pub fn try_from_accessor(module_accessor: *const BattleObjectModuleAccessor) -> Option<Self> {
        unsafe {
            let vtable = (*module_accessor).vtable;
            let hdr_magic = *(vtable as *const u64).sub(1);
            if hdr_magic != HDR_MAGIC {
                return None;
            }

            let raw = *(vtable as *const u64).sub(2) as *mut Vec<DynamicModuleInfo>;

            Some(Self(&mut *raw))
        }
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

#[skyline::hook(offset = INITIALIZE_MODULES_OFFSET)]
fn boma_init(module_accessor: *mut BattleObjectModuleAccessor, args: ModuleInitArgs) {
    call_original!(module_accessor, args);

    let init_args = InitArgs::from(args);
    let modules = REGISTERED_MODULES.read();

    let mut init_modules = vec![];

    for (name, init) in modules.iter() {
        if let Some(module) = init(init_args) {
            init_modules.push(DynamicModuleInfo {
                name,
                module: Box::leak(Box::new(module)) as *mut _ as *mut u8,
            });
        }
    }

    if !init_modules.is_empty() {
        unsafe {
            let new_vtable = Box::leak(Box::new(NewVTable {
                p_old: (*module_accessor).vtable,
                modules: Box::leak(Box::new(init_modules)),
                magic: HDR_MAGIC,
                old: *((*module_accessor).vtable as *const ModuleAccessorVTable),
            }));
            (*module_accessor).vtable = &new_vtable.old as *const ModuleAccessorVTable as u64;
        }
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
        }
    }
}

pub fn install() {
    skyline::install_hooks!(boma_init, boma_start, boma_end, boma_fini);
}
