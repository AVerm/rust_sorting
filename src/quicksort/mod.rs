/// Sorts an array of a type with full ordering (a > b && b > c --> a > c)
/// using the quicksort algorithm. This sorting is performed IN PLACE for space
/// efficiency, so make a backup array if you need the initial sort later.
/// Worst case runtime: Θ(n^2)
/// Expected runtime: Θ(n*log(n))
pub fn sort<T: Ord>(array: &mut [T]) {
    quicksort(array, 0, array.len()-1);
}

fn quicksort<T: Ord>(array: &mut [T], lower: usize, upper: usize) {
    if lower < upper {
        // Reorganize array around one element
        let partition_index = partition(array, lower, upper);
        // Sort lower unsorted half
        quicksort(array, lower, partition_index-1);
        // Sort upper unsorted half
        quicksort(array, partition_index + 1, upper);
    }
}

fn partition<T: Ord>(array: &mut [T], lower: usize, upper: usize) -> usize{
    let mut below = lower;
    for index in lower..=upper-1 {
        if array[index] < array[upper] {
            array.swap(below, index);
            below += 1;
        }
    }
    array.swap(below, upper);
    below
}

#[cfg(test)]
mod quicksort_tests {
    use crate::quicksort::sort;
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
