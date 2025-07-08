use std::collections::HashSet;
use std::hash::Hash;

/// Extensions to the [`slice`] type.
pub trait SliceExt<T> {
    /// Check if the slice contains duplicate elements.
    ///
    /// This method returns `true` if there are any duplicate elements in the slice,
    /// and `false` if all elements are unique.
    ///
    /// # Time Complexity
    ///
    /// This method has O(n) average time complexity, where n is the length of the slice.
    /// In the worst case (with many hash collisions), it could degrade to O(n²), but
    /// this is extremely rare with a good hash function.
    ///
    /// # Space Complexity
    ///
    /// This method uses O(n) additional space to store the seen elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use est::slice::SliceExt;
    ///
    /// let slice_with_dups = [1, 2, 3, 2, 4];
    /// assert!(slice_with_dups.has_dup());
    ///
    /// let slice_without_dups = [1, 2, 3, 4, 5];
    /// assert!(!slice_without_dups.has_dup());
    ///
    /// let empty_slice: [i32; 0] = [];
    /// assert!(!empty_slice.has_dup());
    ///
    /// let single_element = [42];
    /// assert!(!single_element.has_dup());
    /// ```
    ///
    /// # Type Requirements
    ///
    /// The element type `T` must implement [`Hash`] and [`Eq`] traits to be used
    /// in the internal [`HashSet`].
    fn has_dup(&self) -> bool
    where
        T: Hash + Eq;
}

impl<T> SliceExt<T> for [T] {
    fn has_dup(&self) -> bool
    where
        T: Hash + Eq,
    {
        let mut seen = HashSet::with_capacity(self.len());

        for item in self {
            if !seen.insert(item) {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_dup_with_duplicates() {
        let slice = [1, 2, 3, 2, 4];
        assert!(slice.has_dup());
    }

    #[test]
    fn test_has_dup_without_duplicates() {
        let slice = [1, 2, 3, 4, 5];
        assert!(!slice.has_dup());
    }

    #[test]
    fn test_has_dup_empty_slice() {
        let slice: [i32; 0] = [];
        assert!(!slice.has_dup());
    }

    #[test]
    fn test_has_dup_single_element() {
        let slice = [42];
        assert!(!slice.has_dup());
    }

    #[test]
    fn test_has_dup_all_same_elements() {
        let slice = [5, 5, 5, 5];
        assert!(slice.has_dup());
    }

    #[test]
    fn test_has_dup_two_elements_same() {
        let slice = [1, 1];
        assert!(slice.has_dup());
    }

    #[test]
    fn test_has_dup_two_elements_different() {
        let slice = [1, 2];
        assert!(!slice.has_dup());
    }

    #[test]
    fn test_has_dup_strings() {
        let slice = ["hello", "world", "hello"];
        assert!(slice.has_dup());

        let slice_no_dup = ["hello", "world", "rust"];
        assert!(!slice_no_dup.has_dup());
    }

    #[test]
    fn test_has_dup_large_slice() {
        // Test with a larger slice to ensure performance
        let mut vec = Vec::new();
        for i in 0..1000 {
            vec.push(i);
        }
        assert!(!vec.has_dup());

        // Add a duplicate
        vec.push(500);
        assert!(vec.has_dup());
    }

    #[test]
    fn test_has_dup_duplicate_at_end() {
        let slice = [1, 2, 3, 4, 5, 1];
        assert!(slice.has_dup());
    }

    #[test]
    fn test_has_dup_duplicate_at_beginning() {
        let slice = [1, 1, 2, 3, 4, 5];
        assert!(slice.has_dup());
    }

    #[test]
    fn test_has_dup_with_vec() {
        let vec = vec![1, 2, 3, 2, 4];
        assert!(vec.has_dup());

        let vec_no_dup = vec![1, 2, 3, 4, 5];
        assert!(!vec_no_dup.has_dup());
    }

    #[test]
    fn test_has_dup_with_slice_reference() {
        let array = [1, 2, 3, 2, 4];
        let slice = &array[..];
        assert!(slice.has_dup());
    }

    #[test]
    fn test_has_dup_chars() {
        let chars = ['a', 'b', 'c', 'a'];
        assert!(chars.has_dup());

        let chars_no_dup = ['a', 'b', 'c', 'd'];
        assert!(!chars_no_dup.has_dup());
    }
}
