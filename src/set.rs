use std::marker::PhantomData;

#[derive(Debug)]
pub struct Sparse<T> {
    dense: Vec<usize>,
    sparse: Vec<Option<usize>>,
    phantom: PhantomData<T>,
}

// TODO: consider forcing elem to implement SliceIndex instead of Into<usize>
impl<T: Into<usize>> Sparse<T> {
    pub fn new() -> Self {
        Self {
            dense: Vec::new(),
            sparse: Vec::new(),
            phantom: PhantomData,
        }
    }
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            dense: Vec::with_capacity(cap),
            sparse: Vec::with_capacity(cap),
            phantom: PhantomData,
        }
    }

    pub fn from(elems: Vec<T>) -> Self {
        let mut set = Self::with_capacity(elems.len());
        for elem in elems {
            set.insert(elem);
        }
        set
    }

    pub fn insert(&mut self, elem: T) -> bool {
        let idx: usize = elem.into();
        if self.contains_idx(idx) {
            // do nothing
            return true;
        }
        self.dense.push(idx);
        if idx >= self.sparse.len() {
            self.sparse.resize_with(idx, || None);
        }
        self.sparse.insert(idx, Some(self.dense.len() - 1usize));
        false
    }

    fn contains_idx(&self, idx: usize) -> bool {
        if idx >= self.sparse.len() {
            return false;
        }
        self.sparse[idx].is_some()
    }

    pub fn contains(&self, elem: T) -> bool {
        self.contains_idx(elem.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::set::Sparse;

    #[test]
    fn push_contains() {
        let mut set = Sparse::new();
        set.insert(1usize);
        set.insert(3usize);
        set.insert(133usize);

        assert!(set.contains(1));
        assert!(set.contains(3));
        assert!(set.contains(133));
        let cnts = set.contains(2);
        assert!(!set.contains(2));
        assert!(!set.contains(4));
    }

    #[test]
    fn remove() {}

    #[test]
    fn contains() {}

    #[test]
    fn iter() {}

    #[test]
    fn from() {}
}