/// Sorts an array of a type with full ordering (a > b && b > c --> a > c)
/// using the heapsort algorithm. This sorting is performed IN PLACE for space
/// efficiency, so make a backup array if you need the initial sort later.
pub fn sort<T: Ord>(array: &mut [T]) {
    let length = array.len();
    construct_max_heap(array, length-1);
    for i in (1..length).rev() {
        array.swap(0, i);
        max_heapify(array, 0, i-1);
    }
}

/// Construct a valid max-heap out of an array up to index n
fn construct_max_heap<T: Ord>(array: &mut [T], n: usize) {
    for i in (0..n/2).rev() {
        max_heapify(array, i, n)
    }
}

/// Swap elements to make a max-heap from index i to n
fn max_heapify<T: Ord>(array: &mut [T], i: usize, n: usize) {
    // Left child
    let l = 2 * i + 1;
    // Right child
    let r = 2 * i + 2;
    let mut largest;
    // If the index is valid and the left child is larger than its parent
    if l <= n && array[l] > array[i] {
        largest = l;
    }
    else {
        largest = i;
    }
    // If the index is valid and the right child is larger than its parent
    if r <= n && array[r] > array[largest] {
        largest = r;
    }
    if largest != i {
        array.swap(i, largest);
        max_heapify(array, largest, n);
    }
}

#[cfg(test)]
mod heapsort_tests {
    use crate::heapsort::sort;
    #[test]
    fn normal() {
        let mut start = [1, 5, 3, 2, 4];
        let end = [1, 2, 3, 4, 5];
        sort(&mut start);
        assert_eq!(
            start,
            end,
        );
    }
}
