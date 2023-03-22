#![feature(ptr_metadata)]
#![feature(vec_into_raw_parts)]
use std::any::Any;
use std::any::TypeId;

pub fn get_module_by_name<T: Module + DynamicModule>(
    module_accessor: *const smash::app::BattleObjectModuleAccessor,
) -> &'static mut T {
    vmanip::ModuleTable::from_accessor(module_accessor).find()
}

mod vmanip;

use smash::app::BattleObjectModuleAccessor;
pub use vmanip::InitArgs;
pub use vmanip::StartArgs;

pub trait Module: Sized {
    const NAME: &'static str;

    fn new(module_accessor: *mut BattleObjectModuleAccessor, init_args: InitArgs) -> Option<Self>;
}

pub trait DynamicModule: Any + 'static {
    fn module_type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }

    #[allow(unused_variables)]
    fn listen_events(&mut self, module_accessor: *mut BattleObjectModuleAccessor) {}

    #[allow(unused_variables)]
    fn start(&mut self, args: StartArgs) {}

    fn end(&mut self) {}

    fn finalize(&mut self) {}

    fn process_begin(&mut self) {}

    fn process_begin2(&mut self) {}

    fn process_ground(&mut self) {}

    fn process_map_collision(&mut self) {}

    fn process_fix_position(&mut self) {}

    fn process_pre_collision(&mut self) {}

    fn process_collision(&mut self) {}

    fn process_hit(&mut self) {}

    fn process_fix_camera(&mut self) {}

    fn process_end(&mut self) {}

    fn process_end2(&mut self) {}
}

fn cast<T: Any>(module: &'static mut dyn DynamicModule) -> &'static mut T {
    if module.module_type_id() == std::any::TypeId::of::<T>() {
        unsafe {
            let (ptr, _) = (module as *mut dyn DynamicModule).to_raw_parts();
            &mut *(ptr as *mut T)
        }
    } else {
        panic!("module was not {}", std::any::type_name::<T>());
    }
}

pub use vmanip::add_module as add;

pub fn install() {
    vmanip::install();
}
