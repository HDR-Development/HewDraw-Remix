use std::collections::HashMap;

use module::{DynamicModule, Module};
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use smash::{
    app::{lua_bind::StatusModule, BattleObjectModuleAccessor},
    phx::Hash40,
};

use crate::{Variable, VariableAccessor, VariableId, VariableIndex};

#[derive(Copy, Clone)]
struct VariableIdState {
    max_index: VariableIndex,
    status_reset_callback: fn(i32, &mut dyn VariableAccessor),
}

static VAR_AMOUNTS: Lazy<RwLock<HashMap<Hash40, VariableIdState>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

pub struct VarModule {
    flags: Vec<bool>,
    words: Vec<u32>,
    dwords: Vec<u64>,
    floats: Vec<f32>,
    reset_callback: fn(i32, &mut dyn VariableAccessor),
}

impl VarModule {
    pub fn get<V: VariableId, T: Variable>(
        module_accessor: *const BattleObjectModuleAccessor,
        id: V,
    ) -> T {
        let this = module::get_module_by_name::<Self>(module_accessor);
        let status = unsafe { StatusModule::status_kind(module_accessor as _) };

        id.read(status, this)
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn set<V: VariableId, T: Variable>(
        module_accessor: *mut BattleObjectModuleAccessor,
        id: V,
        value: T,
    ) {
        let this = module::get_module_by_name::<Self>(module_accessor);
        let status = unsafe { StatusModule::status_kind(module_accessor) };

        id.write(value, status, this);
    }

    pub fn reset_status(module_accessor: *mut BattleObjectModuleAccessor, status: i32) {
        let this = module::get_module_by_name::<Self>(module_accessor);

        (this.reset_callback)(status, this);
    }

    pub fn reset(module_accessor: *mut BattleObjectModuleAccessor) {
        let this = module::get_module_by_name::<Self>(module_accessor);

        this.flags.fill(false);
        this.words.fill(0);
        this.dwords.fill(0);
        this.floats.fill(0.0);
    }
}

impl VariableAccessor for VarModule {
    fn write_flag(&mut self, index: usize, flag: bool) {
        self.flags[index] = flag;
    }

    fn write_int(&mut self, index: usize, int: i32) {
        self.words[index] = int as u32;
    }

    fn write_uint(&mut self, index: usize, uint: u32) {
        self.words[index] = uint;
    }

    fn write_long(&mut self, index: usize, long: i64) {
        self.dwords[index] = long as u64;
    }

    fn write_ulong(&mut self, index: usize, ulong: u64) {
        self.dwords[index] = ulong;
    }

    fn write_float(&mut self, index: usize, float: f32) {
        self.floats[index] = float;
    }

    fn read_flag(&self, index: usize) -> bool {
        self.flags[index]
    }

    fn read_int(&self, index: usize) -> i32 {
        self.words[index] as i32
    }

    fn read_uint(&self, index: usize) -> u32 {
        self.words[index]
    }

    fn read_long(&self, index: usize) -> i64 {
        self.dwords[index] as i64
    }

    fn read_ulong(&self, index: usize) -> u64 {
        self.dwords[index]
    }

    fn read_float(&self, index: usize) -> f32 {
        self.floats[index]
    }
}

pub fn add_var_amount<T: VariableId>(hash: Hash40) {
    VAR_AMOUNTS.write().insert(
        hash,
        VariableIdState {
            max_index: T::MAX_INDEX,
            status_reset_callback: T::clear_status,
        },
    );
}

impl Module for VarModule {
    const NAME: &'static str = "VarModule";

    fn new(init_args: module::InitArgs) -> Option<Self> {
        let state = VAR_AMOUNTS
            .read()
            .get(&dbg!(init_args.agent_kind_hash))
            .copied()?;
        Some(Self {
            flags: vec![false; state.max_index.flag],
            words: vec![0u32; state.max_index.word],
            dwords: vec![0u64; state.max_index.dword],
            floats: vec![0.0f32; state.max_index.float],
            reset_callback: state.status_reset_callback,
        })
    }
}

impl DynamicModule for VarModule {
    fn start(&mut self, _args: module::StartArgs) {
        println!("Module start");
    }

    fn end(&mut self) {
        println!("Module end");
    }

    fn finalize(&mut self) {
        println!("Module finalize");
    }
}
