use std::ops::{Index, IndexMut};

/// A one-dimensional generic buffer (struct) of type `T`
pub struct Buffer<T> {
    pub data: Vec<T>,
}

impl<T> Buffer<T> {
    pub fn new(data: Vec<T>) -> Self {
        Buffer { data }
    }
}

impl<T> Index<usize> for Buffer<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for Buffer<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

/// A two-dimensional generic buffer (struct) of type `T`
pub struct Buffer2D<T> {
    pub data: Vec<Vec<T>>,
}

impl<T> Buffer2D<T> {
    pub fn new(data: Vec<Vec<T>>) -> Self {
        Buffer2D { data }
    }
}

impl<T> Index<(usize, usize)> for Buffer2D<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0][index.1]
    }
}

impl<T> IndexMut<(usize, usize)> for Buffer2D<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0][index.1]
    }
}
