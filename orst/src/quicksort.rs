use crate::Sorter;

pub struct QuickSort;

fn quicksort<T: Ord>(slice: &mut [T]) {
    match slice.len() {
        0 | 1 => return,
        2 => {
            if slice[0] > slice[1] {
                slice.swap(0, 1);
            }
            return;
        }
        _ => {}
    }

    let (pivot, rest) = slice.split_first_mut().expect("slice is non-empty");
    let mut left = 0;
    let mut right = rest.len() - 1;
    while left <= right {
        if &rest[left] <= pivot {
            // already on the correct side
            left += 1;
        } else if &rest[right] > pivot {
            // right already on the correct side.
            // avoid swapping right to the left then back
            right -= 1;
        } else {
            // left holds a right, and right hods a left, swap them
            rest.swap(left, right);
            left += 1;
            right -= 1;
        }
        if right == 0 {
            break;
        }
    }

    // place the pivot at its final location
    slice.swap(0, left);

    let (left, right) = slice.split_at_mut(left);
    quicksort(left);
    quicksort(&mut right[1..])
}

impl Sorter for QuickSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        quicksort(slice)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut things = vec![4, 2, 5, 3, 1];
        QuickSort.sort(&mut things);
        assert_eq!(things, &[1, 2, 3, 4, 5]);
    }
}
