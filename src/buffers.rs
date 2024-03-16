use std::ops::{Index, IndexMut};

use crate::vertex::Vertex;

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

/// A one-dimensional semi-generic nested (vertex) buffer (struct) with `attributes` of type `A` for each `Vertex`
pub struct Vertices<A> {
    pub buffer: Buffer<Vertex<A>>,
}

impl<A> Vertices<A> {
    pub fn new(data: Vec<Vertex<A>>) -> Self {
        Vertices {
            buffer: Buffer { data },
        }
    }
}

impl<A> Index<usize> for Vertices<A> {
    type Output = Vertex<A>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.buffer[index]
    }
}

impl<A> IndexMut<usize> for Vertices<A> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.buffer[index]
    }
}

/// A one-dimensional non-generic (index) buffer (struct)
pub struct Indices {
    pub buffer: Buffer<i32>,
}

impl Indices {
    pub fn new(data: Vec<i32>) -> Self {
        Indices {
            buffer: Buffer { data },
        }
    }
}

impl Index<usize> for Indices {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.buffer[index]
    }
}

impl IndexMut<usize> for Indices {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.buffer[index]
    }
}
