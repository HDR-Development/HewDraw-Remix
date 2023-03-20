#![feature(ptr_metadata)]
use std::any::Any;

pub fn get_module_by_name<T: Module + DynamicModule>(
    module_accessor: *const smash::app::BattleObjectModuleAccessor,
) -> &'static mut T {
    vmanip::ModuleTable::from_accessor(module_accessor).find()
}

mod vmanip;

pub use vmanip::InitArgs;
pub use vmanip::StartArgs;

pub trait Module: Sized {
    const NAME: &'static str;

    fn new(init_args: InitArgs) -> Option<Self>;
}

pub trait DynamicModule: Any + 'static {
    #[allow(unused_variables)]
    fn start(&mut self, args: StartArgs) {}
    fn end(&mut self) {}
    fn finalize(&mut self) {}
}

fn cast<T: Any>(module: &'static mut dyn DynamicModule) -> &'static mut T {
    if module.type_id() == std::any::TypeId::of::<T>() {
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
