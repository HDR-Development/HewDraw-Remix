use crate::*;


#[repr(C)]
pub(crate) struct BasicEventVTable {
    pub destructor: extern "C" fn(*mut BasicEvent),
    pub deleter: extern "C" fn(*mut BasicEvent),
    pub get_id: extern "C" fn(*const BasicEvent) -> u32,
    pub into_l2ctable: extern "C" fn(*const BasicEvent, *mut lib::L2CTable),
    pub from_l2ctable: extern "C" fn(*mut BasicEvent, *const lib::L2CTable),
    pub into_l2cvalue: extern "C" fn(*const BasicEvent) -> lib::L2CValueHack,
    pub make_l2cvalue: extern "C" fn(*const BasicEvent, lib::L2CValueHack) -> lib::L2CValueHack,
    pub load_l2cvalue: extern "C" fn(*const BasicEvent, *mut lib::L2CValue)
}

impl std::fmt::Debug for BasicEventVTable {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(()) // just don't format lol
    }
}

pub(crate) static BASIC_EVENT_DEFAULT_VTABLE: BasicEventVTable = BasicEventVTable {
    destructor: BasicEventVTable::destructor,
    deleter: BasicEventVTable::deleter,
    get_id: BasicEventVTable::get_id,
    into_l2ctable: BasicEventVTable::into_l2ctable,
    from_l2ctable: BasicEventVTable::from_l2ctable,
    into_l2cvalue: BasicEventVTable::into_l2cvalue,
    make_l2cvalue: BasicEventVTable::make_l2cvalue,
    load_l2cvalue: BasicEventVTable::load_l2cvalue,
};

impl BasicEventVTable {
    pub extern "C" fn destructor(_: *mut BasicEvent) {
        unimplemented!()
    }

    pub extern "C" fn deleter(_: *mut BasicEvent) {
        unimplemented!()
    }

    pub extern "C" fn get_id(_: *const BasicEvent) -> u32 {
        unimplemented!()
    }

    pub extern "C" fn into_l2ctable(_: *const BasicEvent, _: *mut lib::L2CTable) {
        unimplemented!()
    }

    pub extern "C" fn from_l2ctable(_: *mut BasicEvent, _: *const lib::L2CTable) {
        unimplemented!()
    }

    pub extern "C" fn into_l2cvalue(_: *const BasicEvent) -> lib::L2CValueHack {
        unimplemented!()
    }

    pub extern "C" fn make_l2cvalue(_: *const BasicEvent, _: lib::L2CValueHack) -> lib::L2CValueHack {
        unimplemented!()
    }

    pub extern "C" fn load_l2cvalue(_: *const BasicEvent, _: *mut lib::L2CValue) {
        unimplemented!()
    }
}

#[repr(C)]
pub struct BasicEvent {
    vtable: &'static BasicEventVTable,
    pub id: u32,
}

/// Game events natively support virtualization function calls w/ objects, however I don't want to do that so instead we are allowing the user to provide a user data pointer
/// where they can provide their own object if they wish
pub type OnEventFn = extern "C" fn(event: &BasicEvent, user_data: *mut u8);

#[repr(C)]
pub(crate) struct EventListenerVTable {
    pub destructor: extern "C" fn(&mut EventListener),
    pub deleter: extern "C" fn(*mut EventListener),
    pub is_equal: extern "C" fn(&EventListener, &EventListener) -> bool,
    pub call: extern "C" fn(&EventListener, &BasicEvent)
}

// We actually implement the VTable this time since these methods get called
impl EventListenerVTable {
    pub extern "C" fn destructor(_: &mut EventListener) {}
    pub extern "C" fn deleter(this: *mut EventListener) {
        unsafe {
            skyline::libc::free(this as _)
        }
    }
    pub extern "C" fn is_equal(_: &EventListener, _: &EventListener) -> bool { false }
    pub extern "C" fn call(this: &EventListener, event: &BasicEvent) {
        (this.function)(event, this.user_data)
    }
}

static EVENT_LISTENER_VTABLE: EventListenerVTable = EventListenerVTable {
    destructor: EventListenerVTable::destructor,
    deleter: EventListenerVTable::deleter,
    is_equal: EventListenerVTable::is_equal,
    call: EventListenerVTable::call
};

#[repr(C)]
pub struct EventListener {
    vtable: &'static EventListenerVTable,
    unk: u64,
    user_data: *mut u8, // for game originated events, this is an object 9/10 times
    function: OnEventFn,
    offset: u64
}

impl EventListener {
    pub fn new<T: Sized>(function: OnEventFn, data: &mut T) -> *mut Self {
        unsafe {
            let ptr = skyline::libc::malloc(std::mem::size_of::<Self>() as _);
            std::ptr::write(ptr as _, Self {
                vtable: &EVENT_LISTENER_VTABLE,
                unk: 0xFFFFFFFF_00000001,
                user_data: data as *mut T as *mut u8,
                function,
                offset: 0
            });
            ptr as _
        }
        
    }
}

/// A structure which helps manage events and event listeners
/// 
/// Anything can add event listeners here, and many different UI elements are based on this event manager
#[repr(C)]
pub struct EventManager {
    pub event_count: u32,
    padding: u32,
    pub event_listener_lists: [*mut cpp::LinkedList<*mut EventListener>; 2],
    pub active_users: usize,
    some_flag: bool
    // pub list_begin: *mut EventList,
    // ...
}

impl EventManager {
    pub fn get_event_listener_lists(&self, index: usize) -> &[cpp::LinkedList<*mut EventListener>] {
        unsafe {
            std::slice::from_raw_parts(self.event_listener_lists[index], self.event_count as usize)
        }
    }

    pub fn get_event_listener_lists_mut(&mut self, index: usize) -> &mut [cpp::LinkedList<*mut EventListener>] {
        unsafe {
            std::slice::from_raw_parts_mut(self.event_listener_lists[index], self.event_count as usize)
        }
    }
}
