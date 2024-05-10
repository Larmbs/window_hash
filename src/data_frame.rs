use std::hash::BuildHasherDefault;
use std::collections::HashMap;
use twox_hash::XxHash64;
use std::fmt::Debug;
use super::DroppedValueConsumer;


pub struct DataFrame<T, C> where C: DroppedValueConsumer<T> {
    size: usize,
    index: usize,
    items: HashMap<usize, T, BuildHasherDefault<XxHash64>>,
    consumer: C,
}

impl<T, C> DataFrame<T, C> where C: DroppedValueConsumer<T> {
    pub fn new( size: usize, consumer: C ) -> Self {
        Self {
            size,
            index: 0,
            items: HashMap::with_capacity_and_hasher(size, BuildHasherDefault::<XxHash64>::default()),
            consumer,
        }
    }
    pub fn from_frame( items: Vec<T>, consumer: C ) -> Self {
        let size = items.len();
        Self {
            size,
            index: 0,
            items: HashMap::with_capacity_and_hasher(size, BuildHasherDefault::<XxHash64>::default()),
            consumer,
        }
    }
    pub fn len( &self ) -> usize {
        self.size
    }
    fn remove_overflow( &mut self ) {
        if self.index > self.size {
            let val = self.items.remove(&(self.index - self.size));
            self.consumer.consume(val.unwrap()); 
        }
    }
    pub fn get( &self, index: usize ) -> Option<&T> {
        self.items.get(&(self.index - self.size + index + 1))
    }
    pub fn get_slice( &self ) -> Vec<Option<&T>> {
        let mut res = vec![];
        for i in (self.index - self.size + 1)..=self.index {
            res.push(self.items.get(&i));
        }
        res
    }
    pub fn add( &mut self, item: T ) {
        self.index += 1;
        self.items.insert(self.index, item);
        self.remove_overflow();
    } 
}

impl<T, C> Debug for DataFrame<T, C> where C: DroppedValueConsumer<T> + Default  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DataFrame").field("size", &self.size).field("index", &self.index).finish()
    }
}

#[cfg(test)]
mod test {
    use super::DataFrame;
    use crate::DefaultConsumer;

    #[test]
    pub fn test_1() {
        let mut data_frame = DataFrame::new(3, DefaultConsumer::new());
        assert_eq!(data_frame.len(), 3);
        data_frame.add(5);
        data_frame.add(4);
        data_frame.add(3);

        assert_eq!(data_frame.get_slice(), vec![Some(&5), Some(&4), Some(&3)]);

        data_frame.add(3);

        assert_eq!(data_frame.get_slice(), vec![Some(&4), Some(&3), Some(&3)]);

        data_frame.add(20);

        assert_eq!(data_frame.get(0), Some(&3));        
        assert_eq!(data_frame.get(1), Some(&3));
        assert_eq!(data_frame.get(2), Some(&20));
    }
}
