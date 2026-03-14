use std::{ops::{
    Deref,
    DerefMut
}, marker::PhantomData};



#[repr(C)]
pub struct LinkedList<T> {
    length: usize,
    start: *mut LinkedListNode<T>,
    end: *mut LinkedListNode<T>
}

impl<T> LinkedList<T> {
    pub fn len(&self) -> usize {
        self.length
    }

    /// Only use this for comparisons during an iter, it is not safe to use elsewhere
    unsafe fn as_node(&self) -> *mut LinkedListNode<T> {
        &self.start as *const *mut LinkedListNode<T> as *mut LinkedListNode<T>
    }

    pub fn iter(&self) -> LinkedListIter<'_, T> {
        LinkedListIter { 
            term: unsafe {
                self.as_node()
            }, 
            current: unsafe {
                self.as_node()
            },
            phantom: PhantomData
        }
    }

    pub fn iter_mut(&mut self) -> LinkedListIterMut<'_, T> {
        LinkedListIterMut {
            term: unsafe {
                self.as_node()
            },
            current: unsafe {
                self.as_node()
            },
            phantom: PhantomData
        }
    }

    pub fn add_to_front(&mut self, data: T) {
        let next_node = Box::leak(Box::new(LinkedListNode {
            next: self.start,
            prev: unsafe {
                self.as_node()
            },
            data
        }));

        if self.length > 0 {
            unsafe {
                (*self.start).prev = next_node;
            }
        }
        self.start = next_node;
        self.length += 1;
    }

    pub fn add_to_back(&mut self, data: T) {
        let next_node = Box::leak(Box::new(LinkedListNode {
            next: unsafe {
                self.as_node()
            },
            prev: self.end,
            data
        }));

        if self.length > 0 {
            unsafe {
                (*self.end).next = next_node;
            }
        }
        self.end = next_node;
    }
}

#[repr(C)]
pub struct LinkedListNode<T> {
    next: *mut LinkedListNode<T>,
    prev: *mut LinkedListNode<T>,
    data: T,
}

impl<T> LinkedListNode<T> {
    pub fn next(&self) -> *mut LinkedListNode<T> {
        self.next
    }

    pub fn prev(&self) -> *mut LinkedListNode<T> {
        self.prev
    }
}

impl<T> Deref for LinkedListNode<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for LinkedListNode<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T> AsRef<T> for LinkedListNode<T> {
    fn as_ref(&self) -> &T {
        &self.data
    }
}

impl<T> AsMut<T> for LinkedListNode<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.data
    }
}

pub struct LinkedListIter<'a, T> {
    term: *mut LinkedListNode<T>,
    current: *mut LinkedListNode<T>,
    phantom: PhantomData<&'a T>
}

pub struct LinkedListIterMut<'a, T> {
    term: *mut LinkedListNode<T>,
    current: *mut LinkedListNode<T>,
    phantom: PhantomData<&'a mut T>
}

impl<'a, T> Iterator for LinkedListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if (*self.current).next == self.term {
                None
            } else {
                self.current = (*self.current).next;
                Some((*self.current).as_ref())
            }
        }
    }
}

impl<'a, T> Iterator for LinkedListIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if (*self.current).next == self.term {
                None
            } else {
                self.current = (*self.current).next;
                Some((*self.current).as_mut())
            }
        }
    }
}