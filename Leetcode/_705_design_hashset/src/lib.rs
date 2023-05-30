use std::collections::BTreeSet;

// This HashSet uses dynamic memory and is fully
// reallocated when the capacity of all the buckets
// has been reached
#[derive(Default)]
pub struct MyHashSet {
    buckets: Vec<Vec<i32>>,
    nb_buckets: usize,
    bucket_size: usize,
    nb_stored_elements: usize,
}

impl MyHashSet {
    pub fn new() -> Self {
        MyHashSet::allocate(1000, 10)
    }

    // O(bucket_size)
    pub fn add(&mut self, key: i32) {
        if self.nb_stored_elements + 1 >= self.nb_buckets * self.bucket_size {
            *self = self.reallocate();
        }

        let bucket: &mut Vec<i32> = &mut self.buckets[key as usize % self.nb_buckets];
        if !bucket.contains(&key) {
            bucket.push(key);
            self.nb_stored_elements += 1;
        }
    }

    // O(bucket_size)
    pub fn remove(&mut self, key: i32) {
        if let Some(index) = self.find_element(key) {
            self.buckets[key as usize % self.nb_buckets].swap_remove(index);
            self.nb_stored_elements -= 1;
        }
    }

    // O(bucket_size)
    pub fn contains(&self, key: i32) -> bool {
        self.find_element(key).is_some()
    }

    fn allocate(nb_buckets: usize, bucket_size: usize) -> Self {
        let mut hashset: MyHashSet = MyHashSet {
            buckets: Vec::new(),
            nb_buckets,
            bucket_size,
            nb_stored_elements: 0,
        };
        hashset.buckets.reserve(hashset.nb_buckets);
        for _ in 0..hashset.nb_buckets {
            let mut bucket: Vec<i32> = Vec::new();
            bucket.reserve(hashset.bucket_size);
            hashset.buckets.push(bucket);
        }
        hashset
    }

    // O(nb_stored_elements * bucket_size)
    fn reallocate(&self) -> Self {
        let mut hashset: MyHashSet = MyHashSet::allocate(self.nb_buckets * 10, self.bucket_size);

        for bucket in &self.buckets {
            for key in bucket {
                hashset.add(*key);
            }
        }
        hashset
    }

    // O(bucket_size)
    pub fn find_element(&self, key: i32) -> Option<usize> {
        let bucket: &Vec<i32> = &self.buckets[key as usize % self.nb_buckets];
        (0..bucket.len()).find(|&i| bucket[i] == key)
    }
}

#[derive(Default)]
pub struct MyHashSetBtree {
    buckets: Vec<Option<BTreeSet<i32>>>,
}

// Prime number of buckets to reduce collisions
const N_BUCKETS: usize = 1031;

impl MyHashSetBtree {

    pub fn new() -> Self {
        MyHashSetBtree{ buckets: vec![None; N_BUCKETS] }
    }

    pub fn hash(key: i32) -> usize {
        (key as usize) % N_BUCKETS
    }
    
    pub fn add(&mut self, key: i32) {
        let h: usize = Self::hash(key);
        if let Some(bucket) = self.buckets[h].as_mut() {
            bucket.insert(key);
        } else {
            let mut set: BTreeSet<i32> = BTreeSet::new();
            set.insert(key);
            self.buckets[h] = Some(set);
        }
    }
    
    pub fn remove(&mut self, key: i32) {
        let h = Self::hash(key);
        if let Some(bucket) = self.buckets[h].as_mut() {
            bucket.remove(&key);
            if bucket.is_empty() {
                //drop(bucket);
                self.buckets[h] = None;
            }   
        }
    }
    
    pub fn contains(&self, key: i32) -> bool {
        if let Some(bucket) = self.buckets[Self::hash(key)].as_ref() {
            bucket.contains(&key)
        } else {
            false
        }
    }
}