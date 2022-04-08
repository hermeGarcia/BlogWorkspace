use std::marker::PhantomData;
use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct ArenaID<T> {
    index: usize,
    of_type: PhantomData<T>,
}
impl<T> Copy for ArenaID<T> {}
impl<T> Eq for ArenaID<T> {}
impl<T> Ord for ArenaID<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl<T> PartialEq for ArenaID<T> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}
impl<T> PartialOrd for ArenaID<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.index.cmp(&other.index))
    }
}
impl<T> Clone for ArenaID<T> {
    fn clone(&self) -> Self {
        ArenaID {
            index: self.index,
            of_type: self.of_type,
        }
    }
}
impl<T> std::hash::Hash for ArenaID<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.index.hash(state);
    }
}
impl<T> From<usize> for ArenaID<T> {
    fn from(elem: usize) -> ArenaID<T> {
        ArenaID {
            index: elem,
            of_type: PhantomData,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Arena<T> {
    raw: Vec<T>,
}

impl<T> Default for Arena<T> {
    fn default() -> Self {
        Arena { raw: Vec::new() }
    }
}

impl<T> Index<ArenaID<T>> for Arena<T> {
    type Output = T;
    fn index(&self, idx: ArenaID<T>) -> &Self::Output {
        self.get(idx).unwrap()
    }
}
impl<T> IndexMut<ArenaID<T>> for Arena<T> {
    fn index_mut(&mut self, idx: ArenaID<T>) -> &mut Self::Output {
        self.get_mut(idx).unwrap()
    }
}
impl<T> Arena<T> {
    pub fn new() -> Arena<T> {
        Arena::default()
    }
    pub fn store(&mut self, elem: T) -> ArenaID<T> {
        let index = self.raw.len();
        self.raw.push(elem);
        ArenaID::from(index)
    }
    pub fn get(&self, at: ArenaID<T>) -> Option<&T> {
        self.raw.get(at.index)
    }
    pub fn get_mut(&mut self, at: ArenaID<T>) -> Option<&mut T> {
        self.raw.get_mut(at.index)
    }
    pub fn no_elems(&self) -> usize {
        self.raw.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn arena_test() {
        let mut arena = Arena::new();
        let id1 = arena.store("something");
        let id2 = arena.store("something else");
        assert_eq!(arena[id1], "something");
        assert_eq!(arena[id2], "something else");
        arena[id1] = "something different";
        assert_eq!(arena[id1], "something different");
    }
}
