use super::posting_list::PostingList;

use std::collections::HashMap;

#[derive(Debug)]
pub struct InvertedIndex {
    postings: HashMap<u32, PostingList>,
}

impl InvertedIndex {
    pub fn new() -> InvertedIndex {
        InvertedIndex {
            postings: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: u32, value: u32) {
        if !self.postings.contains_key(&key) {
            self.postings.insert(key, PostingList::new());
        }
        let postings = self.postings.get_mut(&key);
        postings.unwrap().insert(value);
    }

    pub fn insert_many(&mut self, key: u32, value: &[u32]) {
        if !self.postings.contains_key(&key) {
            self.postings.insert(key, PostingList::new());
        }
        let postings = self.postings.get_mut(&key);
        postings.unwrap().insert_many(value);
    }
}

#[cfg(test)]
mod tests {
    use super::InvertedIndex;

    #[test]
    fn insert_many() {
        let key: u32 = 12;
        let ids: [u32; 3] = [1, 2, 3];

        let mut index: InvertedIndex = InvertedIndex::new();

        index.insert_many(key, &ids);

        let posting = index.postings.get(&key).unwrap();

        assert_eq!(posting.ids, ids)
    }
}
