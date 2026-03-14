use std::ops::{Index, IndexMut};

#[repr(C)]
pub struct SplitBuffer<T> {
    first: *mut *mut T,
    begin: *mut *mut T,
    end: *mut *mut T,
    end_cap: *mut *mut T,
}

impl<T> SplitBuffer<T> {
    pub const fn block_size() -> usize {
        if std::mem::size_of::<T>() < 0x100 {
            0x1000 / std::mem::size_of::<T>()
        } else {
            0x10
        }
    }

    pub fn len(&self) -> usize {
        unsafe { self.end.offset_from(self.begin) as usize }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn capacity(&self) -> usize {
        unsafe { self.end_cap.offset_from(self.first) as usize }
    }

    pub fn get_buffer(&self, idx: usize) -> Option<&[T]> {
        if idx >= self.len() {
            None
        } else {
            Some(unsafe {
                std::slice::from_raw_parts(
                    *self.begin.add(idx),
                    Self::block_size()
                )
            })
        }
    }

    pub fn get_buffer_mut(&mut self, idx: usize) -> Option<&mut [T]> {
        if idx >= self.len() {
            None
        } else {
            Some(unsafe {
                std::slice::from_raw_parts_mut(
                    *self.begin.add(idx),
                    Self::block_size()
                )
            })
        }
    }
}

#[repr(C)]
pub struct Deque<T> {
    buffer: SplitBuffer<T>,
    start_index: usize,
    length: usize
}

impl<T> Deque<T> {
    const fn split_index(index: usize) -> (usize, usize) {
        let block_size = SplitBuffer::<T>::block_size();
        (index / block_size, index % block_size)
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.length {
            None
        } else {
            let (block, item) = Self::split_index(index);
            self.buffer
                .get_buffer(block)
                .and_then(|buffer| buffer.get(item))
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.length {
            None
        } else {
            let (block, item) = Self::split_index(index);
            self.buffer
                .get_buffer_mut(block)
                .and_then(|buffer| buffer.get_mut(item))
        }
    }
}

struct DequeIterator<'a, T> {
    deque: &'a Deque<T>,
    current: usize,
}

struct DequeIteratorMut<'a, T> {
    deque: &'a mut Deque<T>,
    current: usize
}

impl<'a, T> Iterator for DequeIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let output = self.deque.get(self.current);
        self.current += 1;
        output
    }
}

impl<'a, T> Iterator for DequeIteratorMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let output = self.deque.get_mut(self.current);
        self.current += 1;
        unsafe { std::mem::transmute::<Option<&mut T>, Option<&'a mut T>>(output) }
    }
}

impl<T> Index<usize> for Deque<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<T> IndexMut<usize> for Deque<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}