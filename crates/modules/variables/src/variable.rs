pub trait VariableAccessor {
    fn write_flag(&mut self, index: usize, flag: bool);
    fn write_int(&mut self, index: usize, int: i32);
    fn write_uint(&mut self, index: usize, uint: u32);
    fn write_long(&mut self, index: usize, long: i64);
    fn write_ulong(&mut self, index: usize, ulong: u64);
    fn write_float(&mut self, index: usize, float: f32);

    fn read_flag(&self, index: usize) -> bool;
    fn read_int(&self, index: usize) -> i32;
    fn read_uint(&self, index: usize) -> u32;
    fn read_long(&self, index: usize) -> i64;
    fn read_ulong(&self, index: usize) -> u64;
    fn read_float(&self, index: usize) -> f32;
}

pub trait VariableId: Copy + Clone + PartialEq + PartialOrd + Eq + Ord + std::fmt::Debug {
    const MAX_INDEX: VariableIndex;

    fn write<V: Variable>(self, variable: V, status: i32, accessor: &mut dyn VariableAccessor);
    fn read<V: Variable>(self, status: i32, accessor: &dyn VariableAccessor) -> V;
    fn clear_status(status: i32, accessor: &mut dyn VariableAccessor);
}

pub trait Variable: 'static {
    const LENGTH: VariableIndex;
    fn write(self, index: VariableIndex, accessor: &mut dyn VariableAccessor);
    fn read(index: VariableIndex, accessor: &dyn VariableAccessor) -> Self;
}

#[derive(Copy, Clone, Debug)]
pub struct VariableIndex {
    pub flag: usize,
    pub word: usize,
    pub dword: usize,
    pub float: usize,
}

impl const std::ops::Add for VariableIndex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            flag: self.flag + rhs.flag,
            word: self.word + rhs.word,
            dword: self.dword + rhs.dword,
            float: self.float + rhs.float,
        }
    }
}

macro_rules! prim {
    ($(($t:ty, $write_method:ident, $read_method:ident, $kind:ident, $flag:expr, $word:expr, $dword:expr, $float:expr)),*) => {
        $(
            impl Variable for $t {
                const LENGTH: VariableIndex = VariableIndex {
                    flag: $flag,
                    word: $word,
                    dword: $dword,
                    float: $float
                };

                fn write(self, index: VariableIndex, accessor: &mut dyn VariableAccessor) {
                    accessor.$write_method(index.$kind, self);
                }

                fn read(index: VariableIndex, accessor: &dyn VariableAccessor) -> Self {
                    accessor.$read_method(index.$kind)
                }
            }
        )*
    }
}

prim! {
    (bool, write_flag, read_flag, flag, 1, 0, 0, 0),
    (i32, write_int, read_int, word, 0, 1, 0, 0),
    (u32, write_uint, read_uint, word, 0, 1, 0, 0),
    (i64, write_long, read_long, dword, 0, 0, 1, 0),
    (u64, write_ulong, read_ulong, dword, 0, 0, 1, 0),
    (f32, write_float, read_float, float, 0, 0, 0, 1)
}

impl Variable for smash::phx::Vector2f {
    const LENGTH: VariableIndex = VariableIndex {
        flag: 0,
        word: 0,
        dword: 0,
        float: 2,
    };

    fn write(self, index: VariableIndex, accessor: &mut dyn VariableAccessor) {
        accessor.write_float(index.float, self.x);
        accessor.write_float(index.float + 1, self.y);
    }

    fn read(index: VariableIndex, accessor: &dyn VariableAccessor) -> Self {
        Self {
            x: accessor.read_float(index.float),
            y: accessor.read_float(index.float + 1),
        }
    }
}

impl Variable for smash::phx::Vector3f {
    const LENGTH: VariableIndex = VariableIndex {
        flag: 0,
        word: 0,
        dword: 0,
        float: 3,
    };

    fn write(self, index: VariableIndex, accessor: &mut dyn VariableAccessor) {
        accessor.write_float(index.float, self.x);
        accessor.write_float(index.float + 1, self.y);
        accessor.write_float(index.float + 2, self.z);
    }

    fn read(index: VariableIndex, accessor: &dyn VariableAccessor) -> Self {
        Self {
            x: accessor.read_float(index.float),
            y: accessor.read_float(index.float + 1),
            z: accessor.read_float(index.float + 2),
        }
    }
}

impl Variable for smash::phx::Vector4f {
    const LENGTH: VariableIndex = VariableIndex {
        flag: 0,
        word: 0,
        dword: 0,
        float: 4,
    };

    fn write(self, index: VariableIndex, accessor: &mut dyn VariableAccessor) {
        accessor.write_float(index.float, self.x);
        accessor.write_float(index.float + 1, self.y);
        accessor.write_float(index.float + 2, self.z);
        accessor.write_float(index.float + 3, self.w);
    }

    fn read(index: VariableIndex, accessor: &dyn VariableAccessor) -> Self {
        Self {
            x: accessor.read_float(index.float),
            y: accessor.read_float(index.float + 1),
            z: accessor.read_float(index.float + 2),
            w: accessor.read_float(index.float + 3),
        }
    }
}

impl Variable for smash::phx::Hash40 {
    const LENGTH: VariableIndex = VariableIndex {
        flag: 0,
        word: 0,
        dword: 1,
        float: 0,
    };

    fn write(self, index: VariableIndex, accessor: &mut dyn VariableAccessor) {
        accessor.write_ulong(index.dword, self.hash);
    }

    fn read(index: VariableIndex, accessor: &dyn VariableAccessor) -> Self {
        Self::new_raw(accessor.read_ulong(index.dword))
    }
}
