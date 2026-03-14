use skyline::libc::free;
use std::{
    iter::FromIterator,
    ops::{Index, IndexMut, Range},
};

#[repr(C)]
pub struct Vector<T> {
    start: *mut T,
    end: *mut T,
    eos: *mut T,
}

impl<T> Vector<T> {
    pub fn capacity(&self) -> usize {
        if self.start.is_null() || self.eos.is_null() {
            0
        } else {
            unsafe { self.eos.offset_from(self.start) as usize }
        }
    }

    pub fn len(&self) -> usize {
        if self.start.is_null() || self.end.is_null() {
            0
        } else {
            unsafe { self.end.offset_from(self.start) as usize }
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.as_slice().map(|x| x.get(index)).flatten()
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.as_mut_slice().map(|x| x.get_mut(index)).flatten()
    }

    pub fn as_ptr(&self) -> *const T {
        self.start
    }

    pub fn as_mut_ptr(&self) -> *mut T {
        self.start
    }

    pub fn as_slice(&self) -> Option<&[T]> {
        if self.start.is_null() {
            None
        } else {
            unsafe { Some(std::slice::from_raw_parts(self.start, self.len())) }
        }
    }

    pub fn as_mut_slice(&mut self) -> Option<&mut [T]> {
        if self.start.is_null() {
            None
        } else {
            unsafe { Some(std::slice::from_raw_parts_mut(self.start, self.len())) }
        }
    }

    pub fn iter(&self) -> VectorIter<T> {
        VectorIter::new(self)
    }

    pub fn iter_mut(&mut self) -> VectorIterMut<T> {
        VectorIterMut::new(self)
    }

    pub fn into_vec(self) -> Vec<T> {
        unsafe {
            let vec = Vec::from_iter(self.iter().map(|x| std::ptr::read(x as *const T)));
            std::mem::forget(self);
            vec
        }
    }

    pub fn clear(&mut self) {
        self.truncate(0);
    }

    pub fn truncate(&mut self, new_len: usize) {
        while self.len() > new_len {
            unsafe {
                let last = self.end.sub(1);
                std::ptr::drop_in_place(last);
                self.end = last;
            }
        }
    }

    unsafe fn realloc_and_grow(&mut self) {
        let len = self.len();
        let current_capacity = self.capacity();
        let new_capacity = if current_capacity == 0 {
            1
        } else {
            current_capacity * 2
        };

        let new_mem = skyline::libc::memalign(
            std::mem::align_of::<T>(),
            std::mem::size_of::<T>() * new_capacity,
        );

        if current_capacity == 0 {
            self.start = new_mem.cast();
            self.end = new_mem.cast();
            self.eos = new_mem.cast::<T>().add(new_capacity);
            return;
        }

        std::ptr::copy_nonoverlapping(self.start, new_mem.cast(), len);

        let old_mem = self.start;
        self.start = new_mem.cast();
        self.end = new_mem.cast::<T>().add(len);
        self.eos = new_mem.cast::<T>().add(new_capacity);

        skyline::libc::free(old_mem.cast());
    }

    pub fn push(&mut self, item: T) {
        if unsafe { self.eos.offset_from(self.end) <= std::mem::size_of::<T>() as isize } {
            unsafe { self.realloc_and_grow() };
        }

        unsafe {
            std::ptr::write(self.end, item);
            self.end = self.end.add(1);
        }
    }
}

impl<T> Drop for Vector<T> {
    fn drop(&mut self) {
        unsafe {
            let mut current = self.start;
            while current != self.end {
                std::ptr::drop_in_place(current);
                current = current.add(1);
            }
            free(self.start as _);
        }
    }
}

impl<T> Index<usize> for Vector<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.as_slice().expect("Attempted to index null vector!")[index]
    }
}

impl<T> IndexMut<usize> for Vector<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self
            .as_mut_slice()
            .expect("Attempted to index null vector!")[index]
    }
}

impl<T> Index<Range<usize>> for Vector<T> {
    type Output = [T];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.as_slice().expect("Attempted to index null vector!")[index]
    }
}

impl<T> IndexMut<Range<usize>> for Vector<T> {
    fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {
        &mut self
            .as_mut_slice()
            .expect("Attempted to index null vector!")[index]
    }
}

pub struct VectorIter<'a, T> {
    vector: &'a Vector<T>,
    index: usize,
}

impl<'a, T> VectorIter<'a, T> {
    pub fn new(vector: &'a Vector<T>) -> Self {
        Self { vector, index: 0 }
    }
}

impl<'a, T> Iterator for VectorIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.vector.get(self.index);
        self.index += 1;
        result
    }
}

pub struct VectorIterMut<'a, T> {
    vector: &'a mut Vector<T>,
    index: usize,
}

impl<'a, T> VectorIterMut<'a, T> {
    pub fn new(vector: &'a mut Vector<T>) -> Self {
        Self { vector, index: 0 }
    }
}

impl<'a, T> Iterator for VectorIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.vector.get_mut(self.index);
        self.index += 1;
        unsafe {
            // hacky workaround, if you have a better idea please
            std::mem::transmute::<Option<&mut T>, Option<&'a mut T>>(result)
        }
    }
}
