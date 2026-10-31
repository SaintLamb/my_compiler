macro_rules! define_id {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name(u32);

        impl $name {
            pub fn new(v: u32) -> Self {
                $name(v)
            }
            pub fn raw(self) -> u32 {
                self.0
            }
        }

        impl Idx for $name {
            fn index(self) -> usize {
                self.0 as usize
            }
            fn from_usize(v: usize) -> Self {
                $name(v as u32)
            }
        }
    }
}

pub trait Idx: Copy {
    fn index(self) -> usize;
    fn from_usize(v: usize) -> Self;
} 

define_id!(VRegId);
define_id!(BlockId);
define_id!(InstrId);
define_id!(FrameId);
define_id!(PRegId);

pub struct IndexVec<I: Idx, T> {
    data: Vec<T>,
    _marker: std::marker::PhantomData<I>,
}

impl<I: Idx, T> IndexVec<I, T> {
    pub fn new() -> Self {
        IndexVec {
            data: Vec::new(),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn push(&mut self, val: T) -> I {
        let id = I::from_usize(self.data.len());
        self.data.push(val);
        id
    } 

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut()
    }

    pub fn indices(&self) -> impl Iterator<Item = I> {
        (0..self.data.len()).map(I::from_usize)
    }

    pub fn iter_enumerated(&self) -> impl Iterator<Item = (I, &T)> {
        self.data
            .iter()
            .enumerate()
            .map(|(i, v)| (I::from_usize(i), v))
    }

    pub fn iter_enumerated_mut(&mut self) -> impl Iterator<Item = (I, &mut T)> {
        self.data
            .iter_mut()
            .enumerate()
            .map(|(i, v)| (I::from_usize(i), v))
    }
}

impl<I: Idx, T: Clone> IndexVec<I, T> {
    pub fn with_len(len: usize, value: T) -> Self {
        IndexVec {
            data: vec![value; len],
            _marker: std::marker::PhantomData,
        }
    }
}

impl<I: Idx, T> Default for IndexVec<I, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<I: Idx, T> std::ops::Index<I> for IndexVec<I, T> {
    type Output = T;
    fn index(&self, idx: I) -> &T {
        &self.data[idx.index()]
    }
}

impl<I: Idx, T> std::ops::IndexMut<I> for IndexVec<I, T> {
    fn index_mut(&mut self, idx: I) -> &mut T {
        &mut self.data[idx.index()]
    }
}

impl<I: Idx, T: Clone> Clone for IndexVec<I, T> {
    fn clone(&self) -> Self {
        IndexVec {
            data: self.data.clone(),
            _marker: std::marker::PhantomData,
        }
    }
}

impl<I: Idx, T: std::fmt::Debug> std::fmt::Debug for IndexVec<I, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(&self.data).finish()
    }
}