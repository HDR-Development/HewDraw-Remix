use std::mem::MaybeUninit;

use skyline::libc;

use super::l2c_value::*;
use crate::phx::*;

extern "C" {
    #[link_name = "\u{1}_ZN3lib8L2CValueC1Ev"]
    fn L2CValue_L2CValue(arg: *mut L2CValue);
    #[link_name = "\u{1}_ZN3lib8L2CValueC1ERKS0_"]
    fn L2CValue_L2CValue2(arg: *mut L2CValue, src: *const L2CValue);
    #[link_name = "\u{1}_ZN3lib8L2CValueC1Eb"]
    fn L2CValue_L2CValue3(arg: *mut L2CValue, val: bool);
    #[link_name = "\u{1}_ZN3lib8L2CValueC1Ei"]
    fn L2CValue_L2CValue4(arg: *mut L2CValue, val: i32);
    #[link_name = "\u{1}_ZN3lib8L2CValueC1Ej"]
    fn L2CValue_L2CValue5(arg: *mut L2CValue, val: u32);
    #[link_name = "\u{1}_ZN3lib8L2CValueC1El"]
    fn L2CValue_L2CValue6(arg: *mut L2CValue, val: i64);
    #[link_name = "\u{1}_ZN3lib8L2CValueC1Em"]
    fn L2CValue_L2CValue7(arg: *mut L2CValue, val: u64);
    #[link_name = "\u{1}_ZN3lib8L2CValueC1Ef"]
    fn L2CValue_L2CValue8(arg: *mut L2CValue, val: f32);
    #[link_name = "\u{1}_ZN3lib8L2CValueC1EPv"]
    fn L2CValue_L2CValue9(arg: *mut L2CValue, val: *mut libc::c_void);
    #[link_name = "\u{1}_ZN3lib8L2CValueC1EPNS_8L2CTableE"]
    fn L2CValue_L2CValue10(arg: *mut L2CValue, val: *mut L2CTable);
    #[link_name = "\u{1}_ZN3lib8L2CValueC1EPNS_20L2CInnerFunctionBaseE"]
    fn L2CValue_L2CValue11(arg: *mut L2CValue, val: *mut L2CInnerFunctionBase);
    #[link_name = "\u{1}_ZN3lib8L2CValueC1EN3phx6Hash40E"]
    fn L2CValue_L2CValue12(arg: *mut L2CValue, val: Hash40);
    #[link_name = "\u{1}_ZN3lib8L2CValueC1EPKc"]
    fn L2CValue_L2CValue13(arg: *mut L2CValue, val: *const libc::c_char);
    #[link_name = "\u{1}_ZNK3lib8L2CValue7as_boolEv"]
    fn L2CValue_as_bool(arg: *const L2CValue) -> bool;
    #[link_name = "\u{1}_ZNK3lib8L2CValue7as_hashEv"]
    fn L2CValue_as_hash(arg: *const L2CValue) -> Hash40;
    #[link_name = "\u{1}_ZNK3lib8L2CValue17as_inner_functionEv"]
    fn L2CValue_as_inner_function(arg: *const L2CValue) -> *mut L2CInnerFunctionBase;
    #[link_name = "\u{1}_ZNK3lib8L2CValue10as_integerEv"]
    fn L2CValue_as_integer(arg: *const L2CValue) -> u64;
    #[link_name = "\u{1}_ZNK3lib8L2CValue9as_numberEv"]
    fn L2CValue_as_number(arg: *const L2CValue) -> f32;
    #[link_name = "\u{1}_ZNK3lib8L2CValue10as_pointerEv"]
    fn L2CValue_as_pointer(arg: *const L2CValue) -> *mut libc::c_void;
    #[link_name = "\u{1}_ZNK3lib8L2CValue9as_stringEv"]
    fn L2CValue_as_string(arg: *const L2CValue) -> *const libc::c_char;
    #[link_name = "\u{1}_ZNK3lib8L2CValue8as_tableEv"]
    fn L2CValue_as_table(arg: *const L2CValue) -> *mut L2CTable;
    #[link_name = "\u{1}_ZN3lib8L2CValueD1Ev"]
    fn L2CValue_dtor(arg: *mut L2CValue);
    #[link_name = "\u{1}_ZN3lib8L2CTableC1Ei"]
    fn L2CTable_L2CTable(arg: *mut L2CTable, count: i32);
    #[link_name = "\u{1}_Znwm"]
    fn cpp_new(size: u64) -> *mut libc::c_void;
}

impl L2CValue {

    #[allow(non_snake_case)]
    #[inline]
    pub fn Void() -> Self {
        unsafe {
            let mut ret = MaybeUninit::uninit();
            L2CValue_L2CValue(ret.as_mut_ptr());
            ret.assume_init()
        }
    }

    #[allow(non_snake_case)]
    #[inline]
    pub fn Bool(val: bool) -> Self {
        unsafe {
            let mut ret = MaybeUninit::uninit();
            L2CValue_L2CValue3(ret.as_mut_ptr(), val);
            ret.assume_init()
        }
    }

    #[allow(non_snake_case)]
    #[inline]
    pub fn I32(val: i32) -> Self {
        unsafe {
            let mut ret = MaybeUninit::uninit();
            L2CValue_L2CValue4(ret.as_mut_ptr(), val);
            ret.assume_init()
        }
    }

    #[allow(non_snake_case)]
    #[inline]
    pub fn U32(val: u32) -> Self {
        unsafe {
            let mut ret = MaybeUninit::uninit();
            L2CValue_L2CValue5(ret.as_mut_ptr(), val);
            ret.assume_init()
        }
    }

    #[allow(non_snake_case)]
    #[inline]
    pub fn I64(val: i64) -> Self {
        unsafe {
            let mut ret = MaybeUninit::uninit();
            L2CValue_L2CValue6(ret.as_mut_ptr(), val);
            ret.assume_init()
        }
    }

    #[allow(non_snake_case)]
    #[inline]
    pub fn U64(val: u64) -> Self {
        unsafe {
            let mut ret = MaybeUninit::uninit();
            L2CValue_L2CValue7(ret.as_mut_ptr(), val);
            ret.assume_init()
        }
    }

    #[allow(non_snake_case)]
    #[inline]
    pub fn F32(val: f32) -> Self {
        unsafe {
            let mut ret = MaybeUninit::uninit();
            L2CValue_L2CValue8(ret.as_mut_ptr(), val);
            ret.assume_init()
        }
    }

    #[allow(non_snake_case)]
    #[inline]
    pub fn Ptr(val: *mut libc::c_void) -> Self {
        unsafe {
            let mut ret = MaybeUninit::uninit();
            L2CValue_L2CValue9(ret.as_mut_ptr(), val);
            ret.assume_init()
        }
    }

    #[allow(non_snake_case)]
    #[inline]
    pub fn Table(val: *mut L2CTable) -> Self{
        unsafe {
            let mut ret = MaybeUninit::uninit();
            L2CValue_L2CValue10(ret.as_mut_ptr(), val);
            ret.assume_init()
        }
    }

    #[allow(non_snake_case)]
    #[inline]
    pub fn InnerFunc(val: *mut L2CInnerFunctionBase) -> Self {
        unsafe {
            let mut ret = MaybeUninit::uninit();
            L2CValue_L2CValue11(ret.as_mut_ptr(), val);
            ret.assume_init()
        }
    }

    #[allow(non_snake_case)]
    #[inline]
    pub fn Hash40(val: Hash40) -> Self {
        unsafe {
            let mut ret = MaybeUninit::uninit();
            L2CValue_L2CValue12(ret.as_mut_ptr(), val);
            ret.assume_init()
        }
    }

    #[allow(non_snake_case)]
    #[inline]
    pub fn Hash40s<S: AsRef<str>>(val: S) -> Self {
        Self::Hash40(Hash40::new(val.as_ref()))
    }

    #[allow(non_snake_case)]
    #[inline]
    pub fn String<S: AsRef<str>>(val: S) -> Self {
        unsafe {
            let mut ret = MaybeUninit::uninit();
            let str_ref = val.as_ref();
            let c_str = [str_ref.as_bytes(), "\u{0}".as_bytes()].concat().as_ptr() as *const libc::c_char;
            L2CValue_L2CValue13(ret.as_mut_ptr(), c_str);
            ret.assume_init()
        }
    }

    #[inline]
    pub fn get_bool(&self) -> bool {
        unsafe {
            L2CValue_as_bool(self)
        }
    }

    #[inline]
    pub fn get_i32(&self) -> i32 {
        unsafe {
            L2CValue_as_integer(self) as i32
        }
    }

    #[inline]
    pub fn get_u32(&self) -> u32 {
        unsafe {
            L2CValue_as_integer(self) as u32
        }
    }

    #[inline]
    pub fn get_i64(&self) -> i64 {
        unsafe {
            L2CValue_as_integer(self) as i64
        }
    }

    #[inline]
    pub fn get_u64(&self) -> u64 {
        unsafe {
            L2CValue_as_integer(self)
        }
    }

    #[inline]
    pub fn get_f32(&self) -> f32 {
        unsafe {
            L2CValue_as_number(self)
        }
    }

    #[inline]
    pub fn get_f64(&self) -> f64 {
        unsafe {
            L2CValue_as_number(self) as f64
        }
    }

    #[inline]
    pub fn get_ptr(&self) -> *const libc::c_void {
        unsafe {
            L2CValue_as_pointer(self) as *const libc::c_void
        }
    }

    #[inline]
    pub fn get_ptr_mut(&self) -> *mut libc::c_void {
        unsafe {
            L2CValue_as_pointer(self)
        }
    }

    #[inline]
    pub fn get_table(&self) -> *mut L2CTable {
        unsafe {
            L2CValue_as_table(self)
        }
    }

    #[inline]
    pub fn get_inner_func(&self) -> *mut L2CInnerFunctionBase {
        unsafe {
            L2CValue_as_inner_function(self)
        }
    }

    #[inline]
    pub fn get_hash(&self) -> Hash40 {
        unsafe {
            L2CValue_as_hash(self)
        }
    }

    #[inline]
    pub fn get_string(&self) -> String {
        unsafe {
            skyline::from_c_str(L2CValue_as_string(self))
        }
    }

    #[inline]
    pub fn assign(&mut self, other: &Self) {
        unsafe {
            L2CValue_dtor(self);
            L2CValue_L2CValue2(self, other);
        }
    }
}

impl Drop for L2CValue {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            L2CValue_dtor(self)
        }
    }
}

impl Into<bool> for &L2CValue {
    fn into(self) -> bool {
        self.get_bool()
    }
}

impl Into<Hash40> for &L2CValue {
    fn into(self) -> Hash40 {
        self.get_hash()
    }
}

impl Into<*mut L2CInnerFunctionBase> for &L2CValue {
    fn into(self) -> *mut L2CInnerFunctionBase {
        self.get_inner_func()
    }
}

impl Into<*mut L2CTable> for &L2CValue {
    fn into(self) -> *mut L2CTable {
        self.get_table()
    }
}

impl Into<String> for &L2CValue {
    fn into(self) -> String {
        self.get_string()
    }
}

impl Into<L2CValue> for () {
    fn into(self) -> L2CValue {
        L2CValue::Void()
    }
}

impl Into<L2CValue> for bool {
    fn into(self) -> L2CValue {
        L2CValue::Bool(self)
    }
}

impl Into<L2CValue> for i32 {
    fn into(self) -> L2CValue {
        L2CValue::I32(self)
    }
}

impl Into<L2CValue> for u32 {
    fn into(self) -> L2CValue {
        L2CValue::U32(self)
    }
}

impl Into<L2CValue> for i64 {
    fn into(self) -> L2CValue {
        L2CValue::I64(self)
    }
}

impl Into<L2CValue> for u64 {
    fn into(self) -> L2CValue {
        L2CValue::U64(self)
    }
}

impl Into<L2CValue> for f32 {
    fn into(self) -> L2CValue {
        L2CValue::F32(self)
    }
}

impl Into<L2CValue> for f64 {
    fn into(self) -> L2CValue {
        L2CValue::F32(self as f32)
    }
}

impl Into<L2CValue> for *mut libc::c_void {
    fn into(self) -> L2CValue {
        L2CValue::Ptr(self)
    }
}

impl Into<L2CValue> for *mut L2CTable {
    fn into(self) -> L2CValue {
        L2CValue::Table(self)
    }
}

impl Into<L2CValue> for *mut L2CInnerFunctionBase {
    fn into(self) -> L2CValue {
        L2CValue::InnerFunc(self)
    }
}

impl Into<L2CValue> for Hash40 {
    fn into(self) -> L2CValue {
        L2CValue::Hash40(self)
    }
}

impl Into<L2CValue> for String {
    fn into(self) -> L2CValue {
        L2CValue::String(self)
    }
}

impl Into<L2CValue> for &str {
    fn into(self) -> L2CValue {
        L2CValue::String(self)
    }
}

impl Clone for L2CValue {
    fn clone(&self) -> Self {
        unsafe {
            let mut ret = MaybeUninit::uninit();
            L2CValue_L2CValue2(ret.as_mut_ptr(), self);
            ret.assume_init()
        }
    }
}

macro_rules! impl_into_int_l2cvalue {
    (
        $(
            $ty: ty
        )*
    ) => {
        $(
            impl Into<$ty> for L2CValue {
                fn into(self) -> $ty {
                    unsafe {
                        L2CValue_as_integer(&self as *const L2CValue) as $ty
                    }
                }
            }

            impl Into<$ty> for &L2CValue {
                fn into(self) -> $ty {
                    unsafe { 
                        L2CValue_as_integer(self as *const L2CValue) as $ty
                    }
                }
            }
        )*
    };
}

macro_rules! impl_into_float_l2cvalue {
    (
        $(
            $ty: ty
        )*
    ) => {
        $(
            impl Into<$ty> for L2CValue {
                fn into(self) -> $ty {
                    unsafe {
                        L2CValue_as_number(&self as *const L2CValue) as $ty
                    }
                }
            }

            impl Into<$ty> for &L2CValue {
                fn into(self) -> $ty {
                    unsafe {
                        L2CValue_as_number(self as *const L2CValue) as $ty
                    }
                }
            }
        )*
    };
}

macro_rules! impl_partialeq_int_l2cvalue {
    (
        $(
            $ty:ty
        )*
    ) => {
        $(
            impl PartialEq<$ty> for L2CValue {
                fn eq(&self, other: &$ty) -> bool {
                    self.val_type == L2CValueType::Int && (self.get_u64() as $ty) == *other
                }
            }

            impl PartialEq<L2CValue> for $ty {
                fn eq(&self, other: &L2CValue) -> bool {
                    other.val_type == L2CValueType::Int && (other.get_u64() as $ty) == *self
                }
            }
        )*
    }
}

macro_rules! impl_partialeq_float_l2cvalue {
    (
        $(
            $ty:ty
        )*
    ) => {
        $(
            impl PartialEq<$ty> for L2CValue {
                fn eq(&self, other: &$ty) -> bool {
                    self.val_type == L2CValueType::Num && (self.get_f32() as $ty) == *other
                }
            }

            impl PartialEq<L2CValue> for $ty {
                fn eq(&self, other: &L2CValue) -> bool {
                    other.val_type == L2CValueType::Num && (other.get_f32() as $ty) == *self
                }
            }
        )*
    }
}

impl_partialeq_int_l2cvalue!(i8 u8 i16 u16 i32 u32 u64 i64);
impl_partialeq_float_l2cvalue!(f32 f64);

impl_into_int_l2cvalue!(i8 u8 i16 u16 i32 u32 u64 i64);
impl_into_float_l2cvalue!(f32 f64);

// Wrappers for the strong L2CValues, which the rest of this crate depends on

impl L2CValue {
    #[inline]
    pub const fn const_new_void() -> Self {
        Self {
            val_type: L2CValueType::Void,
            unk1: 0,
            inner: L2CValueInner { raw: 0 as u64 },
            unk2: 0
        }
    }

    #[inline]
    pub const fn const_new_bool(val: bool) -> Self {
        Self {
            val_type: L2CValueType::Bool,
            unk1: 0,
            inner: L2CValueInner { raw: val as u64 },
            unk2: 0
        }
    }

    #[inline]
    pub const fn const_new_int(val: u64) -> Self {
        Self {
            val_type: L2CValueType::Int,
            unk1: 0,
            inner: L2CValueInner { raw: val as u64 },
            unk2: 0
        }
    }

    #[inline]
    pub const fn const_new_num(val: f32) -> Self {
        Self {
            val_type: L2CValueType::Num,
            unk1: 0,
            inner: L2CValueInner { raw_float: val },
            unk2: 0
        }
    }

    #[inline]
    pub const fn const_new_hash(val: u64) -> Self {
        Self {
            val_type: L2CValueType::Hash,
            unk1: 0,
            inner: L2CValueInner { raw: val },
            unk2: 0,
        }
    }

    #[inline]
    pub fn new_void() -> Self {
        L2CValue::Void()
    }

    #[inline]
    pub fn new_bool(val: bool) -> Self {
        L2CValue::Bool(val)
    }

    #[inline]
    pub fn new_int(val: u64) -> Self {
        L2CValue::U64(val)
    }

    #[inline]
    pub fn new_num(val: f32) -> Self {
        L2CValue::F32(val)
    }

    #[inline]
    pub fn new_hash(val: u64) -> Self {
        L2CValue::Hash40(Hash40::new_raw(val))
    }

    pub fn try_get_bool(&self) -> Option<bool> {
        if let L2CValueType::Bool = self.val_type {
            Some(unsafe { self.inner.raw } & 1 != 0)
        } else {
            None
        }
    }

    pub fn try_get_int(&self) -> Option<u64> {
        if let L2CValueType::Int = self.val_type {
            Some(unsafe { self.inner.raw })
        } else {
            None
        }
    }

    pub fn try_get_num(&self) -> Option<f32> {
        if let L2CValueType::Num = self.val_type {
            Some(unsafe { self.inner.raw_float })
        } else {
            None
        }
    }

    pub fn try_get_ptr<T>(&self) -> Option<*mut T> {
        if let L2CValueType::Pointer = self.val_type {
            Some(unsafe { self.inner.raw as *mut T })
        } else {
            None
        }
    }

    #[inline]
    pub fn get_int(&self) -> u64 {
        self.get_u64()
    }

    #[inline]
    pub fn get_num(&self) -> f32 {
        self.get_f32()
    }
}

impl L2CTable {
    pub fn new(size: i32) -> *mut Self {
        unsafe {
            let ret = cpp_new(0x48) as *mut L2CTable;
            L2CTable_L2CTable(ret, size);
            ret
        }
    }
}