use crate::phx::Hash40;
use std::{
    fmt::Debug,
    ops::{Index, IndexMut},
};

#[repr(C)]
pub struct MapEntry<K, V> {
    next: *mut MapEntry<K, V>,
    hash: u64,
    key: K,
    value: V,
}

impl<K, V> MapEntry<K, V> {
    pub fn key(&self) -> &K {
        &self.key
    }

    pub fn raw_hash(&self) -> u64 {
        self.hash
    }

    pub fn value(&self) -> &V {
        &self.value
    }

    pub fn value_mut(&mut self) -> &mut V {
        &mut self.value
    }

    pub fn next(&self) -> Option<&Self> {
        unsafe { self.next.as_ref() }
    }

    pub fn next_mut(&mut self) -> Option<&mut Self> {
        unsafe { self.next.as_mut() }
    }
}

#[repr(C)]
pub struct HashMap<K, V> {
    buckets: *const *mut MapEntry<K, V>,
    bucket_count: usize,
    entries: *mut MapEntry<K, V>,
    entry_count: usize,
    prime_coefficient: f32, // no clue if that's the right name or not, but it's 1.0f
}

impl<K, V> HashMap<K, V> {
    fn buckets(&self) -> Option<&[*mut MapEntry<K, V>]> {
        if self.buckets.is_null() || self.bucket_count == 0 {
            None
        } else {
            unsafe { Some(std::slice::from_raw_parts(self.buckets, self.bucket_count)) }
        }
    }

    fn entries(&self) -> Option<&[MapEntry<K, V>]> {
        if self.entries.is_null() || self.entry_count == 0 {
            None
        } else {
            unsafe { Some(std::slice::from_raw_parts(self.entries, self.entry_count)) }
        }
    }

    fn entries_mut(&mut self) -> Option<&mut [MapEntry<K, V>]> {
        if self.entries.is_null() || self.entry_count == 0 {
            None
        } else {
            unsafe {
                Some(std::slice::from_raw_parts_mut(
                    self.entries,
                    self.entry_count,
                ))
            }
        }
    }

    pub fn iter(&self) -> KeyValueIter<K, V> {
        KeyValueIter::new(self)
    }

    pub fn iter_mut(&mut self) -> KeyValueIterMut<K, V> {
        KeyValueIterMut::new(self)
    }

    pub fn keys(&self) -> KeyIter<K, V> {
        KeyIter(KeyValueIter::new(self))
    }

    pub fn values(&self) -> ValueIter<K, V> {
        ValueIter(KeyValueIter::new(self))
    }

    pub fn values_mut(&mut self) -> ValueIterMut<K, V> {
        ValueIterMut(KeyValueIterMut::new(self))
    }

    pub fn get(&self, key: &K) -> Option<&V>
    where
        K: PartialEq,
    {
        self.entries()
            .map(|entries| {
                for entry in entries {
                    if entry.key() == key {
                        return Some(entry.value());
                    }
                }
                None
            })
            .flatten()
    }

    pub fn get_hash(&self, hash: u64) -> Option<&V> {
        match self.buckets() {
            Some(buckets) => {
                let bucket_idx = (hash as usize) % buckets.len();
                let mut current = if let Some(start) = buckets.get(bucket_idx) {
                    unsafe { Some(&**start) }
                } else {
                    return None;
                };

                while let Some(current_entry) = current {
                    if current_entry.raw_hash() == hash {
                        return Some(current_entry.value());
                    }
                    let current_bucket_idx = (current_entry.raw_hash() as usize) % buckets.len();
                    if current_bucket_idx != bucket_idx {
                        break;
                    }
                    current = current_entry.next();
                }
                None
            }
            None => None,
        }
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&V>
    where
        K: PartialEq,
    {
        self.entries_mut()
            .map(|entries| {
                for entry in entries {
                    if entry.key() == key {
                        return Some(entry.value());
                    }
                }
                None
            })
            .flatten()
    }

    pub fn get_hash_mut(&mut self, hash: u64) -> Option<&mut V> {
        match self.buckets() {
            Some(buckets) => {
                let bucket_idx = (hash as usize) % buckets.len();
                let mut current = if let Some(start) = buckets.get(bucket_idx) {
                    unsafe { Some(&mut **start) }
                } else {
                    return None;
                };

                while let Some(current_entry) = current {
                    if current_entry.raw_hash() == hash {
                        return Some(current_entry.value_mut());
                    }
                    let current_bucket_idx = (current_entry.raw_hash() as usize) % buckets.len();
                    if current_bucket_idx != bucket_idx {
                        return None;
                    }
                    current = current_entry.next_mut();
                }
                None
            }
            None => None,
        }
    }
}

pub struct KeyValueIter<'a, K, V> {
    map: &'a HashMap<K, V>,
    current_entry: *const MapEntry<K, V>,
}

#[repr(transparent)]
pub struct KeyIter<'a, K, V>(KeyValueIter<'a, K, V>);

#[repr(transparent)]
pub struct ValueIter<'a, K, V>(KeyValueIter<'a, K, V>);

impl<'a, K, V> KeyValueIter<'a, K, V> {
    pub(crate) fn new(map: &'a HashMap<K, V>) -> Self {
        Self {
            map,
            current_entry: std::ptr::null(),
        }
    }
}

impl<'a, K, V> Iterator for KeyValueIter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_entry.is_null() {
            if self.map.entries.is_null() {
                None
            } else {
                self.current_entry = self.map.entries as *const MapEntry<K, V>;
                unsafe { Some(((*self.current_entry).key(), (*self.current_entry).value())) }
            }
        } else {
            let next = unsafe { (*self.current_entry).next() };

            match next {
                Some(entry) => {
                    self.current_entry = entry as *const MapEntry<K, V>;
                    Some((entry.key(), entry.value()))
                }
                None => None,
            }
        }
    }
}

impl<'a, K, V> Iterator for KeyIter<'a, K, V> {
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(key, _)| key)
    }
}

impl<'a, K, V> Iterator for ValueIter<'a, K, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(_, value)| value)
    }
}

pub struct KeyValueIterMut<'a, K, V> {
    map: &'a mut HashMap<K, V>,
    current_entry: *mut MapEntry<K, V>,
}

#[repr(transparent)]
pub struct ValueIterMut<'a, K, V>(KeyValueIterMut<'a, K, V>);

impl<'a, K, V> KeyValueIterMut<'a, K, V> {
    pub(crate) fn new(map: &'a mut HashMap<K, V>) -> Self {
        Self {
            map,
            current_entry: std::ptr::null_mut(),
        }
    }
}

impl<'a, K, V> Iterator for KeyValueIterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_entry.is_null() {
            if self.map.entries.is_null() {
                None
            } else {
                self.current_entry = self.map.entries;
                unsafe {
                    Some((
                        (*self.current_entry).key(),
                        (*self.current_entry).value_mut(),
                    ))
                }
            }
        } else {
            let next = unsafe { (*self.current_entry).next_mut() };

            match next {
                Some(entry) => {
                    self.current_entry = entry;
                    Some((&entry.key, &mut entry.value))
                }
                None => None,
            }
        }
    }
}

impl<'a, K, V> Iterator for ValueIterMut<'a, K, V> {
    type Item = &'a mut V;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(_, value)| value)
    }
}

impl<H: Into<Hash40>, V> Index<H> for HashMap<Hash40, V> {
    type Output = V;

    fn index(&self, index: H) -> &Self::Output {
        let hash = index.into();
        self.get_hash(hash.0).expect("Could not index HashMap!")
    }
}

impl<H: Into<Hash40>, V> IndexMut<H> for HashMap<Hash40, V> {
    fn index_mut(&mut self, index: H) -> &mut Self::Output {
        let hash = index.into();
        self.get_hash_mut(hash.0).expect("Could not index HashMap!")
    }
}

impl<K: Debug, V: Debug> Debug for HashMap<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}
