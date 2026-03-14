use std::ops::{
    Index,
    IndexMut
};

macro_rules! impl_array_methods {
    ($name:ident) => {
        impl<T> $name<T> {
            pub fn as_slice<'a>(&'a self) -> &'a [T] {
                unsafe {
                    std::slice::from_raw_parts(self.data, self.count)
                }
            }
        
            pub fn as_slice_mut<'a>(&'a mut self) -> &'a mut [T] {
                unsafe {
                    std::slice::from_raw_parts_mut(self.data, self.count)
                }
            }
        
            pub fn len(&self) -> usize {
                self.count
            }
        
            pub fn as_ptr(&self) -> *const T {
                self.data
            }
        
            pub fn as_mut_ptr(&self) -> *mut T {
                self.data
            }
        
            pub fn iter(&self) -> std::slice::Iter<'_, T> {
                self.as_slice().iter()
            }
        
            pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
                self.as_slice_mut().iter_mut()
            }
        
            pub fn get<'a>(&'a self, index: usize) -> Option<&'a T> {
                self.as_slice().get(index)
            }
        
            pub fn get_mut<'a>(&'a mut self, index: usize) -> Option<&'a mut T> {
                self.as_slice_mut().get_mut(index)
            }
        }
        
        impl<T> Index<usize> for $name<T> {
            type Output = T;
        
            fn index(&self, index: usize) -> &Self::Output {
                &self.as_slice()[index]
            }
        }
        
        impl<T> IndexMut<usize> for $name<T> {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                &mut self.as_slice_mut()[index]
            }
        }
    }
}

#[repr(C)]
pub struct Array<T> {
    data: *mut T,
    count: usize
}

#[repr(C)]
pub struct Array2<T> {
    count: usize,
    data: *mut T,
}

impl_array_methods!(Array);
impl_array_methods!(Array2);