
use std::hash::Hash;
use std::hash::Hasher;
use std::collections::hash_map::DefaultHasher;

trait BloomFilter {
    fn contains<T: Hash>(&self, item: &T) -> bool;
    fn insert<T: Hash>(&mut self, item: &T) -> Result<(), ()>;
}

pub struct SimpleFilter {
    database: Vec<bool>,
}

impl BloomFilter for SimpleFilter {

    fn contains<T: Hash>(&self, item: &T) -> bool {
        (0..5).map(|index| {
            let mut hasher = DefaultHasher::new();
            index.hash(&mut hasher);
            item.hash(&mut hasher);

            hasher.finish() as usize % self.database.len()
        }).all(|index| self.database[index])
    }

    fn insert<T: Hash>(&mut self, item: &T) -> Result<(), ()> {
        if self.contains(item) {
            Err(())
        } else {
            (0..5).map(|index| {
                let mut hasher = DefaultHasher::new();
                index.hash(&mut hasher);
                item.hash(&mut hasher);

                let index = hasher.finish() as usize % self.database.len();
                self.database[index] = true;
            }).count();

            Ok(())
        }
    }
}

impl SimpleFilter {

    pub fn new() -> Self {

        let mut v = vec![];
        v.resize(100, false);

        SimpleFilter {
            database: v,
        }
    }

}

#[cfg(test)]
mod test {

    use SimpleFilter;
    use BloomFilter;

    #[test]
    fn test() {
        let mut bf = SimpleFilter::new();
        assert!(!bf.contains(&0));

        bf.insert(&0);
        assert!(bf.contains(&0));
    }
}