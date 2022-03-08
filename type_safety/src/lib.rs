use std::marker::PhantomData;
use std::ops::{Index, IndexMut};

pub struct Sorted;
pub struct UnSorted;

#[derive(Clone)]
pub struct SafeVec<E, S> {
    raw_vec: Vec<E>,
    state: PhantomData<S>,
}

impl<E, S> SafeVec<E, S> {
    pub fn len(&self) -> usize {
        self.raw_vec.len()
    }
    pub fn pop(&mut self) -> Option<E> {
        self.raw_vec.pop()
    }
}

impl<E> SafeVec<E, UnSorted>
where
    E: PartialEq + PartialOrd + Ord + Eq,
{
    pub fn push(&mut self, elem: E) {
        self.raw_vec.push(elem);
    }
    pub fn sort(mut self) -> SafeVec<E, Sorted> {
        self.raw_vec.sort();
        SafeVec { raw_vec: self.raw_vec, state: PhantomData }
    }
}

impl<E> SafeVec<E, Sorted>
where
    E: PartialEq + PartialOrd + Ord + Eq,
{
    pub fn new() -> SafeVec<E, Sorted> {
        SafeVec { raw_vec: Vec::new(), state: PhantomData }
    }
    pub fn relaxed(self) -> SafeVec<E, UnSorted> {
        SafeVec { raw_vec: self.raw_vec, state: PhantomData }
    }
    pub fn sort_insert(&mut self, elem: E) {
        match self.raw_vec.binary_search(&elem) {
            Err(index) => {
                self.raw_vec.insert(index, elem);
            }
            Ok(index) => {
                self.raw_vec.insert(index + 1, elem);
            }
        }
    }
    pub fn search(&self, elem: &E) -> Option<usize> {
        self.raw_vec.binary_search(elem).ok()
    }
    pub fn merge(
        mut left: SafeVec<E, Sorted>,
        mut right: SafeVec<E, Sorted>,
    ) -> SafeVec<E, Sorted> {
        let mut result = Vec::new();
        loop {
            match (left.raw_vec.pop(), right.raw_vec.pop()) {
                (Some(v0), Some(v1)) if v0 > v1 => {
                    right.raw_vec.push(v1);
                    result.push(v0);
                }
                (Some(v0), Some(v1)) => {
                    left.raw_vec.push(v0);
                    result.push(v1);
                }
                (None, Some(v0)) => {
                    result.push(v0);
                }
                (Some(v0), None) => {
                    result.push(v0);
                }
                _ => break,
            }
        }
        result.reverse();
        SafeVec { raw_vec: result, state: PhantomData }
    }
}

impl<E, S> Index<usize> for SafeVec<E, S> {
    type Output = E;

    fn index(&self, index: usize) -> &Self::Output {
        &self.raw_vec[index]
    }
}

impl<E> IndexMut<usize> for SafeVec<E, UnSorted> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.raw_vec[index]
    }
}

#[cfg(test)]
mod tests {
    use crate::SafeVec;
    #[test]
    fn sort_insert() {
        let mut v0 = SafeVec::new();
        v0.sort_insert(1);
        v0.sort_insert(2);
        v0.sort_insert(3);
        assert_eq!(v0.raw_vec, vec![1, 2, 3]);
        let mut v0 = SafeVec::new();
        v0.sort_insert(1);
        v0.sort_insert(1);
        v0.sort_insert(1);
        assert_eq!(v0.raw_vec, vec![1, 1, 1]);
    }
    #[test]
    fn test_sort() {
        let mut v0 = SafeVec::new();
        let mut v1 = SafeVec::new();
        let mut sorted = Vec::new();
        for i in 0..10 {
            if i % 2 == 0 {
                v0.sort_insert(i);
            } else {
                v1.sort_insert(i);
            }
            sorted.push(i);
        }
        let result = SafeVec::merge(v0, v1);
        assert_eq!(result.raw_vec, sorted);
    }
}
