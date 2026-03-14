use crate::*;

mod events;
pub use events::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E"]
        pub(super) static INSTANCE: *mut app::FighterManager;

        #[link_name = "_ZN3app8lua_bind38FighterManager__disable_ko_camera_implEPNS_14FighterManagerE"]
        pub(super) fn disable_ko_camera(manager: *mut app::FighterManager);
    
        #[link_name = "_ZN3app8lua_bind37FighterManager__enable_ko_camera_implEPNS_14FighterManagerE"]
        pub(super) fn enable_ko_camera(manager: *mut app::FighterManager);
    
        #[link_name = "_ZN3app8lua_bind32FighterManager__entry_count_implEPNS_14FighterManagerE"]
        pub(super) fn entry_count(manager: *const app::FighterManager) -> u32;
    
        #[link_name = "_ZN3app8lua_bind33FighterManager__exit_finalbg_implEPNS_14FighterManagerE"]
        pub(super) fn exit_finalbg(manager: *mut app::FighterManager);
    
        #[link_name = "_ZN3app8lua_bind31FighterManager__exit_movie_implEPNS_14FighterManagerE"]
        pub(super) fn exit_movie(manager: *mut app::FighterManager);
    
        #[link_name = "_ZN3app8lua_bind49FighterManager__get_beat_point_diff_from_top_implEPNS_14FighterManagerENS_14FighterEntryIDE"]
        pub(super) fn get_beat_point_diff_from_top(manager: *const app::FighterManager, entry_id: app::FighterEntryID) -> i32;
    
        #[link_name = "_ZN3app8lua_bind33FighterManager__get_entry_id_implEPNS_14FighterManagerEi"]
        pub(super) fn get_entry_id(manager: *const app::FighterManager, entry_no: i32) -> app::FighterEntryID;
    
        #[link_name = "_ZN3app8lua_bind33FighterManager__get_entry_no_implEPNS_14FighterManagerENS_14FighterEntryIDE"]
        pub(super) fn get_entry_no(manager: *const app::FighterManager, entry_id: app::FighterEntryID) -> i32;
    
        #[link_name = "_ZN3app8lua_bind38FighterManager__get_fighter_entry_implEPNS_14FighterManagerENS_14FighterEntryIDE"]
        pub(super) fn get_fighter_entry(manager: *const app::FighterManager, entry_id: app::FighterEntryID) -> *mut app::FighterEntry;
    
        #[link_name = "_ZN3app8lua_bind44FighterManager__get_fighter_information_implEPNS_14FighterManagerENS_14FighterEntryIDE"]
        pub(super) fn get_fighter_information(manager: *const app::FighterManager, entry_id: app::FighterEntryID) -> *mut app::FighterInformation;
    
        #[link_name = "_ZN3app8lua_bind36FighterManager__get_fighter_pos_implEPNS_14FighterManagerENS_14FighterEntryIDEi"]
        pub(super) fn get_fighter_pos(manager: *const app::FighterManager, entry_id: app::FighterEntryID, entry_no: i32) -> cpp::simd::Vector3;
    
        #[link_name = "_ZN3app8lua_bind45FighterManager__get_final_actor_entry_id_implEPNS_14FighterManagerE"]
        pub(super) fn get_final_actor_entry_id(manager: *const app::FighterManager) -> app::FighterEntryID;
    
        #[link_name = "_ZN3app8lua_bind42FighterManager__get_jack_final_cut_in_implEPNS_14FighterManagerE"]
        pub(super) fn get_jack_final_cut_in(manager: *const app::FighterManager) -> i32;
    
        #[link_name = "_ZN3app8lua_bind55FighterManager__get_no_discretion_final_beat_count_implEPNS_14FighterManagerE"]
        pub(super) fn get_no_discretion_final_beat_count(manager: *const app::FighterManager) -> u32;
    
        #[link_name = "_ZN3app8lua_bind40FighterManager__get_top_rank_player_implEPNS_14FighterManagerEi"]
        pub(super) fn get_top_rank_player(manager: *const app::FighterManager, arg2: i32) -> u32;
    
        #[link_name = "_ZN3app8lua_bind44FighterManager__get_top_rank_player_num_implEPNS_14FighterManagerE"]
        pub(super) fn get_top_rank_player_num(manager: *const app::FighterManager) -> u8;
    
        #[link_name = "_ZN3app8lua_bind33FighterManager__hide_finalbg_implEPNS_14FighterManagerEj"]
        pub(super) fn hide_finalbg(manager: *mut app::FighterManager, arg2: u32);
    
        #[link_name = "_ZN3app8lua_bind50FighterManager__is_available_discretion_final_implEPNS_14FighterManagerE"]
        pub(super) fn is_available_discretion_final(manager: *const app::FighterManager) -> bool;
    
        #[link_name = "_ZN3app8lua_bind41FighterManager__is_disable_ko_camera_implEPNS_14FighterManagerE"]
        pub(super) fn is_disable_ko_camera(manager: *const app::FighterManager) -> bool;
    
        #[link_name = "_ZN3app8lua_bind48FighterManager__is_discretion_final_enabled_implEPNS_14FighterManagerE"]
        pub(super) fn is_discretion_final_enabled(manager: *const app::FighterManager) -> bool;
    
        #[link_name = "_ZN3app8lua_bind33FighterManager__is_end_movie_implEPNS_14FighterManagerE"]
        pub(super) fn is_end_movie(manager: *const app::FighterManager) -> bool;
    
        #[link_name = "_ZN3app8lua_bind29FighterManager__is_final_implEPNS_14FighterManagerE"]
        pub(super) fn is_final(manager: *const app::FighterManager) -> bool;
    
        #[link_name = "_ZN3app8lua_bind38FighterManager__is_homerun_versus_implEPNS_14FighterManagerE"]
        pub(super) fn is_homerun_versus(manager: *const app::FighterManager) -> bool;
    
        #[link_name = "_ZN3app8lua_bind42FighterManager__is_melee_mode_homerun_implEPNS_14FighterManagerE"]
        pub(super) fn is_melee_mode_homerun(manager: *const app::FighterManager) -> bool;
    
        #[link_name = "_ZN3app8lua_bind52FighterManager__is_melee_mode_online_tournament_implEPNS_14FighterManagerE"]
        pub(super) fn is_melee_mode_online_tournament(manager: *const app::FighterManager) -> bool;
    
        #[link_name = "_ZN3app8lua_bind38FighterManager__is_prepared_movie_implEPNS_14FighterManagerE"]
        pub(super) fn is_prepared_movie(manager: *const app::FighterManager) -> bool;
    
        #[link_name = "_ZN3app8lua_bind37FighterManager__is_process_movie_implEPNS_14FighterManagerE"]
        pub(super) fn is_process_movie(manager: *const app::FighterManager) -> bool;
    
        #[link_name = "_ZN3app8lua_bind41FighterManager__is_process_technique_implEPNS_14FighterManagerE"]
        pub(super) fn is_process_technique(manager: *const app::FighterManager) -> bool;
    
        #[link_name = "_ZN3app8lua_bind32FighterManager__is_ready_go_implEPNS_14FighterManagerE"]
        pub(super) fn is_ready_go(manager: *const app::FighterManager) -> bool;
    
        #[link_name = "_ZN3app8lua_bind42FighterManager__is_rebirth_plate_line_implEPNS_14FighterManagerENS_14FighterEntryIDE"]
        pub(super) fn is_rebirth_plate_line(manager: *const app::FighterManager, arg2: app::FighterEntryID) -> bool;
    
        #[link_name = "_ZN3app8lua_bind35FighterManager__is_result_mode_implEPNS_14FighterManagerE"]
        pub(super) fn is_result_mode(manager: *const app::FighterManager) -> bool;
    
        #[link_name = "_ZN3app8lua_bind51FighterManager__notify_log_event_collision_hit_implEPNS_14FighterManagerEjjfib"]
        pub(super) fn notify_log_event_collision_hit(manager: *mut app::FighterManager, object_id: u32, object_id2: u32, arg4: f32, arg5: i32, arg6: bool);
    
        #[link_name = "_ZN3app8lua_bind37FighterManager__one_on_one_ratio_implEPNS_14FighterManagerE"]
        pub(super) fn one_on_one_ratio(manager: *const app::FighterManager) -> f32;
    
        #[link_name = "_ZN3app8lua_bind34FighterManager__pause_finalbg_implEPNS_14FighterManagerEj"]
        pub(super) fn pause_finalbg(manager: *mut app::FighterManager, arg2: u32);
    
        #[link_name = "_ZN3app8lua_bind34FighterManager__prepare_movie_implEPNS_14FighterManagerEN3phx6Hash40E"]
        pub(super) fn prepare_movie(manager: *mut app::FighterManager, path: phx::Hash40);
    
        #[link_name = "_ZN3app8lua_bind34FighterManager__reset_fighter_implEPNS_14FighterManagerEb"]
        pub(super) fn reset_fighter(manager: *mut app::FighterManager, arg2: bool);
    
        #[link_name = "_ZN3app8lua_bind35FighterManager__resume_finalbg_implEPNS_14FighterManagerEj"]
        pub(super) fn resume_finalbg(manager: *mut app::FighterManager, arg2: u32);
    
        #[link_name = "_ZN3app8lua_bind46FighterManager__set_controller_rumble_all_implEPNS_14FighterManagerEN3phx6Hash40Eibj"]
        pub(super) fn set_controller_rumble_all(manager: *mut app::FighterManager, rumble_kind: phx::Hash40, arg3: i32, flag: bool, object_id: u32);
    
        #[link_name = "_ZN3app8lua_bind37FighterManager__set_cursor_whole_implEPNS_14FighterManagerEb"]
        pub(super) fn set_cursor_whole(manager: *mut app::FighterManager, whole: bool);
    
        #[link_name = "_ZN3app8lua_bind57FighterManager__set_dead_up_camera_hit_cursor_status_implEPNS_14FighterManagerEb"]
        pub(super) fn set_dead_up_camera_hit_cursor_status(manager: *mut app::FighterManager, arg2: bool);
    
        #[link_name = "_ZN3app8lua_bind40FighterManager__set_final_fear_face_implEPNS_14FighterManagerENS_14FighterEntryIDEi"]
        pub(super) fn set_final_fear_face(manager: *mut app::FighterManager, entry_id: app::FighterEntryID, fear_face: app::FighterFacial);
    
        #[link_name = "_ZN3app8lua_bind30FighterManager__set_final_implEPNS_14FighterManagerENS_14FighterEntryIDENS_21FighterAvailableFinalEj"]
        pub(super) fn set_final(manager: *mut app::FighterManager, entry_id: app::FighterEntryID, available: app::FighterAvailableFinal, arg4: u32);
    
        #[link_name = "_ZN3app8lua_bind38FighterManager__set_position_lock_implEPNS_14FighterManagerENS_14FighterEntryIDEb"]
        pub(super) fn set_position_lock(manager: *mut app::FighterManager, entry_id: app::FighterEntryID, lock: bool);
    
        #[link_name = "_ZN3app8lua_bind40FighterManager__set_visible_finalbg_implEPNS_14FighterManagerEb"]
        pub(super) fn set_visible_finalbg(manager: *mut app::FighterManager, visible: bool);
    
        #[link_name = "_ZN3app8lua_bind33FighterManager__show_finalbg_implEPNS_14FighterManagerEj"]
        pub(super) fn show_finalbg(manager: *mut app::FighterManager, arg2: u32);
    
        #[link_name = "_ZN3app8lua_bind34FighterManager__start_finalbg_implEPNS_14FighterManagerEif"]
        pub(super) fn start_finalbg(manager: *mut app::FighterManager, arg2: i32, arg3: f32);
    
        #[link_name = "_ZN3app8lua_bind32FighterManager__start_movie_implEPNS_14FighterManagerEf"]
        pub(super) fn start_movie(manager: *mut app::FighterManager, arg2: f32);
    
        #[link_name = "_ZN3app8lua_bind48FighterManager__start_movie_on_rendering_2d_implEPNS_14FighterManagerEf"]
        pub(super) fn start_movie_on_rendering_2d(manager: *mut app::FighterManager, arg2: f32);
    
        #[link_name = "_ZN3app8lua_bind38FighterManager__total_fighter_num_implEPNS_14FighterManagerE"]
        pub(super) fn total_fighter_num(manager: *const app::FighterManager) -> u32;
    
    }
}

#[repr(C)]
pub struct FighterManager {
    event_manager: *mut FighterEventManager
}

impl FighterManager {
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

    pub fn disable_ko_camera(&mut self) {
        unsafe {
            impl_::disable_ko_camera(self)
        }
    }

    pub fn enable_ko_camera(&mut self) {
        unsafe {
            impl_::enable_ko_camera(self)
        }
    }

    pub fn entry_count(&self) -> u32 {
        unsafe {
            impl_::entry_count(self)
        }
    }

    pub fn exit_finalbg(&mut self) {
        unsafe {
            impl_::exit_finalbg(self)
        }
    }

    pub fn exit_movie(&mut self) {
        unsafe {
            impl_::exit_movie(self)
        }
    }

    pub fn get_beat_point_diff_from_top(&self, entry_id: app::FighterEntryID) -> i32 {
        unsafe {
            impl_::get_beat_point_diff_from_top(self, entry_id)
        }
    }

    pub fn get_entry_id(&self, entry_no: i32) -> app::FighterEntryID {
        unsafe {
            impl_::get_entry_id(self, entry_no)
        }
    }

    pub fn get_entry_no(&self, entry_id: app::FighterEntryID) -> i32 {
        unsafe {
            impl_::get_entry_no(self, entry_id)
        }
    }

    pub fn get_fighter_entry(&self, entry_id: app::FighterEntryID) -> Option<&app::FighterEntry> {
        unsafe {
            impl_::get_fighter_entry(self, entry_id).as_ref()
        }
    }

    pub fn get_fighter_information(&self, entry_id: app::FighterEntryID) -> Option<&app::FighterInformation> {
        unsafe {
            impl_::get_fighter_information(self, entry_id).as_ref()
        }
    }

    pub fn get_fighter_entry_mut(&mut self, entry_id: app::FighterEntryID) -> Option<&mut app::FighterEntry> {
        unsafe {
            impl_::get_fighter_entry(self, entry_id).as_mut()
        }
    }

    pub fn get_fighter_information_mut(&mut self, entry_id: app::FighterEntryID) -> Option<&mut app::FighterInformation> {
        unsafe {
            impl_::get_fighter_information(self, entry_id).as_mut()
        }
    }

    pub fn get_fighter_pos(&self, entry_id: app::FighterEntryID, entry_no: i32) -> phx::Vec3 {
        unsafe {
            impl_::get_fighter_pos(self, entry_id, entry_no).into()
        }
    }

    pub fn get_final_actor_entry_id(&self) -> app::FighterEntryID {
        unsafe {
            impl_::get_final_actor_entry_id(self)
        }
    }

    pub fn get_jack_final_cut_in(&self) -> i32 {
        unsafe {
            impl_::get_jack_final_cut_in(self)
        }
    }

    pub fn get_no_discretion_final_beat_count(&self) -> u32 {
        unsafe {
            impl_::get_no_discretion_final_beat_count(self)
        }
    }

    pub fn get_top_rank_player(&self, arg2: i32) -> u32 {
        unsafe {
            impl_::get_top_rank_player(self, arg2)
        }
    }

    pub fn get_top_rank_player_num(&self) -> u8 {
        unsafe {
            impl_::get_top_rank_player_num(self)
        }
    }

    pub fn hide_finalbg(&mut self, arg2: u32) {
        unsafe {
            impl_::hide_finalbg(self, arg2)
        }
    }

    pub fn is_available_discretion_final(&self) -> bool {
        unsafe {
            impl_::is_available_discretion_final(self)
        }
    }

    pub fn is_disable_ko_camera(&self) -> bool {
        unsafe {
            impl_::is_disable_ko_camera(self)
        }
    }

    pub fn is_discretion_final_enabled(&self) -> bool {
        unsafe {
            impl_::is_discretion_final_enabled(self)
        }
    }

    pub fn is_end_movie(&self) -> bool {
        unsafe {
            impl_::is_end_movie(self)
        }
    }

    pub fn is_final(&self) -> bool {
        unsafe {
            impl_::is_final(self)
        }
    }

    pub fn is_homerun_versus(&self) -> bool {
        unsafe {
            impl_::is_homerun_versus(self)
        }
    }

    pub fn is_melee_mode_homerun(&self) -> bool {
        unsafe {
            impl_::is_melee_mode_homerun(self)
        }
    }

    pub fn is_melee_mode_online_tournament(&self) -> bool {
        unsafe {
            impl_::is_melee_mode_online_tournament(self)
        }
    }

    pub fn is_prepared_movie(&self) -> bool {
        unsafe {
            impl_::is_prepared_movie(self)
        }
    }

    pub fn is_process_movie(&self) -> bool {
        unsafe {
            impl_::is_process_movie(self)
        }
    }

    pub fn is_process_technique(&self) -> bool {
        unsafe {
            impl_::is_process_technique(self)
        }
    }

    pub fn is_ready_go(&self) -> bool {
        unsafe {
            impl_::is_ready_go(self)
        }
    }

    pub fn is_rebirth_plate_line(&self, arg2: app::FighterEntryID) -> bool {
        unsafe {
            impl_::is_rebirth_plate_line(self, arg2)
        }
    }

    pub fn is_result_mode(&self) -> bool {
        unsafe {
            impl_::is_result_mode(self)
        }
    }

    pub fn notify_log_event_collision_hit(&mut self, object_id: u32, object_id2: u32, arg4: f32, arg5: i32, arg6: bool) {
        unsafe {
            impl_::notify_log_event_collision_hit(self, object_id, object_id2, arg4, arg5, arg6)
        }
    }

    pub fn one_on_one_ratio(&self) -> f32 {
        unsafe {
            impl_::one_on_one_ratio(self)
        }
    }

    pub fn pause_finalbg(&mut self, arg2: u32) {
        unsafe {
            impl_::pause_finalbg(self, arg2)
        }
    }

    pub fn prepare_movie(&mut self, path: phx::Hash40) {
        unsafe {
            impl_::prepare_movie(self, path)
        }
    }

    pub fn reset_fighter(&mut self, arg2: bool) {
        unsafe {
            impl_::reset_fighter(self, arg2)
        }
    }

    pub fn resume_finalbg(&mut self, arg2: u32) {
        unsafe {
            impl_::resume_finalbg(self, arg2)
        }
    }

    pub fn set_controller_rumble_all(&mut self, rumble_kind: phx::Hash40, arg3: i32, flag: bool, object_id: u32) {
        unsafe {
            impl_::set_controller_rumble_all(self, rumble_kind, arg3, flag, object_id)
        }
    }

    pub fn set_cursor_whole(&mut self, whole: bool) {
        unsafe {
            impl_::set_cursor_whole(self, whole)
        }
    }

    pub fn set_dead_up_camera_hit_cursor_status(&mut self, arg2: bool) {
        unsafe {
            impl_::set_dead_up_camera_hit_cursor_status(self, arg2)
        }
    }

    pub fn set_final_fear_face(&mut self, entry_id: app::FighterEntryID, fear_face: app::FighterFacial) {
        unsafe {
            impl_::set_final_fear_face(self, entry_id, fear_face)
        }
    }

    pub fn set_final(&mut self, entry_id: app::FighterEntryID, available: app::FighterAvailableFinal, arg4: u32) {
        unsafe {
            impl_::set_final(self, entry_id, available, arg4)
        }
    }

    pub fn set_position_lock(&mut self, entry_id: app::FighterEntryID, lock: bool) {
        unsafe {
            impl_::set_position_lock(self, entry_id, lock)
        }
    }

    pub fn set_visible_finalbg(&mut self, visible: bool) {
        unsafe {
            impl_::set_visible_finalbg(self, visible)
        }
    }

    pub fn show_finalbg(&mut self, arg2: u32) {
        unsafe {
            impl_::show_finalbg(self, arg2)
        }
    }

    pub fn start_finalbg(&mut self, arg2: i32, arg3: f32) {
        unsafe {
            impl_::start_finalbg(self, arg2, arg3)
        }
    }

    pub fn start_movie(&mut self, arg2: f32) {
        unsafe {
            impl_::start_movie(self, arg2)
        }
    }

    pub fn start_movie_on_rendering_2d(&mut self, arg2: f32) {
        unsafe {
            impl_::start_movie_on_rendering_2d(self, arg2)
        }
    }

    pub fn total_fighter_num(&self) -> u32 {
        unsafe {
            impl_::total_fighter_num(self)
        }
    }
}

// This section is for the fighter event handler
// None of the names here were found in game but we figured they'd probably match the scheme

/// The basic fighter event struct for any event sent through the event manager.
///
/// This struct allows for downcasting based off of event IDs.
/// 
/// The event's data should be read only, as modifying any of the contents could lead to improper downcasting
#[repr(C)]
#[derive(Debug)]
pub struct FighterEvent {
    vtable: &'static lib::BasicEventVTable,
    event_id: FighterEventID,
    entry_id: app::FighterEntryID
}

impl FighterEvent {
    /// Try to downcast a basic event down to a different event kind, based off of the event id
    pub fn try_downcast<T: FighterEventInheriter>(&self) -> Option<&T> {
        let event_id = T::get_fighter_event_id();
        if self.event_id == event_id {
            Some(unsafe {
                std::mem::transmute(self)
            })
        } else {
            None
        }
    }

    pub fn get_event_id(&self) -> FighterEventID {
        self.event_id
    }

    pub fn get_raw_event_id(&self) -> u32 {
        unsafe {
            *(&self.event_id as *const FighterEventID as *const u32)
        }
    }

    pub fn get_entry_id(&self) -> app::FighterEntryID {
        self.entry_id
    }

    /// Create a fighter event
    pub(crate) fn new(id: FighterEventID, entry_id: app::FighterEntryID) -> Self {
        Self {
            vtable: &lib::BASIC_EVENT_DEFAULT_VTABLE,
            event_id: id,
            entry_id: entry_id
        }
    }
}

/// The trait to implement to allow downcasting to an event, sealed so that only this crate can implement any
#[sealed::sealed(pub(crate))]
pub trait FighterEventInheriter {
    fn get_fighter_event_id() -> FighterEventID; 
}

/// Game events natively support virtualization function calls w/ objects, however I don't want to do that so instead we are allowing the user to provide a user data pointer
/// where they can provide their own object if they wish
pub type OnFighterEventFn = extern "C" fn(event: &FighterEvent, user_data: *mut u8);

#[repr(C)]
struct FighterEventListenerVTable {
    pub destructor: extern "C" fn(&mut FighterEventListener),
    pub deleter: extern "C" fn(*mut FighterEventListener),
    pub is_equal: extern "C" fn(&FighterEventListener, &FighterEventListener) -> bool,
    pub call: extern "C" fn(&FighterEventListener, &FighterEvent)
}

// We actually implement the VTable this time since these methods get called
impl FighterEventListenerVTable {
    pub extern "C" fn destructor(_: &mut FighterEventListener) {}
    pub extern "C" fn deleter(this: *mut FighterEventListener) {
        unsafe {
            skyline::libc::free(this as _)
        }
    }
    pub extern "C" fn is_equal(_: &FighterEventListener, _: &FighterEventListener) -> bool { false }
    pub extern "C" fn call(this: &FighterEventListener, event: &FighterEvent) {
        (this.function)(event, this.user_data)
    }
}

static FIGHTER_EVENT_LISTENER_VTABLE: FighterEventListenerVTable = FighterEventListenerVTable {
    destructor: FighterEventListenerVTable::destructor,
    deleter: FighterEventListenerVTable::deleter,
    is_equal: FighterEventListenerVTable::is_equal,
    call: FighterEventListenerVTable::call,
};

#[repr(C)]
struct FighterEventListener {
    vtable: &'static FighterEventListenerVTable,
    unk: u64,
    user_data: *mut u8, // for game originated events, this is an object 9/10 times
    function: OnFighterEventFn,
    offset: u64
}

impl FighterEventListener {
    /// Creates a new FighterEventListener without user data
    pub fn new(handler: OnFighterEventFn) -> Self {
        Self {
            vtable: &FIGHTER_EVENT_LISTENER_VTABLE,
            unk: 0xFFFFFFFF_00000001, // taken from one found in game, not sure what this is
            user_data: std::ptr::null_mut(),
            function: handler,
            offset: 0x0
        }
    }

    /// Creates a new FighterEventListener with user data
    pub fn new_with_user_data(handler: OnFighterEventFn, data: *mut u8) -> Self {
        let mut result = Self::new(handler);
        result.user_data = data;
        result
    }

    /// Calls the event listener from the VTable so that both game-originated and mod-originated are supported
    pub fn call(&self, event: &FighterEvent) {
        (self.vtable.call)(self, event)
    }
}

/// The event manager for all fighters.
/// 
/// Anything can add event listeners here, and many different UI elements are based on this event manager
#[repr(C)]
struct FighterEventManager {
    pub event_count: u32,
    padding: u32,
    pub event_listener_lists: *mut cpp::LinkedList<*mut FighterEventListener>
    // pub list_begin: *mut EventList,
    // ...
}

impl FighterEventManager {
    pub fn get_event_listener_lists(&self) -> &[cpp::LinkedList<*mut FighterEventListener>] {
        unsafe {
            std::slice::from_raw_parts(self.event_listener_lists, self.event_count as usize)
        }
    }

    pub fn get_event_listener_lists_mut(&mut self) -> &mut [cpp::LinkedList<*mut FighterEventListener>] {
        unsafe {
            std::slice::from_raw_parts_mut(self.event_listener_lists, self.event_count as usize)
        }
    }
}

impl FighterManager {
    /// Sends a FighterEvent
    /// # Arguments
    /// * `event` - The event to send
    pub fn send_event<A: AsRef<FighterEvent>>(&self, event: A) {
        let event = event.as_ref();
        let event_id = event.event_id as usize;
        let lists = unsafe {
            (*self.event_manager).get_event_listener_lists()
        };
        if let Some(list) = lists.get(event_id) {
            for listener in list.iter() {
                unsafe {
                    (**listener).call(event)
                }
            }
        }
    }

    /// Adds an event listener
    /// # Arguments
    /// * `event_id` - The ID of the event to listen on
    /// * `handler` - The handler of the event
    /// # Returns
    /// Whether or not it was able to add the event listener
    pub fn add_event_listener(&self, event_id: FighterEventID, handler: OnFighterEventFn) -> bool {
        let lists = unsafe {
            (*self.event_manager).get_event_listener_lists_mut()
        };

        if let Some(list) = lists.get_mut(event_id as usize) {
            list.add_to_back(Box::leak(Box::new(FighterEventListener::new(handler))));
            true
        } else {
            false
        }
    }

    /// Adds an event listener
    /// # Arguments
    /// * `event_id` - The ID of the event to listen on
    /// * `handler` - The handler of the event
    /// * `data` - The user data to attach to the event listener
    /// # Returns
    /// Whether or not it was able to add the event listener
    pub fn add_event_listener_with_user_data(&self, event_id: FighterEventID, handler: OnFighterEventFn, data: *mut u8) -> bool {
        let lists = unsafe {
            (*self.event_manager).get_event_listener_lists_mut()
        };

        if let Some(list) = lists.get_mut(event_id as usize) {
            list.add_to_back(Box::leak(Box::new(FighterEventListener::new_with_user_data(handler, data))));
            true
        } else {
            false
        }
    }

    /// Adds an event listener
    /// # Arguments
    /// * `event_id` - The ID of the event to listen on, without a strong type requirement
    /// * `handler` - The handler of the event
    /// # Returns
    /// Whether or not it was able to add the event listener
    pub fn add_event_listener_raw_id(&self, event_id: u32, handler: OnFighterEventFn) -> bool {
        let lists = unsafe {
            (*self.event_manager).get_event_listener_lists_mut()
        };

        if let Some(list) = lists.get_mut(event_id as usize) {
            list.add_to_back(Box::leak(Box::new(FighterEventListener::new(handler))));
            true
        } else {
            false
        }
    }

    /// Adds an event listener
    /// # Arguments
    /// * `event_id` - The ID of the event to listen on, without a strong type requirement
    /// * `handler` - The handler of the event
    /// * `data` - The user data to attach to the event listener
    /// # Returns
    /// Whether or not it was able to add the event listener
    pub fn add_event_listener_with_user_data_raw_id(&self, event_id: u32, handler: OnFighterEventFn, data: *mut u8) -> bool {
        let lists = unsafe {
            (*self.event_manager).get_event_listener_lists_mut()
        };

        if let Some(list) = lists.get_mut(event_id as usize) {
            list.add_to_back(Box::leak(Box::new(FighterEventListener::new_with_user_data(handler, data))));
            true
        } else {
            false
        }
    }

    /// Adds an event listener to all possible events
    /// # Arguments
    /// * `handler` - The handler of the event
    /// # Returns
    /// Whether or not it was able to add the event listener
    pub fn add_global_event_listener(&self, handler: OnFighterEventFn) {
        let lists = unsafe {
            (*self.event_manager).get_event_listener_lists_mut()
        };

        for list in lists.iter_mut() {
            list.add_to_back(Box::leak(Box::new(FighterEventListener::new_with_user_data(handler, std::ptr::null_mut()))));
        }
    }

    /// Adds an event listener to all possible events
    /// # Arguments
    /// * `handler` - The handler of the event
    /// * `data` - The user data to attach to the event listener
    /// # Returns
    /// Whether or not it was able to add the event listener
    pub fn add_global_event_listener_with_user_data(&self, handler: OnFighterEventFn, user_data: *mut u8) {
        let lists = unsafe {
            (*self.event_manager).get_event_listener_lists_mut()
        };

        for list in lists.iter_mut() {
            list.add_to_back(Box::leak(Box::new(FighterEventListener::new_with_user_data(handler, user_data))));
        }
    }
}