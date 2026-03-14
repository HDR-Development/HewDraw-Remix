use crate::*;
use skyline::libc::{c_char, c_void, malloc};
use std::{cmp::Ordering, fmt, mem::MaybeUninit, ops::*};

extern "C" {
    #[link_name = "\u{1}_ZN3lib8L2CValueC1Ev"]
    fn nil_ctor(v: *mut L2CValue);

    #[link_name = "\u{1}_ZN3lib8L2CValueC1ERKS0_"]
    fn copy_ctor(v: *mut L2CValue, other: *const L2CValue);

    #[link_name = "\u{1}_ZN3lib8L2CValueC1Eb"]
    fn bool_ctor(v: *mut L2CValue, other: bool);

    #[link_name = "\u{1}_ZN3lib8L2CValueC1Ei"]
    fn int_ctor(v: *mut L2CValue, other: i32);

    #[link_name = "\u{1}_ZN3lib8L2CValueC1Ej"]
    fn uint_ctor(v: *mut L2CValue, other: u32);

    #[link_name = "\u{1}_ZN3lib8L2CValueC1El"]
    fn long_ctor(v: *mut L2CValue, other: i64);

    #[link_name = "\u{1}_ZN3lib8L2CValueC1Em"]
    fn ulong_ctor(v: *mut L2CValue, other: u64);

    #[link_name = "\u{1}_ZN3lib8L2CValueC1Ef"]
    fn float_ctor(v: *mut L2CValue, other: f32);

    #[link_name = "\u{1}_ZN3lib8L2CValueC1EPv"]
    fn ptr_ctor(v: *mut L2CValue, other: *mut c_void);

    #[link_name = "\u{1}_ZN3lib8L2CValueC1EPNS_8L2CTableE"]
    fn table_ctor(v: *mut L2CValue, other: *mut lib::L2CTable);

    #[allow(dead_code)]
    #[link_name = "\u{1}_ZN3lib8L2CValueC1EPNS_20L2CInnerFunctionBaseE"]
    fn func_ctor(v: *mut L2CValue, other: *mut lib::L2CInnerFunctionBase);

    #[link_name = "\u{1}_ZN3lib8L2CValueC1EN3phx6Hash40E"]
    fn hash_ctor(v: *mut L2CValue, other: phx::Hash40);

    #[link_name = "\u{1}_ZN3lib8L2CValueC1EPKc"]
    fn string_ctor(v: *mut L2CValue, c_string: *const c_char);

    #[link_name = "\u{1}_ZNK3lib8L2CValuentEv"]
    fn logic_not(v: *const L2CValue) -> L2CValueHack;

    #[link_name = "\u{1}_ZNK3lib8L2CValuermERKS0_"]
    fn modulo(v: *const L2CValue, other: *const L2CValue) -> L2CValueHack;

    #[link_name = "\u{1}_ZNK3lib8L2CValueanERKS0_"]
    fn bit_and(v: *const L2CValue, other: *const L2CValue) -> L2CValueHack;

    #[link_name = "\u{1}_ZNK3lib8L2CValuemlERKS0_"]
    fn multiply(v: *const L2CValue, other: *const L2CValue) -> L2CValueHack;

    #[link_name = "\u{1}_ZNK3lib8L2CValueplERKS0_"]
    fn add(v: *const L2CValue, other: *const L2CValue) -> L2CValueHack;

    #[link_name = "\u{1}_ZNK3lib8L2CValuengEv"]
    fn negate(v: *const L2CValue) -> L2CValueHack;

    #[link_name = "\u{1}_ZNK3lib8L2CValuemiERKS0_"]
    fn subtract(v: *const L2CValue, other: *const L2CValue) -> L2CValueHack;

    #[allow(dead_code)]
    #[link_name = "\u{1}_ZNK3lib8L2CValuecvbEv"]
    fn bool_cast(v: *const L2CValue) -> bool;

    #[link_name = "\u{1}_ZNK3lib8L2CValuedvERKS0_"]
    fn divide(v: *const L2CValue, other: *const L2CValue) -> L2CValueHack;

    #[link_name = "\u{1}_ZNK3lib8L2CValueltERKS0_"]
    fn less_than(v: *const L2CValue, other: *const L2CValue) -> bool;

    #[link_name = "\u{1}_ZNK3lib8L2CValuelsERKS0_"]
    fn shift_left(v: *const L2CValue, other: *const L2CValue) -> L2CValueHack;

    #[allow(dead_code)]
    #[link_name = "\u{1}_ZNK3lib8L2CValueleERKS0_"]
    fn less_equal(v: *const L2CValue, other: *const L2CValue) -> L2CValueHack;

    #[link_name = "\u{1}_ZN3lib8L2CValueaSERKS0_"]
    fn assign(v: *mut L2CValue, other: *const L2CValue) -> *mut L2CValue;

    #[link_name = "\u{1}_ZNK3lib8L2CValueeqERKS0_"]
    fn equals(v: *const L2CValue, other: *const L2CValue) -> bool;

    #[link_name = "\u{1}_ZNK3lib8L2CValuersERKS0_"]
    fn shift_right(v: *const L2CValue, other: *const L2CValue) -> L2CValueHack;

    #[link_name = "\u{1}_ZNK3lib8L2CValueixEi"]
    fn int_index(v: *const L2CValue, index: i32) -> *mut L2CValue;

    #[link_name = "\u{1}_ZNK3lib8L2CValueixEN3phx6Hash40E"]
    fn hash_index(v: *const L2CValue, hash: phx::Hash40) -> *mut L2CValue;

    #[link_name = "\u{1}_ZNK3lib8L2CValueixERKS0_"]
    fn value_index(v: *const L2CValue, other: *const L2CValue) -> *mut L2CValue;

    #[link_name = "\u{1}_ZNK3lib8L2CValueeoERKS0_"]
    fn bit_xor(v: *const L2CValue, other: *const L2CValue) -> L2CValueHack;

    #[link_name = "\u{1}_ZNK3lib8L2CValueorERKS0_"]
    fn bit_or(v: *const L2CValue, other: *const L2CValue) -> L2CValueHack;

    #[link_name = "\u{1}_ZNK3lib8L2CValuecoEv"]
    fn bit_not(v: *const L2CValue) -> L2CValueHack;

    #[link_name = "\u{1}_ZN3lib8L2CValueD1Ev"]
    fn dtor(v: *mut L2CValue);

    #[link_name = "\u{1}_ZNK3lib8L2CValue7as_boolEv"]
    fn as_bool(v: *const L2CValue) -> bool;

    #[link_name = "\u{1}_ZNK3lib8L2CValue7as_hashEv"]
    fn as_hash(v: *const L2CValue) -> phx::Hash40;

    #[allow(dead_code)]
    #[link_name = "\u{1}_ZNK3lib8L2CValue17as_inner_functionEv"]
    fn as_inner_function(v: *const L2CValue) -> *mut lib::L2CInnerFunctionBase;

    #[link_name = "\u{1}_ZNK3lib8L2CValue10as_integerEv"]
    fn as_integer(v: *const L2CValue) -> u64;

    #[link_name = "\u{1}_ZNK3lib8L2CValue9as_numberEv"]
    fn as_number(v: *const L2CValue) -> f32;

    #[link_name = "\u{1}_ZNK3lib8L2CValue10as_pointerEv"]
    fn as_pointer(v: *const L2CValue) -> *mut c_void;

    #[allow(dead_code)]
    #[link_name = "\u{1}_ZNK3lib8L2CValue9as_stringEv"]
    fn as_string(v: *const L2CValue) -> *const c_char;

    #[allow(dead_code)]
    #[link_name = "\u{1}_ZNK3lib8L2CValue8as_tableEv"]
    fn as_table(v: *const L2CValue) -> *mut lib::L2CTable;

    #[link_name = "\u{1}_ZNK3lib8L2CValue9to_stringEv"]
    fn to_string(v: *const L2CValue) -> lib::L2CValueHack;
}

#[repr(C)]
pub struct L2CValueHack {
    value: L2CValue,
    extra_space: [u8; 0x10],
}

impl Into<L2CValue> for L2CValueHack {
    fn into(self) -> L2CValue {
        let L2CValueHack { value, .. } = self;
        value
    }
}

impl From<L2CValue> for L2CValueHack {
    #[allow(invalid_value)]
    fn from(value: L2CValue) -> Self {
        Self {
            value,
            extra_space: unsafe { std::mem::MaybeUninit::uninit().assume_init() },
        }
    }
}

#[cfg(feature = "type_assert")]
impl L2CValueHack {
    pub fn assert() {
        assert_eq!(size_of!(L2CValueHack), 0x20);
        assert_eq!(offset_of!(L2CValueHack, value), 0x0);
        assert_eq!(offset_of!(L2CValueHack, extra_space), 0x10);
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ValueType {
    Nil = 0,
    Bool = 1,
    Integer = 2,
    Number = 3,
    UserData = 4,
    Table = 5,
    InnerFunction = 6,
    Hash = 7,
    String = 8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union InnerData {
    pub integer: u64,
    pub float: f32,
    pub user_data: *mut c_void,
    pub table: *mut lib::L2CTable,
    pub inner_function: *mut lib::L2CInnerFunctionBase,
    pub hash: phx::Hash40,
    pub c_string: *const cpp::String,
}

#[repr(C)]
pub struct L2CValue {
    pub ty: ValueType,
    pub padding: u32,
    pub raw: InnerData,
}

impl L2CValue {
    pub fn is_nil(&self) -> bool {
        matches!(self.ty, ValueType::Nil)
    }

    pub fn try_bool(&self) -> Option<bool> {
        unsafe {
            if matches!(self.ty, ValueType::Bool) {
                Some(self.raw.integer != 0)
            } else {
                None
            }
        }
    }

    pub fn try_integer(&self) -> Option<u64> {
        unsafe {
            if matches!(self.ty, ValueType::Integer) {
                Some(self.raw.integer)
            } else {
                None
            }
        }
    }

    pub fn try_float(&self) -> Option<f32> {
        unsafe {
            if matches!(self.ty, ValueType::Number) {
                Some(self.raw.float)
            } else {
                None
            }
        }
    }

    pub fn try_pointer(&self) -> Option<*mut c_void> {
        unsafe {
            if matches!(self.ty, ValueType::UserData) {
                if self.raw.user_data.is_null() {
                    None
                } else {
                    Some(self.raw.user_data)
                }
            } else {
                None
            }
        }
    }

    pub fn try_table(&self) -> Option<&lib::L2CTable> {
        unsafe {
            if matches!(self.ty, ValueType::Table) {
                self.raw.table.as_ref()
            } else {
                None
            }
        }
    }

    pub fn try_table_mut(&mut self) -> Option<&mut lib::L2CTable> {
        unsafe {
            if matches!(self.ty, ValueType::Table) {
                self.raw.table.as_mut()
            } else {
                None
            }
        }
    }

    pub fn try_inner_function(&self) -> Option<&lib::L2CInnerFunctionBase> {
        unsafe {
            if matches!(self.ty, ValueType::InnerFunction) {
                self.raw.inner_function.as_ref()
            } else {
                None
            }
        }
    }

    pub fn try_hash(&self) -> Option<phx::Hash40> {
        unsafe {
            if matches!(self.ty, ValueType::Hash) {
                Some(self.raw.hash)
            } else {
                None
            }
        }
    }

    pub fn try_string(&self) -> Option<String> {
        unsafe {
            if matches!(self.ty, ValueType::String) {
                if self.raw.c_string.is_null() {
                    None
                } else {
                    Some((*self.raw.c_string).to_string())
                }
            } else {
                None
            }
        }
    }

    pub fn from_table(table: *mut lib::L2CTable) -> Self {
        unsafe {
            let mut value = MaybeUninit::uninit();
            table_ctor(value.as_mut_ptr(), table);
            value.assume_init()
        }
    }

    pub fn nil() -> Self {
        unsafe {
            let mut value = MaybeUninit::uninit();
            nil_ctor(value.as_mut_ptr());
            value.assume_init()
        }
    }

    pub fn new<T: Into<L2CValue>>(val: T) -> Self {
        val.into()
    }

    pub fn get<'a, T>(&'a self) -> T
    where
        &'a L2CValue: Into<T>,
    {
        self.into()
    }

    pub fn new_table() -> Self {
        unsafe {
            let table = malloc(std::mem::size_of::<lib::L2CTable>()) as *mut lib::L2CTable;
            lib::L2CTable::initialize_raw_memory(table, 0);
            Self::from_table(table)
        }
    }

    pub fn to_string(&self) -> String {
        unsafe {
            let value: L2CValue = to_string(self).into();
            if matches!(value.ty, ValueType::String) {
                (*value.raw.c_string).to_string()
            } else {
                String::from("error!")
            }
        }
    }

    pub fn invert(&self) -> L2CValue {
        unsafe { bit_not(self).into() }
    }
}

impl Drop for L2CValue {
    fn drop(&mut self) {
        unsafe {
            dtor(self);
        }
    }
}

impl Clone for L2CValue {
    fn clone(&self) -> Self {
        unsafe {
            let mut value = MaybeUninit::uninit();
            copy_ctor(value.as_mut_ptr(), self);
            value.assume_init()
        }
    }
}

impl From<&str> for L2CValue {
    fn from(string: &str) -> Self {
        let c_string = [string, "\0"].concat();
        unsafe {
            let mut value = MaybeUninit::uninit();
            string_ctor(value.as_mut_ptr(), c_string.as_ptr() as _);
            value.assume_init()
        }
    }
}

impl From<String> for L2CValue {
    fn from(string: String) -> Self {
        let c_string = [string.as_ref(), "\0"].concat();
        unsafe {
            let mut value = MaybeUninit::uninit();
            string_ctor(value.as_mut_ptr(), c_string.as_ptr() as _);
            value.assume_init()
        }
    }
}

impl Into<bool> for &L2CValue {
    fn into(self) -> bool {
        unsafe { as_bool(self) }
    }
}

impl Into<i32> for &L2CValue {
    fn into(self) -> i32 {
        unsafe { as_integer(self) as i32 }
    }
}

impl Into<u32> for &L2CValue {
    fn into(self) -> u32 {
        unsafe { as_integer(self) as u32 }
    }
}

impl Into<i64> for &L2CValue {
    fn into(self) -> i64 {
        unsafe { as_integer(self) as i64 }
    }
}

impl Into<u64> for &L2CValue {
    fn into(self) -> u64 {
        unsafe { as_integer(self) as u64 }
    }
}

impl Into<f32> for &L2CValue {
    fn into(self) -> f32 {
        unsafe { as_number(self) }
    }
}

impl<T> Into<*mut T> for &L2CValue {
    fn into(self) -> *mut T {
        unsafe { as_pointer(self) as _ }
    }
}

impl Into<phx::Hash40> for &L2CValue {
    fn into(self) -> phx::Hash40 {
        unsafe { as_hash(self) }
    }
}

impl<T> From<*mut T> for L2CValue {
    fn from(other: *mut T) -> Self {
        unsafe {
            let mut value = MaybeUninit::uninit();
            ptr_ctor(value.as_mut_ptr(), other as _);
            value.assume_init()
        }
    }
}

impl<T> From<*const T> for L2CValue {
    fn from(other: *const T) -> Self {
        unsafe {
            let mut value = MaybeUninit::uninit();
            ptr_ctor(value.as_mut_ptr(), other as _);
            value.assume_init()
        }
    }
}

impl fmt::Display for L2CValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.ty {
            ValueType::Nil if f.alternate() => write!(f, "(nil)"),
            ValueType::Nil => write!(f, "nil"),
            ValueType::Bool if f.alternate() => write!(f, "Bool({})", self.try_bool().unwrap()),
            ValueType::Bool => write!(f, "{}", self.try_bool().unwrap()),
            ValueType::Integer if f.alternate() => {
                write!(f, "Integer({})", self.try_integer().unwrap())
            }
            ValueType::Integer => write!(f, "{}", self.try_integer().unwrap()),
            ValueType::Number if f.alternate() => {
                write!(f, "Number({})", self.try_float().unwrap())
            }
            ValueType::Number => write!(f, "{}", self.try_float().unwrap()),
            ValueType::UserData if f.alternate() => {
                write!(f, "UserData({:#x})", self.try_pointer().unwrap() as u64)
            }
            ValueType::UserData => write!(f, "{:#x}", self.try_pointer().unwrap() as u64),
            ValueType::Table => write!(f, "Table"),
            ValueType::InnerFunction => write!(f, "InnerFunction"),
            ValueType::Hash => write!(f, "Hash({})", self.try_hash().unwrap()),
            ValueType::String => write!(f, "\"{}\"", self.try_string().unwrap()),
        }
    }
}

macro_rules! impl_arith_trait {
    ($trait:ident, $assign_trait:ident, $impl_fn_name:ident, $assign_fn_name:ident, $fn_name:ident) => {
        impl $trait for L2CValue {
            type Output = L2CValue;

            fn $impl_fn_name(self, rhs: Self) -> Self::Output {
                unsafe { $fn_name(&self, &rhs).into() }
            }
        }

        impl $assign_trait for L2CValue {
            fn $assign_fn_name(&mut self, rhs: Self) {
                unsafe {
                    assign(self, &$fn_name(self, &rhs).into());
                }
            }
        }

        impl $trait for &L2CValue {
            type Output = L2CValue;

            fn $impl_fn_name(self, rhs: Self) -> Self::Output {
                unsafe { $fn_name(self, rhs).into() }
            }
        }

        impl $assign_trait<&L2CValue> for &mut L2CValue {
            fn $assign_fn_name(&mut self, rhs: &L2CValue) {
                unsafe {
                    assign(*self, &$fn_name(*self, rhs).into());
                }
            }
        }

        impl $trait<L2CValue> for &L2CValue {
            type Output = L2CValue;

            fn $impl_fn_name(self, rhs: L2CValue) -> Self::Output {
                unsafe { $fn_name(self, &rhs).into() }
            }
        }

        impl $assign_trait<L2CValue> for &mut L2CValue {
            fn $assign_fn_name(&mut self, rhs: L2CValue) {
                unsafe {
                    assign(*self, &$fn_name(*self, &rhs).into());
                }
            }
        }

        impl $trait<&L2CValue> for L2CValue {
            type Output = L2CValue;

            fn $impl_fn_name(self, rhs: &L2CValue) -> Self::Output {
                unsafe { $fn_name(&self, rhs).into() }
            }
        }

        impl $assign_trait<&L2CValue> for L2CValue {
            fn $assign_fn_name(&mut self, rhs: &L2CValue) {
                unsafe {
                    assign(self, &$fn_name(self, rhs).into());
                }
            }
        }
    };
}

macro_rules! impl_from {
    ($from:ty, $fn_name:ident) => {
        impl From<$from> for L2CValue {
            fn from(other: $from) -> Self {
                unsafe {
                    let mut value = MaybeUninit::uninit();
                    $fn_name(value.as_mut_ptr(), other);
                    value.assume_init()
                }
            }
        }
    };
}

impl_arith_trait!(Add, AddAssign, add, add_assign, add);
impl_arith_trait!(Sub, SubAssign, sub, sub_assign, subtract);
impl_arith_trait!(Mul, MulAssign, mul, mul_assign, multiply);
impl_arith_trait!(Div, DivAssign, div, div_assign, divide);
impl_arith_trait!(Shl, ShlAssign, shl, shl_assign, shift_left);
impl_arith_trait!(Shr, ShrAssign, shr, shr_assign, shift_right);
impl_arith_trait!(Rem, RemAssign, rem, rem_assign, modulo);
impl_arith_trait!(BitOr, BitOrAssign, bitor, bitor_assign, bit_or);
impl_arith_trait!(BitAnd, BitAndAssign, bitand, bitand_assign, bit_and);
impl_arith_trait!(BitXor, BitXorAssign, bitxor, bitxor_assign, bit_xor);

impl Neg for L2CValue {
    type Output = L2CValue;

    fn neg(self) -> Self::Output {
        unsafe { negate(&self).into() }
    }
}

impl Neg for &L2CValue {
    type Output = L2CValue;

    fn neg(self) -> Self::Output {
        unsafe { negate(self).into() }
    }
}

impl Not for L2CValue {
    type Output = bool;

    fn not(self) -> Self::Output {
        unsafe { as_bool(&logic_not(&self).into()) }
    }
}

impl Not for &L2CValue {
    type Output = bool;

    fn not(self) -> Self::Output {
        unsafe { as_bool(&logic_not(self).into()) }
    }
}

impl_from!(bool, bool_ctor);
impl_from!(i32, int_ctor);
impl_from!(u32, uint_ctor);
impl_from!(i64, long_ctor);
impl_from!(u64, ulong_ctor);
impl_from!(f32, float_ctor);
impl_from!(phx::Hash40, hash_ctor);

impl PartialOrd for L2CValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        unsafe {
            if less_than(self, other) {
                Some(Ordering::Less)
            } else if less_than(other, self) {
                Some(Ordering::Greater)
            } else if equals(other, self) {
                Some(Ordering::Equal)
            } else {
                None
            }
        }
    }
}

impl PartialEq for L2CValue {
    fn eq(&self, other: &Self) -> bool {
        unsafe { equals(other, self) }
    }
}

macro_rules! impl_int_index {
    ($int:ty) => {
        impl Index<$int> for L2CValue {
            type Output = L2CValue;

            fn index(&self, index: $int) -> &Self::Output {
                unsafe {
                    let result = int_index(self, index as i32);
                    if result.is_null() {
                        panic!("Failed to index L2CValue");
                    } else {
                        &*result
                    }
                }
            }
        }

        impl IndexMut<$int> for L2CValue {
            fn index_mut(&mut self, index: $int) -> &mut Self::Output {
                unsafe {
                    let result = int_index(self, index as i32);
                    if result.is_null() {
                        panic!("Failed to index L2CValue");
                    } else {
                        &mut *result
                    }
                }
            }
        }
    };
}

impl_int_index!(i32);
impl_int_index!(u32);
impl_int_index!(i64);
impl_int_index!(u64);
impl_int_index!(isize);
impl_int_index!(usize);

impl Index<L2CValue> for L2CValue {
    type Output = L2CValue;

    fn index(&self, value: L2CValue) -> &Self::Output {
        unsafe {
            let result = value_index(self, &value);
            if result.is_null() {
                panic!("Failed to index L2CValue");
            } else {
                &*result
            }
        }
    }
}

impl IndexMut<L2CValue> for L2CValue {
    fn index_mut(&mut self, value: L2CValue) -> &mut Self::Output {
        unsafe {
            let result = value_index(self, &value);
            if result.is_null() {
                panic!("Failed to index L2CValue");
            } else {
                &mut *result
            }
        }
    }
}

impl Index<&L2CValue> for L2CValue {
    type Output = L2CValue;

    fn index(&self, value: &L2CValue) -> &Self::Output {
        unsafe {
            let result = value_index(self, value);
            if result.is_null() {
                panic!("Failed to index L2CValue");
            } else {
                &*result
            }
        }
    }
}

impl IndexMut<&L2CValue> for L2CValue {
    fn index_mut(&mut self, value: &L2CValue) -> &mut Self::Output {
        unsafe {
            let result = value_index(self, value);
            if result.is_null() {
                panic!("Failed to index L2CValue");
            } else {
                &mut *result
            }
        }
    }
}

impl Index<phx::Hash40> for L2CValue {
    type Output = L2CValue;

    fn index(&self, hash: phx::Hash40) -> &Self::Output {
        unsafe {
            let result = hash_index(self, hash);
            if result.is_null() {
                panic!("Failed to index L2CValue");
            } else {
                &*result
            }
        }
    }
}

impl Index<&str> for L2CValue {
    type Output = L2CValue;

    fn index(&self, index: &str) -> &Self::Output {
        unsafe {
            let result = hash_index(self, phx::hash40(index));
            if result.is_null() {
                panic!("Failed to index L2CValue");
            } else {
                &*result
            }
        }
    }
}

impl Index<String> for L2CValue {
    type Output = L2CValue;

    fn index(&self, index: String) -> &Self::Output {
        unsafe {
            let result = hash_index(self, phx::hash40(&index));
            if result.is_null() {
                panic!("Failed to index L2CValue");
            } else {
                &*result
            }
        }
    }
}

impl IndexMut<phx::Hash40> for L2CValue {
    fn index_mut(&mut self, hash: phx::Hash40) -> &mut Self::Output {
        unsafe {
            let result = hash_index(self, hash);
            if result.is_null() {
                panic!("Failed to index L2CValue");
            } else {
                &mut *result
            }
        }
    }
}

impl IndexMut<&str> for L2CValue {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        unsafe {
            let result = hash_index(self, phx::hash40(index));
            if result.is_null() {
                panic!("Failed to index L2CValue");
            } else {
                &mut *result
            }
        }
    }
}

impl IndexMut<String> for L2CValue {
    fn index_mut(&mut self, index: String) -> &mut Self::Output {
        unsafe {
            let result = hash_index(self, phx::hash40(&index));
            if result.is_null() {
                panic!("Failed to index L2CValue");
            } else {
                &mut *result
            }
        }
    }
}

#[cfg(feature = "type_assert")]
impl L2CValue {
    pub fn assert() {
        assert_eq!(size_of!(L2CValue), 0x10);
        assert_eq!(offset_of!(L2CValue, ty), 0x0);
        assert_eq!(offset_of!(L2CValue, raw), 0x8);
    }
}
