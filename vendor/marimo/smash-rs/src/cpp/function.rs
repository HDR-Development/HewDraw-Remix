use skyline::libc::c_void;

#[repr(C)]
pub struct FunctionBase {
    vtable: *const *const c_void
}