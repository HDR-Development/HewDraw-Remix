pub use params_macros::WorkParams;

pub mod __private {
    pub use smash;
}

mod module;

pub use self::module::{add, MarioFireballParams, ParamModule};

pub trait WorkParams: Sized {
    fn parse(
        module_accessor: *mut smash::app::BattleObjectModuleAccessor,
        object_name: &'static str,
    ) -> Self;
}

pub trait WorkParamObject: WorkParams {
    const NAME: &'static str;
}
