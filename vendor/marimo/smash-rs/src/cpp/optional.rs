// Based on boost optional
#[repr(C)]
pub struct Optional<T> {
    pub tag: bool,
    pub item: T
}

impl<T> Optional<T> {
    pub fn into_option(self) -> Option<T> {
        let Self { tag, item } = self;
        if tag {
            Some(item)
        } else {
            None
        }
    }

    pub fn is(&self) -> bool {
        self.tag
    }
}

impl<T> Into<Option<T>> for Optional<T> {
    fn into(self) -> Option<T> {
        let Self { tag, item } = self;
        if tag {
            Some(item)
        } else {
            None
        }
    }
}

impl<T> From<Option<T>> for Optional<T> {
    fn from(other: Option<T>) -> Self {
        match other {
            Some(item) => Optional {
                tag: true,
                item
            },
            None => Optional {
                tag: false,
                item: unsafe { std::mem::zeroed() }
            }
        }
    }
}