use std::result::Result;

const HEAP_MAX_SIZE: usize = 1000;
pub struct Heap {
    array: [i32; HEAP_MAX_SIZE],
    size: usize,
    d: u32,
}

#[derive(Debug)]
pub enum HeapError {
    HeapFull,
    EmptyHeap,
    NoSuchParent,
    ParentReachedEnd,
    SonReachedEnd,
    InvalidSonIndex,
}

impl Heap {
    /// Creates a new d-ary max-heap from the given slice.
    ///
    /// # Arguments
    /// * `d` - Branching factor for the heap
    /// * `slice` - Initial values for the heap
    ///
    /// # Edge cases
    /// * If slice exceeds MAX_SIZE, only the first MAX_SIZE elements are used
    /// * Automatically builds a valid max-heap from the provided elements
    pub fn new(d: u32, slice: &[i32]) -> Self {
        let mut heap = Self {
            array: [-1; HEAP_MAX_SIZE],
            size: 0,
            d,
        };

        let slice_len = std::cmp::min(slice.len(), HEAP_MAX_SIZE);
        heap.array[..slice_len].copy_from_slice(&slice[..slice_len]);
        heap.size = slice_len;
        heap.build_heap();
        heap
    }

    /// Inserts a new item into the heap and maintains the max-heap property.
    ///
    /// # Edge cases
    /// * Returns HeapFull error if the heap has reached MAX_SIZE
    /// * Maintains max-heap property using heapify_up
    pub fn insert(&mut self, item: i32) -> std::result::Result<(), HeapError> {
        if self.size >= HEAP_MAX_SIZE {
            return Err(HeapError::HeapFull);
        }

        self.array[self.size] = item;
        self.size += 1;
        return self.heapify_up(self.size - 1);
    }

    /// Removes and returns the maximum element from the heap.
    ///
    /// # Edge cases
    /// * Returns EmptyHeap error if heap is empty
    pub fn extract_max(&mut self) -> Result<i32, HeapError> {
        if self.size == 0 {
            Err(HeapError::EmptyHeap)
        } else {
            let max = self.array[0];
            self.array[0] = self.array[self.size - 1];
            self.size -= 1;
            self.heapify_down(0)?;
            Ok(max)
        }
    }

    /// Changes the branching factor of the heap and rebuilds it.
    ///
    pub fn change_d(&mut self, d: u32) {
        self.d = d;
        self.build_heap();
    }

    /// Prints a visual representation of the heap by levels.
    pub fn print(&self) {
        println!("Heap (d={})", self.d);
        if self.size == 0 {
            println!("Empty heap :(");
            return;
        }

        let mut start = 0;
        let mut count = 1;
        let mut level = 0;

        while start < self.size {
            let end = std::cmp::min(self.size, start + count);
            print!("Level {}: ", level);
            for i in start..end {
                print!("{} ", self.array[i]);
            }
            println!();
            start = end;
            count *= self.d as usize;
            level += 1;
        }
    }

    /// Builds a max-heap from an unordered array by applying heapify_down
    /// on all the nodes that aren't leaves.
    fn build_heap(&mut self) {
        for i in (0..(self.size / 2)).rev() {
            self.heapify_down(i).unwrap();
        }
    }

    /// Restores max-heap property by moving element at given index down the heap.
    fn heapify_down(&mut self, idx: usize) -> Result<(), HeapError> {
        let mut largest_idx: usize = idx;
        let mut largest_val: i32 = self.array[idx];

        for n_son in 0..self.d {
            match self.get_n_son(idx, n_son) {
                Ok(son_idx) => {
                    if self.array[son_idx] > largest_val {
                        largest_idx = son_idx;
                        largest_val = self.array[son_idx];
                    }
                }
                Err(HeapError::SonReachedEnd) => {}
                Err(x) => return Err(x),
            }
        }

        if largest_idx != idx {
            // We found a son with a bigger value, then exchange, bringing son up
            self.array[largest_idx] = self.array[idx];
            self.array[idx] = largest_val;
            return self.heapify_down(largest_idx);
        }

        Ok(())
    }

    /// Restores max-heap property by moving element at given index up the heap.
    fn heapify_up(&mut self, idx: usize) -> Result<(), HeapError> {
        let mut smallest_idx: usize = idx;
        let mut smallest_val: i32 = self.array[idx];

        match self.get_parent(idx) {
            Ok(parent_idx) => {
                if self.array[parent_idx] < smallest_val {
                    smallest_idx = parent_idx;
                    smallest_val = self.array[parent_idx];
                }
            }
            Err(HeapError::ParentReachedEnd) => return Ok(()),
            Err(x) => return Err(x),
        }

        if smallest_idx != idx {
            // we found a parent with a smaller value, then exchange bringing parent down
            self.array[smallest_idx] = self.array[idx];
            self.array[idx] = smallest_val;
            return self.heapify_up(smallest_idx);
        }

        Ok(())
    }

    /// Gets the parent index of a given node.
    ///
    /// # Edge cases
    /// * Returns ParentReachedEnd error for root node (index 0)
    /// * Returns NoSuchParent error if calculated parent is outside valid range
    fn get_parent(&self, idx: usize) -> Result<usize, HeapError> {
        if idx == 0 {
            return Err(HeapError::ParentReachedEnd);
        }
        let parent_idx = (idx - 1) / self.d as usize;
        if parent_idx < self.size {
            Ok(parent_idx)
        } else {
            Err(HeapError::NoSuchParent)
        }
    }

    /// Gets the nth son index of a given node.
    ///
    /// # Edge cases
    /// * Returns InvalidSonIndex error if n >= d (invalid son number)
    /// * Returns SonReachedEnd error if calculated son index is outside heap bounds
    fn get_n_son(&self, idx: usize, n: u32) -> Result<usize, HeapError> {
        if n >= self.d {
            return Err(HeapError::InvalidSonIndex);
        }

        let son_idx = idx * (self.d as usize) + (n as usize) + 1;
        if son_idx < self.size {
            Ok(son_idx)
        } else {
            Err(HeapError::SonReachedEnd)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heap_creation() {
        let heap = Heap::new(2, &[3, 1, 4, 1, 5, 9]);
        assert_eq!(heap.size, 6);
    }

    #[test]
    fn test_insert() {
        let mut heap = Heap::new(2, &[]);
        assert!(heap.insert(10).is_ok());
        assert!(heap.insert(20).is_ok());
        assert_eq!(heap.size, 2);
        assert_eq!(heap.array[0], 20); // Max-heap property
    }

    #[test]
    fn test_heapify_up() {
        let mut heap = Heap::new(2, &[]);
        assert!(heap.insert(10).is_ok());
        assert!(heap.insert(20).is_ok());
        assert!(heap.insert(5).is_ok());
        assert_eq!(heap.array[0], 20); // Max-heap property
    }

    #[test]
    fn test_heapify_down() {
        let mut heap = Heap::new(2, &[20, 10, 5]);
        heap.array[0] = 1; // Break max-heap property
        let res = heap.heapify_down(0);
        assert!(res.is_ok());
        assert_eq!(heap.array[0], 10); // Max-heap property restored
    }
    #[test]
    fn test_extract_max() {
        let mut heap = Heap::new(2, &[3, 1, 4, 1, 5, 9]);

        // Extract max and verify it's correct
        let max_res = heap.extract_max();
        assert!(max_res.is_ok());
        assert_eq!(max_res.unwrap(), 9);

        // Verify the size decreased
        assert_eq!(heap.size, 5);

        // Verify heap property is maintained
        for i in 0..heap.size {
            if let Ok(left) = heap.get_n_son(i, 0) {
                assert!(heap.array[i] >= heap.array[left]);
            }
            if let Ok(right) = heap.get_n_son(i, 1) {
                assert!(heap.array[i] >= heap.array[right]);
            }
        }

        // Extract the next max and verify
        let second_max = heap.extract_max();
        assert!(second_max.is_ok());
        assert_eq!(second_max.unwrap(), 5);
    }

    #[test]
    fn test_get_parent() {
        let heap = Heap::new(2, &[3, 1, 4, 1, 5, 9]);
        assert!(heap.get_parent(1).is_ok());
        assert_eq!(heap.get_parent(1).unwrap_or(999), 0);
        assert!(heap.get_parent(2).is_ok());
        assert_eq!(heap.get_parent(2).unwrap_or(999), 0);
        assert!(heap.get_parent(0).is_err());
    }

    #[test]
    fn test_get_n_son() {
        let heap = Heap::new(2, &[3, 1, 4, 1, 5, 9, 10, 12]);
        assert!(heap.get_n_son(0, 0).is_ok());
        assert_eq!(heap.get_n_son(0, 0).unwrap_or(999), 1);
        assert!(heap.get_n_son(0, 1).is_ok());
        assert_eq!(heap.get_n_son(0, 1).unwrap_or(999), 2);
        assert!(heap.get_n_son(0, 2).is_err());

        // Test for a node at a deeper level
        assert!(heap.get_n_son(1, 0).is_ok());
        assert_eq!(heap.get_n_son(1, 0).unwrap_or(999), 3);
        assert!(heap.get_n_son(1, 1).is_ok());
        assert_eq!(heap.get_n_son(1, 1).unwrap_or(999), 4);
        assert!(heap.get_n_son(2, 0).is_ok());
        assert_eq!(heap.get_n_son(2, 0).unwrap_or(999), 5);
        assert!(heap.get_n_son(2, 1).is_ok());
        assert_eq!(heap.get_n_son(2, 1).unwrap_or(999), 6);
    }
}
