pub trait Sorter {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord;
}

pub fn sort<S, T>(sorter: &S, slice: &mut [T])
where
    S: Sorter,
    T: Ord,
{
    sorter.sort(slice)
}

mod bubblesort;
mod insertionsort;
mod quicksort;
mod selectionsort;

pub use bubblesort::BubbleSort;
pub use insertionsort::InsertionSort;
pub use quicksort::QuickSort;
pub use selectionsort::SelectionSort;

pub struct StdSorter;
impl Sorter for StdSorter {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        slice.sort();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn std_works() {
        let mut things = vec![4, 2, 3, 1];
        sort(&StdSorter, &mut things);
        assert_eq!(things, &[1, 2, 3, 4]);
    }
}
