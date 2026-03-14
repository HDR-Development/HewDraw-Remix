use std::ops::{Index, IndexMut};

use skyline::libc;


#[repr(C)]
pub struct FixedVec<T, const N: usize> {
    vtable: *const *const libc::c_void,
    capacity: usize,
    length: usize,
    start: usize,
    end: usize,
    ptr: *mut T,
    array: [T; N]
}

impl<T, const N: usize> FixedVec<T, N> {
    pub fn cap(&self) -> usize {
        self.capacity
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.length {
            Some(&self.array[(self.start + index) % self.capacity])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.length {
            Some(&mut self.array[(self.start + index) % self.capacity])
        } else {
            None
        }
    }

    pub fn push(&mut self, item: T) -> bool {
        if self.length == self.capacity {
            false
        } else {
            self.array[self.end] = item;
            if self.end + 1 == self.capacity {
                self.end = 0
            }
            self.length += 1;
            true
        }
    }

    pub fn push_overwrite(&mut self, item: T) {
        if self.length == self.capacity {
            self.array[self.end] = item;
            self.end = (self.end + 1) % self.capacity;
            self.start = (self.start + 1) % self.capacity;
        } else {
            self.array[self.end] = item;
            self.end = (self.end + 1) % self.capacity;
            self.length += 1;
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.length == 0 {
            None
        } else {
            let mut item = unsafe { std::mem::zeroed() };
            std::mem::swap(&mut item, &mut self.array[self.start]);
            self.start = (self.start + 1) % self.capacity;
            self.length -= 1;
            Some(item)
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.length == 0 {
            None
        } else {
            let mut item = unsafe { std::mem::zeroed() };
            std::mem::swap(&mut item, &mut self.array[self.start]);
            self.end = (self.end + self.capacity - 1) % self.capacity; // add cap - 1 instead of sub 1 since when we get to 0, -1 % cap == -1
            self.length -= 1;
            Some(item)
        }
    }

    pub fn iter<'a>(&'a self) -> FixedVecIter<'a, T, N> {
        FixedVecIter {
            index: 0,
            vec: self
        }
    }

    pub fn iter_mut<'a>(&'a mut self) -> FixedVecIterMut<'a, T, N> {
        FixedVecIterMut {
            index: 0,
            vec: self
        }
    }
}

impl<T, const N: usize> Index<usize> for FixedVec<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        self.get(index).expect("Index out of bounds")
    }
}

impl<T, const N: usize> IndexMut<usize> for FixedVec<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        self.get_mut(index).expect("Index out of bounds")
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a FixedVec<T, N> {
    type Item = &'a T;
    type IntoIter = FixedVecIter<'a, T, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a mut FixedVec<T, N> {
    type Item = &'a mut T;
    type IntoIter = FixedVecIterMut<'a, T, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

pub struct FixedVecIter<'a, T, const N: usize> {
    index: usize,
    vec: &'a FixedVec<T, N>
}

pub struct FixedVecIterMut<'a, T, const N: usize> {
    index: usize,
    vec: &'a mut FixedVec<T, N>
}

impl<'a, T, const N: usize> Iterator for FixedVecIter<'a, T, N> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.vec.get(self.index);
        self.index += 1;
        item
    }
}

impl<'a, T, const N: usize> Iterator for FixedVecIterMut<'a, T, N> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.vec.get_mut(self.index);
        self.index += 1;
        unsafe {
            std::mem::transmute(item)
        }
    }
}