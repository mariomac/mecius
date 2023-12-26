use std::marker::PhantomData;


#[derive(Debug)]
pub struct Sparse<T> {
    dense: Vec<Entry<T>>,
    sparse: Vec<Option<usize>>,
}

#[derive(Debug)]
struct Entry<T> {
    key: usize,
    value: T,
}

impl<T> Sparse<T> where Entry<T>: From<T> {
    pub fn new() -> Self {
        Self {
            dense: Vec::new(),
            sparse: Vec::new(),
        }
    }
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            dense: Vec::with_capacity(cap),
            sparse: Vec::with_capacity(cap),
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
        let entry: Entry<T> = elem.into();
        if self.contains_idx(entry.key) {
            // do nothing
            return true;
        }
        if entry.key >= self.sparse.len() {
            self.sparse.resize_with(entry.key, || None);
        }
        self.sparse.insert(entry.key, Some(self.dense.len()));
        self.dense.push(entry);
        false
    }

    fn contains_idx(&self, idx: usize) -> bool {
        if idx >= self.sparse.len() {
            return false;
        }
        self.sparse[idx].is_some()
    }

    pub fn contains(&self, elem: T) -> bool {
        let entry: Entry<T> = elem.into();
        self.contains_idx(entry.key)
    }

    pub fn remove(&mut self, elem: T) -> bool {
        let rm_entry: Entry<T> = elem.into();
        let sparse_rm_entry = self.sparse.get(rm_entry.key);
        if let Some(rm_entry_dense_idx) = *sparse_rm_entry.unwrap() {
            // remove entry from sparse
            self.sparse[rm_entry.key] = None;

            // replace removed entry by last entry in dense
            let last_entry = self.dense.pop().unwrap();
            let last_entry_key = last_entry.key;
            self.dense[rm_entry_dense_idx] = last_entry;
            // repoint sparse entry for the moved element
            self.sparse[last_entry_key] = Some(rm_entry.key);
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::set::{Entry, Sparse};

    impl From<i32> for Entry<i32> {
        fn from(value: i32) -> Self {
            Entry {
                key: value as usize,
                value,
            }
        }
    }

    #[test]
    fn push_contains() {
        let mut set = Sparse::new();
        set.insert(1);
        set.insert(3);
        set.insert(133);

        assert!(set.contains(1));
        assert!(set.contains(3));
        assert!(set.contains(133));
        assert!(!set.contains(2));
        assert!(!set.contains(4));
    }

    #[test]
    fn remove() {
        let mut set = Sparse::new();
        set.insert(1);
        set.insert(3);
        set.insert(133);

        assert!(set.remove(3));
        assert!(!set.remove(4));
        assert!(!set.remove(3));

        assert!(set.contains(1));
        assert!(set.contains(133));
        assert!(!set.contains(2));
        assert!(!set.contains(3));
        assert!(!set.contains(4));
    }

    #[test]
    fn contains() {}

    #[test]
    fn iter() {}

    #[test]
    fn from() {}
}