use std::ops::{Index, IndexMut};

pub trait Idx: Copy + std::fmt::Debug + Eq + std::hash::Hash + 'static {
    fn index(self) -> usize;
    fn from_usize(v: usize) -> Self;
}

macro_rules! define_id {
    ($name: ident) => {
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

        impl $crate::id::Idx for $name {
            fn index(self) -> usize {
                self.0 as usize
            }
            fn from_usize(v: usize) -> Self {
                $name(u32::try_from(v).expect("id overflow"))
            }
        }
    };
}
pub(crate) use define_id;

pub struct IndexVec<I: Idx, T> {
    data: Vec<T>,
    _marker: std::marker::PhantomData<I>,
}

impl<I: Idx, T> IndexVec<I, T> {
    pub fn new() -> Self {
        IndexVec { data: Vec::new(), _marker: std::marker::PhantomData }
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
        self.data.iter().enumerate().map(|(i, v)| (I::from_usize(i), v))
    }
    pub fn iter_enumerated_mut(&mut self) -> impl Iterator<Item = (I, &mut T)> {
        self.data.iter_mut().enumerate().map(|(i, v)| (I::from_usize(i), v))
    }
    pub fn from_vec(data: Vec<T>) -> Self {
        IndexVec { data, _marker: std::marker::PhantomData }
    }
    pub fn into_vec(self) -> Vec<T> {
        self.data
    }
}

impl<I: Idx, T: Clone> IndexVec<I, T> {
    pub fn with_len(len: usize, value: T) -> Self {
        IndexVec { data: vec![value; len], _marker: std::marker::PhantomData }
    }
}

impl<I: Idx, T> Default for IndexVec<I, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<I: Idx, T: Clone> Clone for IndexVec<I, T> {
    fn clone(&self) -> Self {
        IndexVec { data: self.data.clone(), _marker: std::marker::PhantomData }
    }
}

impl<I: Idx, T: std::fmt::Debug> std::fmt::Debug for IndexVec<I, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(&self.data).finish()
    }
}

impl<I: Idx, T> Index<I> for IndexVec<I, T> {
    type Output = T;
    fn index(&self, idx: I) -> &T {
        &self.data[idx.index()]
    }
}

impl<I: Idx, T> IndexMut<I> for IndexVec<I, T> {
    fn index_mut(&mut self, idx: I) -> &mut T {
        &mut self.data[idx.index()]
    }
}