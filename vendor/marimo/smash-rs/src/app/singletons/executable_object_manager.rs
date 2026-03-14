use crate::*;

#[repr(C)]
pub struct ExecutableObject {
    name: *const u8,
    category: i32,
    padding: u32,
    run_on_frame_advance: bool,
    padding2: [u8; 0x2F],
    function: *mut ExecutableFunction,
}

#[repr(C)]
pub struct ExecutableFunctionVTable {
    destructor: extern "C" fn(&mut ExecutableFunction),
    deleter: extern "C" fn(&mut ExecutableFunction),
    copy_into: extern "C" fn(&ExecutableFunction, &mut ExecutableFunction),
    copy_from: extern "C" fn(&mut ExecutableFunction, &ExecutableFunction),
    destructor2: extern "C" fn(&mut ExecutableFunction),
    deleter2: extern "C" fn(&mut ExecutableFunction),
    call: extern "C" fn(&ExecutableFunction),
}

#[repr(C)]
pub struct ExecutableFunction {
    vtable: &'static ExecutableFunctionVTable,
    unk: u64,
    abs_function_or_vtable_offset: u64,
    shift_and_is_virtual: u64,
    object: u64,
}

#[repr(C)]
pub struct ExecutableObjectManager {
    pub current_category: i32,
    pub should_category_run: [i32; 10],
    pub unk_category: [i32; 10],
    pub objects_by_category: [cpp::Deque<cpp::SharedPtr<ExecutableObject>>; 10],
}
