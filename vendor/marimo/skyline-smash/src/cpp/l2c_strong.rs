use super::l2c_value::*;
impl L2CValue {
    pub const fn new_void() -> Self {
        Self {
            val_type: L2CValueType::Void,
            unk1: 0,
            inner: L2CValueInner { raw: 0 as u64 },
            unk2: 0
        }
    }

    pub const fn new_bool(val: bool) -> Self {
        Self {
            val_type: L2CValueType::Bool,
            unk1: 0,
            inner: L2CValueInner { raw: val as u64 },
            unk2: 0
        }
    }

    pub const fn new_int(val: u64) -> Self {
        Self {
            val_type: L2CValueType::Int,
            unk1: 0,
            inner: L2CValueInner { raw: val as u64 },
            unk2: 0
        }
    }

    pub const fn new_num(val: f32) -> Self {
        Self {
            val_type: L2CValueType::Num,
            unk1: 0,
            inner: L2CValueInner { raw_float: val },
            unk2: 0
        }
    }

    pub const fn new_hash(val: u64) -> Self {
        Self {
            val_type: L2CValueType::Hash,
            unk1: 0,
            inner: L2CValueInner { raw: val },
            unk2: 0,
        }
    }

    pub fn try_get_bool(&self) -> Option<bool> {
        if let L2CValueType::Bool = self.val_type {
            Some(unsafe { self.inner.raw } & 1 != 0)
        } else {
            None
        }
    }

    #[track_caller]
    pub fn get_bool(&self) -> bool {
        if let Some(val) = self.try_get_bool() {
            val
        } else {
            panic!("L2CValue: {:?} not a bool", self);
        }
    }

    pub fn try_get_int(&self) -> Option<u64> {
        if let L2CValueType::Int = self.val_type {
            Some(unsafe { self.inner.raw })
        } else {
            None
        }
    }

    #[track_caller]
    pub fn get_int(&self) -> u64 {
        if let Some(val) = self.try_get_int() {
            val
        } else {
            panic!("L2CValue: {:?} not an int", self);
        }
    }

    pub fn try_get_num(&self) -> Option<f32> {
        if let L2CValueType::Num = self.val_type {
            Some(unsafe { self.inner.raw_float })
        } else {
            None
        }
    }

    #[track_caller]
    pub fn get_num(&self) -> f32 {
        if let Some(val) = self.try_get_num() {
            val
        } else {
            panic!("L2CValue: {:?} not a float", self);
        }
    }

    pub fn try_get_ptr<T>(&self) -> Option<*mut T> {
        if let L2CValueType::Pointer = self.val_type {
            Some(unsafe { self.inner.raw as *mut T })
        } else {
            None
        }
    }

    #[track_caller]
    pub fn get_ptr<T>(&self) -> *mut T {
        if let Some(val) = self.try_get_ptr() {
            val
        } else {
            panic!("L2CValue: {:?} is not a pointer", self);
        }
    }

    pub fn assign(&mut self, other: &Self) {
        self.val_type = other.val_type;
        self.unk1 = other.unk1;
        self.inner = other.inner;
    }
}