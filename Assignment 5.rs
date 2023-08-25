use std::collections::HashMap;

// Define the SortByKey trait
trait SortByKey<K: Ord, V> {
    fn sort_by_key(&mut self);
}

// Implement the SortByKey trait for HashMap
impl<K: Ord + Clone + std::hash::Hash, V: Clone> SortByKey<K, V> for HashMap<K, V> {
    fn sort_by_key(&mut self) {
        let mut keys: Vec<_> = self.keys().cloned().collect();
        keys.sort();
        
        let mut sorted_map = HashMap::new();
        
        for key in &keys {
            if let Some(value) = self.get(key) {
                sorted_map.insert(key.clone(), value.clone());
            }
        }
        
        *self = sorted_map;
    }
}

fn main() {
    // Create a new HashMap instance
    let mut my_map: HashMap<i32, &str> = HashMap::new();

    // Add key-value pairs to the map
    my_map.insert(3, "apple");
    my_map.insert(1, "banana");
    my_map.insert(2, "cherry");

    println!("Original map: {:?}", my_map);

    // Sort the map by keys using the SortByKey trait
    my_map.sort_by_key();

    println!("Sorted map: {:?}", my_map);
}
