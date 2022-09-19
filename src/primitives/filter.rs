extern crate roaring;
use roaring::RoaringBitmap;

#[derive(Debug, PartialEq)]
pub struct Filter {
    bitmap: RoaringBitmap,
}

impl Filter {
    pub fn new() -> Filter {
        Filter {
            bitmap: RoaringBitmap::new(),
        }
    }

    pub fn from_bitmap(bitmap: RoaringBitmap) -> Filter {
        Filter { bitmap: bitmap }
    }

    pub fn insert(&mut self, id: u32) {
        self.bitmap.insert(id);
    }

    pub fn insert_many(&mut self, ids: &[u32]) {
        for id in ids {
            self.bitmap.insert(*id);
        }
    }

    pub fn and(&self, other: &Filter) -> Filter {
        Filter::from_bitmap(self.bitmap.clone() & other.bitmap.clone())
    }

    pub fn or(&self, other: &Filter) -> Filter {
        Filter::from_bitmap(self.bitmap.clone() | other.bitmap.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::Filter;

    #[test]
    fn insert_many() {
        let ids: [u32; 3] = [1, 2, 3];
        let mut filter: Filter = Filter::new();
        filter.insert_many(&ids);

        let mut expected: Filter = Filter::new();
        expected.insert(1);
        expected.insert(2);
        expected.insert(3);

        assert_eq!(filter, expected)
    }

    #[test]
    fn union() {
        let mut filter1 = Filter::new();
        let mut filter2 = Filter::new();

        filter1.insert_many(&[1, 12]);
        filter2.insert_many(&[5, 12]);

        filter1 = filter1.and(&filter2);

        let mut expected = Filter::new();
        expected.insert(12);

        assert_eq!(filter1, expected)
    }
}
