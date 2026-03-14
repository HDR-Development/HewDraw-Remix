use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3lib9SingletonIN3app19FighterCutInManagerEE9instance_E"]
        pub(super) static INSTANCE: *mut app::FighterCutInManager;

        #[link_name = "_ZN3app19FighterCutInManager13is_one_on_oneEv"]
        pub(super) fn is_one_on_one() -> bool;
    
        #[link_name = "_ZN3app19FighterCutInManager30is_one_on_one_including_thrownERKNS_26BattleObjectModuleAccessorE"]
        pub(super) fn is_one_on_one_including_thrown(module_accessor: *const app::BattleObjectModuleAccessor) -> bool;
    
        #[link_name = "_ZN3app19FighterCutInManager10is_vr_modeEv"]
        pub(super) fn is_vr_mode() -> bool;
    
        #[link_name = "_ZN3app8lua_bind34FighterCutInManager__add_task_implEPNS_19FighterCutInManagerEj"]
        pub(super) fn add_task(this: *mut app::FighterCutInManager, battle_object_id: u32);

        #[link_name = "_ZN3app8lua_bind34FighterCutInManager__is_owner_implEPNS_19FighterCutInManagerERKNS_26BattleObjectModuleAccessorE"]
        pub(super) fn is_owner(this: *const app::FighterCutInManager, module_accessor: *const app::BattleObjectModuleAccessor) -> bool;

        #[link_name = "_ZN3app8lua_bind33FighterCutInManager__is_play_implEPNS_19FighterCutInManagerE"]
        pub(super) fn is_play(this: *const app::FighterCutInManager) -> bool;

        #[link_name = "_ZN3app8lua_bind47FighterCutInManager__is_play_motion_camera_implEPNS_19FighterCutInManagerE"]
        pub(super) fn is_play_motion_camera(this: *const app::FighterCutInManager) -> bool;

        #[link_name = "_ZN3app8lua_bind40FighterCutInManager__is_play_status_implEPNS_19FighterCutInManagerE"]
        pub(super) fn is_play_status(this: *const app::FighterCutInManager) -> bool;

        #[link_name = "_ZN3app8lua_bind37FighterCutInManager__request_end_implEPNS_19FighterCutInManagerE"]
        pub(super) fn request_end(this: *mut app::FighterCutInManager);

        #[link_name = "_ZN3app8lua_bind39FighterCutInManager__request_start_implEPNS_19FighterCutInManagerERNS_26BattleObjectModuleAccessorEPNS_15CutInTransactorENS_9CutInTypeEPKNS_9CutInDataENS_13CutInPriorityE"]
        pub(super) fn request_start(this: *mut app::FighterCutInManager, module_accessor: *mut app::BattleObjectModuleAccessor, transactor: *mut super::CutInTransactorImpl, ty: app::CutInType, data: *const app::CutInData, priority: app::CutInPriority);

        #[link_name = "_ZN3app8lua_bind49FighterCutInManager__set_throw_finish_offset_implEPNS_19FighterCutInManagerEN3phx8Vector3fE"]
        pub(super) fn set_throw_finish_offset(this: *mut app::FighterCutInManager, offset: phx::Vector3f);

        #[link_name = "_ZN3app8lua_bind52FighterCutInManager__set_throw_finish_zoom_rate_implEPNS_19FighterCutInManagerEf"]
        pub(super) fn set_throw_finish_zoom_rate(this: *mut app::FighterCutInManager, rate: f32);

        // Related to FighterPitBCutInTransactorFinal below
        #[link_name = "_ZN3app31FighterPitBCutInTransactorFinal8instanceEv"]
        pub(super) fn instance() -> *mut app::FighterPitBCutInTransactorFinal;
    }
}

pub trait CutInTransactor {
    fn handle_cut_in(&mut self, module_accessor: &mut app::BattleObjectModuleAccessor) -> bool;
}

#[allow(dead_code)]
struct CutInTransactorVTable {
    pub destructor: extern "C" fn(&mut CutInTransactorImpl),
    pub deleter: extern "C" fn(*mut CutInTransactorImpl),
    pub handle_cut_in: extern "C" fn(&mut CutInTransactorImpl, &mut app::BattleObjectModuleAccessor) -> bool
}

static CUT_IN_TRANSACTOR_VTABLE: CutInTransactorVTable = CutInTransactorVTable {
    destructor: CutInTransactorImpl::destructor,
    deleter: CutInTransactorImpl::deleter,
    handle_cut_in: CutInTransactorImpl::handle_cut_in
};

#[repr(C)]
struct CutInTransactorImpl {
    vtable: &'static CutInTransactorVTable,
    object: Box<dyn CutInTransactor>
}

impl CutInTransactorImpl {
    pub extern "C" fn destructor(&mut self) {
        unimplemented!()
    }

    pub extern "C" fn deleter(_: *mut Self) {
        unimplemented!()
    }

    pub extern "C" fn handle_cut_in(&mut self, module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
        self.object.handle_cut_in(module_accessor)
    }

    pub fn new(transactor: Box<dyn CutInTransactor>) -> Self {
        Self {
            vtable: &CUT_IN_TRANSACTOR_VTABLE,
            object: transactor
        }
    }
}

#[repr(C)]
pub struct CutInData;

#[repr(C)]
pub struct FighterCutInManager {}

impl FighterCutInManager {
    pub fn instance() -> Option<&'static Self> {
        unsafe {
            impl_::INSTANCE.as_ref()
        }
    }

    pub fn instance_mut() -> Option<&'static mut Self> {
        unsafe {
            impl_::INSTANCE.as_mut()
        }
    }

    pub fn is_one_on_one() -> bool {
        unsafe {
            impl_::is_one_on_one()
        }
    }
    
    pub fn is_one_on_one_including_thrown(module_accessor: &app::BattleObjectModuleAccessor) -> bool {
        unsafe {
            impl_::is_one_on_one_including_thrown(module_accessor)
        }
    }
    
    pub fn is_vr_mode() -> bool {
        unsafe {
            impl_::is_vr_mode()
        }
    }

    pub fn add_task(&mut self, battle_object_id: u32) {
        unsafe {
            impl_::add_task(self, battle_object_id);
        }
    }

    pub fn is_owner(&self, module_accessor: &app::BattleObjectModuleAccessor) -> bool {
        unsafe {
            impl_::is_owner(self, module_accessor)
        }
    }

    pub fn is_play(&self) -> bool {
        unsafe {
            impl_::is_play(self)
        }
    }

    pub fn is_play_motion_camera(&self) -> bool {
        unsafe {
            impl_::is_play_motion_camera(self)
        }
    }

    pub fn is_play_status(&self) -> bool {
        unsafe {
            impl_::is_play_status(self)
        }
    }

    pub fn request_end(&mut self) {
        unsafe {
            impl_::request_end(self)
        }
    }

    pub fn request_start(
        &mut self,
        module_accessor: &mut app::BattleObjectModuleAccessor,
        transactor: Option<Box<dyn CutInTransactor>>,
        ty: app::CutInType,
        data: Option<&CutInData>,
        priority: app::CutInPriority
    )
    {
        let transactor = transactor.map(|x| CutInTransactorImpl::new(x));
        unsafe {
            let data_ptr = data.map_or_else(|| std::ptr::null(), |x| x as *const CutInData);
            if let Some(mut transactor) = transactor {
                impl_::request_start(
                    self,
                    module_accessor,
                    &mut transactor,
                    ty,
                    data_ptr,
                    priority
                );
            } else {
                impl_::request_start(
                    self,
                    module_accessor,
                    std::ptr::null_mut(),
                    ty,
                    data_ptr,
                    priority
                );
            }
        }
    }

    pub fn set_throw_finish_offset(&mut self, offset: phx::Vector3f) {
        unsafe {
            impl_::set_throw_finish_offset(self, offset)
        }
    }

    pub fn set_throw_finish_zoom_rate(&mut self, rate: f32) {
        unsafe {
            impl_::set_throw_finish_zoom_rate(self, rate)
        }
    }
}

// I don't really have a better place to put this because it's not worth one line
#[repr(C)]
pub struct FighterPitBCutInTransactorFinal {
    vtable: &'static CutInTransactorVTable,
    // ...
}

impl FighterPitBCutInTransactorFinal {
    pub fn instance() -> Option<&'static Self> {
        unsafe {
            impl_::instance().as_ref()
        }
    }

    pub fn instance_mut() -> Option<&'static mut Self> {
        unsafe {
            impl_::instance().as_mut()
        }
    }
}