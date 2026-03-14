use crate::*;

#[repr(C)]
pub struct L2CInnerFunctionBase {
    function: *mut cpp::FunctionBase,
    ref_count: u32,
}