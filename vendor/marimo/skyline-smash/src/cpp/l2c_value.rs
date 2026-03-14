use core::cmp::Ordering;
use core::fmt;

#[repr(C)]
#[derive(Copy, Clone)]
pub union L2CValueInner {
    pub raw: u64,
    pub raw_float: f32,
    pub raw_pointer: *mut libc::c_void,
    pub raw_table: *mut L2CTable,
    pub raw_innerfunc: *mut L2CInnerFunctionBase,
}

impl fmt::Debug for L2CValueInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            f.debug_tuple("")
                .field(&self.raw)
                .field(&self.raw_float)
                .field(&self.raw_pointer)
                .field(&self.raw_table)
                .field(&self.raw_innerfunc)
                .finish()
        }
    }
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum L2CValueType {
    Void = 0,
    Bool = 1,
    Int = 2,
    Num = 3,
    Pointer = 4,
    Table = 5,
    InnerFunc = 6,
    Hash = 7,
    String = 8,
}

#[repr(C)]
#[cfg_attr(not(feature = "weak_l2cvalue"), derive(Copy, Clone))]
pub struct L2CValue {
    pub val_type: L2CValueType,
    pub unk1: u32,
    pub inner: L2CValueInner,
    pub unk2: u8, // for enforcing X8 AArch64 struct behavior
}

impl Default for L2CValue {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::<Self>::uninit().assume_init() }
    }
}

impl fmt::Debug for L2CValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            match self.val_type {
                L2CValueType::Void => f
                    .debug_tuple("")
                    .field(&self.val_type)
                    .field(&self.inner.raw)
                    .finish(),
                L2CValueType::Bool => f
                    .debug_tuple("")
                    .field(&self.val_type)
                    .field(&(self.inner.raw != 0))
                    .finish(),
                L2CValueType::Int => f
                    .debug_tuple("")
                    .field(&self.val_type)
                    .field(&self.inner.raw)
                    .finish(),
                L2CValueType::Num => f
                    .debug_tuple("")
                    .field(&self.val_type)
                    .field(&self.inner.raw_float)
                    .finish(),
                L2CValueType::InnerFunc => f
                    .debug_tuple("")
                    .field(&self.val_type)
                    .field(&self.inner.raw_innerfunc)
                    .finish(),
                L2CValueType::Hash => f
                    .debug_tuple("")
                    .field(&self.val_type)
                    .field(&self.inner.raw)
                    .finish(),
                _ => f
                    .debug_tuple("")
                    .field(&self.val_type)
                    .field(&self.unk1)
                    .field(&self.inner.raw)
                    .field(&self.unk2)
                    .finish(),
            }
        }
    }
}

pub trait LuaTableIndex: Sized {
    fn index_with<'a>(&self, l2c_val: &'a L2CValue) -> &'a L2CValue;
    fn index_with_mut<'a>(&self, l2c_val: &'a mut L2CValue) -> &'a mut L2CValue;
}

impl LuaTableIndex for i32 {
    fn index_with<'a>(&self, l2c_val: &'a L2CValue) -> &'a L2CValue {
        unsafe { crate::lib::L2CValue__index_int(l2c_val, *self) }
    }

    fn index_with_mut<'a>(&self, l2c_val: &'a mut L2CValue) -> &'a mut L2CValue {
        unsafe { crate::lib::L2CValue__index_int_mut(l2c_val, *self) }
    }
}

impl LuaTableIndex for L2CValue {
    fn index_with<'a>(&self, l2c_val: &'a L2CValue) -> &'a L2CValue {
        unsafe { crate::lib::L2CValue__index_L2CValue(l2c_val, self) }
    }

    fn index_with_mut<'a>(&self, l2c_val: &'a mut L2CValue) -> &'a mut L2CValue {
        unsafe { crate::lib::L2CValue__index_L2CValue_mut(l2c_val, self) }
    }
}

impl LuaTableIndex for u64 {
    fn index_with<'a>(&self, l2c_val: &'a L2CValue) -> &'a L2CValue {
        unsafe { crate::lib::L2CValue__index_hash40(l2c_val, *self) }
    }

    fn index_with_mut<'a>(&self, l2c_val: &'a mut L2CValue) -> &'a mut L2CValue {
        unsafe { crate::lib::L2CValue__index_hash40_mut(l2c_val, *self) }
    }
}

impl LuaTableIndex for &str {
    fn index_with<'a>(&self, l2c_val: &'a L2CValue) -> &'a L2CValue {
        let hash = crate::hash40(*self);

        hash.index_with(l2c_val)
    }

    fn index_with_mut<'a>(&self, l2c_val: &'a mut L2CValue) -> &'a mut L2CValue {
        let hash = crate::hash40(*self);

        hash.index_with_mut(l2c_val)
    }
}

impl<Idx: LuaTableIndex> core::ops::Index<Idx> for L2CValue {
    type Output = L2CValue;

    fn index(&self, index: Idx) -> &Self::Output {
        index.index_with(self)
    }
}

impl<Idx: LuaTableIndex> core::ops::IndexMut<Idx> for L2CValue {
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        index.index_with_mut(self)
    }
}

pub fn lua_val<T: Into<L2CValue> + Sized>(val: T) -> L2CValue {
    val.into()
}

impl Into<L2CValue> for LuaConst {
    fn into(self) -> L2CValue {
        L2CValue::new_int(*self as u64)
    }
}

impl Default for L2CValueInner {
    fn default() -> Self {
        Self { raw: 0 }
    }
}

impl Default for L2CValueType {
    fn default() -> Self {
        Self::Void
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct L2CTable_meta {
    pub a: u64,
    pub b: u64,
    pub c: u64,
    pub d: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct L2CTable {
    pub refcnt: u32,
    pub unk: u32,
    pub begin: u64,
    pub end: u64,
    pub also_end: u64,
    pub meta: L2CTable_meta,
    pub unk_ptr: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct L2CInnerFunctionBase {
    pub unk: u64,
    pub refcnt: u32,
}

impl super::root::lib::L2CAgent {
    pub fn new(lua_state: u64) -> Self {
        unsafe {
            let mut l2c_agent = std::mem::MaybeUninit::uninit();
            super::root::lib::L2CAgent_L2CAgent_constr(l2c_agent.as_mut_ptr(), lua_state);
            l2c_agent.assume_init()
        }
    }
}
/*pub unsafe fn lua_const<S: AsRef<[u8]>>(string: S) -> libc::c_int {
    let mut val : i32 = -1;
    if lib::lua_bind_get_value(lua_bind_hash::lua_bind_hash(string), &mut val) {
        val
    } else{
        -1
    }
}*/

#[derive(Debug, Copy, Clone, Eq)]
pub struct LuaConst {
    value: i32,
}

impl LuaConst {
    pub const fn new(value: u32) -> Self {
        Self { value: value as i32 }
    }

    pub fn as_lua_int(&self) -> L2CValue {
        L2CValue::new_int(**self as u64)
    }
}
//Release

impl From<LuaConst> for i32 {
    fn from(lua_const: LuaConst) -> Self {
        *lua_const
    }
}

impl From<LuaConst> for u32 {
    fn from(lua_const: LuaConst) -> Self {
        *lua_const as u32
    }
}

macro_rules! lua_const_partialeq_impl {
    (
        $(
            $ty:ty
        )*
    ) => {
        $(
            impl PartialEq<$ty> for LuaConst {
                fn eq(&self, other: &$ty) -> bool {
                    return **self == other.clone() as i32;
                }
            }

            impl PartialOrd<$ty> for LuaConst {
                fn partial_cmp(&self, other: &$ty) -> Option<Ordering> {
                    Some((**self).cmp(&(other.clone() as i32)))
                }
            }

            impl PartialEq<LuaConst> for $ty {
                fn eq(&self, other: &LuaConst) -> bool {
                    return *self as i32 == **other;
                }
            }

            impl PartialOrd<LuaConst> for $ty {
                fn partial_cmp(&self, other: &LuaConst) -> Option<Ordering> {
                    Some((*self as i32).cmp(&**other))
                }
            }
        )*
    };
}

impl PartialEq for LuaConst {
    fn eq(&self, other: &LuaConst) -> bool {
        *self == **other
    }
}

impl PartialEq<LuaConst> for L2CValue {
    fn eq(&self, other: &LuaConst) -> bool {
        self.val_type == L2CValueType::Int && **other == self.get_int() as i32
    }
}

impl PartialEq<L2CValue> for LuaConst {
    fn eq(&self, other: &L2CValue) -> bool {
        other.val_type == L2CValueType::Int && **self == other.get_int() as i32
    }
}

impl PartialOrd<LuaConst> for L2CValue {
    #[track_caller]
    fn partial_cmp(&self, other: &LuaConst) -> Option<Ordering> {
        Some((self.get_int() as i32).cmp(&**other))
    }
}

lua_const_partialeq_impl!(i32 u32 u64);

impl core::ops::Deref for LuaConst {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
