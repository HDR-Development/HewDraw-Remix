use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use super::super::{BattleObject, Fighter};
use crate::cpp;

#[repr(C)]
pub struct BattleObjectTable {
    vtable: *const *const (),
    objects: *mut BattleObject,
    addresses: *mut u16,
    count: u32,
    mutex: cpp::Mutex,
}

#[repr(C)]
pub struct BattleObjectManagerInner {
    active: *mut BattleObject,
    inactive: *mut BattleObject,
    unk: [u8; 0x90],
    fighters: BattleObjectTable,
    weapons: BattleObjectTable,
    enemies: BattleObjectTable,
    gimmicks: BattleObjectTable,
    items: BattleObjectTable,
}

impl BattleObjectManagerInner {
    pub fn active_iter(&self) -> BattleObjectIterator {
        BattleObjectIterator {
            first: self.active,
            current: self.active,
            has_started: false,
        }
    }

    pub fn active_iter_mut(&mut self) -> BattleObjectIteratorMut {
        BattleObjectIteratorMut {
            first: self.active,
            current: self.active,
            has_started: false,
        }
    }

    pub fn fighters(&self) -> CastedObjectIterator<Fighter> {
        CastedObjectIterator {
            start: self.fighters.objects,
            count: self.fighters.count as usize,
            current: 0,
            object_size: 0xf940,
            _phantom: PhantomData,
        }
    }

    pub fn fighters_mut(&mut self) -> CastedObjectIteratorMut<Fighter> {
        CastedObjectIteratorMut {
            start: self.fighters.objects,
            count: self.fighters.count as usize,
            current: 0,
            object_size: 0xf940,
            _phantom: PhantomData,
        }
    }
}

#[repr(C)]
pub struct BattleObjectManager {
    inner: *mut BattleObjectManagerInner,
}

impl BattleObjectManager {
    pub fn instance() -> Option<&'static Self> {
        unsafe {
            *((skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *const u8)
                .add(0x52b5a00) as *const Option<&'static Self>)
        }
    }

    pub fn instance_mut() -> Option<&'static mut Self> {
        unsafe {
            std::ptr::read(
                (skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *const u8)
                    .add(0x52b5a00) as *const Option<&'static mut Self>,
            )
        }
    }
}

impl Deref for BattleObjectManager {
    type Target = BattleObjectManagerInner;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.inner }
    }
}

impl DerefMut for BattleObjectManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.inner }
    }
}

pub struct BattleObjectIterator {
    first: *const BattleObject,
    current: *const BattleObject,
    has_started: bool,
}

impl Iterator for BattleObjectIterator {
    type Item = &'static BattleObject;

    fn next(&mut self) -> Option<Self::Item> {
        if self.has_started && self.current == self.first {
            return None;
        }

        self.has_started = true;
        let current = self.current;

        unsafe {
            let mut next = (*current).next as *const _;
            loop {
                if next == self.first {
                    return None;
                }

                if (*next).unknown_byte3 >= (*current).unknown_byte3 {
                    break;
                }

                next = (*current).next as *const _;
            }

            self.current = next;

            Some(&*next)
        }
    }
}

pub struct BattleObjectIteratorMut {
    first: *mut BattleObject,
    current: *mut BattleObject,
    has_started: bool,
}

impl Iterator for BattleObjectIteratorMut {
    type Item = &'static mut BattleObject;

    fn next(&mut self) -> Option<Self::Item> {
        if self.has_started && self.current == self.first {
            return None;
        }

        self.has_started = true;
        let current = self.current;

        unsafe {
            let mut next = (*current).next;
            loop {
                if next == self.first {
                    return None;
                }

                if (*next).unknown_byte3 >= (*current).unknown_byte3 {
                    break;
                }

                next = (*next).next;
            }

            self.current = next;

            Some(&mut *current)
        }
    }
}

pub struct CastedObjectIterator<T: 'static> {
    start: *const BattleObject,
    count: usize,
    current: usize,
    object_size: usize,
    _phantom: PhantomData<T>,
}

impl<T: 'static> Iterator for CastedObjectIterator<T> {
    type Item = &'static T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            loop {
                if self.current == self.count {
                    return None;
                }

                let next = self.start.byte_add(self.object_size * self.current);
                self.current += 1;

                if (*next).status < 3 {
                    continue;
                }

                return Some(&*(next as *const T));
            }
        }
    }
}

pub struct CastedObjectIteratorMut<T: 'static> {
    start: *mut BattleObject,
    count: usize,
    current: usize,
    object_size: usize,
    _phantom: PhantomData<T>,
}

impl<T: 'static> Iterator for CastedObjectIteratorMut<T> {
    type Item = &'static mut T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            loop {
                if self.current == self.count {
                    return None;
                }

                let next = self.start.byte_add(self.object_size * self.current);
                self.current += 1;

                if (*next).status < 3 {
                    continue;
                }

                return Some(&mut *(next as *mut T));
            }
        }
    }
}
