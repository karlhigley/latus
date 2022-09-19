#[derive(Debug, PartialEq)]
pub struct PostingList {
    pub ids: Vec<u32>,
}

impl PostingList {
    pub fn new() -> PostingList {
        PostingList {
            ids: Vec::<u32>::new(),
        }
    }

    pub fn insert(&mut self, id: u32) {
        self.ids.extend([id].iter())
    }

    pub fn insert_many(&mut self, ids: &[u32]) {
        self.ids.extend(ids.iter())
    }
}

#[cfg(test)]
mod tests {
    use super::PostingList;

    #[test]
    fn insert_many() {
        let ids: [u32; 3] = [1, 2, 3];
        let mut postings: PostingList = PostingList::new();
        postings.insert_many(&ids);

        let mut expected: PostingList = PostingList::new();
        expected.insert(1);
        expected.insert(2);
        expected.insert(3);

        assert_eq!(postings, expected)
    }
}
