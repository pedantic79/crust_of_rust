use orst::*;
use rand::{distributions::Standard, thread_rng, Rng};

use std::{cell::Cell, cmp, rc::Rc};

#[derive(Clone)]
struct SortEvaluator<T> {
    t: T,
    cmps: Rc<Cell<usize>>,
}

impl<T: PartialEq> PartialEq for SortEvaluator<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cmps.set(self.cmps.get() + 1);
        self.t == other.t
    }
}

impl<T: Eq> Eq for SortEvaluator<T> {}

impl<T: PartialOrd> PartialOrd for SortEvaluator<T> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.cmps.set(self.cmps.get() + 1);
        self.t.partial_cmp(&other.t)
    }
}

impl<T: Ord> Ord for SortEvaluator<T> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.cmps.set(self.cmps.get() + 1);
        self.t.cmp(&other.t)
    }
}

fn main() {
    let counter = Rc::new(Cell::new(0));
    let rng = &mut thread_rng();

    for n in [1, 10, 100, 1000, 10000] {
        for _ in 0..10 {
            let values: Vec<_> = rng
                .sample_iter(Standard)
                .take(n)
                .map(|t: usize| SortEvaluator {
                    t,
                    cmps: Rc::clone(&counter),
                })
                .collect();
            let took = bench(BubbleSort, &values, &counter);
            println!("bubble {} {}", n, took);
            let took = bench(InsertionSort { smart: true }, &values, &counter);
            println!("insertion-smart {} {}", n, took);
            let took = bench(InsertionSort { smart: false }, &values, &counter);
            println!("insertion-dumb {} {}", n, took);
            let took = bench(SelectionSort, &values, &counter);
            println!("selection {} {}", n, took);
            let took = bench(QuickSort, &values, &counter);
            println!("quick {} {}", n, took);
            let took = bench(StdSorter, &values, &counter);
            println!("std {} {}", n, took);
        }
    }

    eprintln!("DONE");
}

fn bench<T: Ord + Clone, S: Sorter>(
    sorter: S,
    values: &[SortEvaluator<T>],
    counter: &Cell<usize>,
) -> usize {
    let mut values = values.to_vec();
    counter.set(0);
    sorter.sort(&mut values);
    let count = counter.get();

    assert!(values.windows(2).all(|x| x[0] <= x[1]));
    count
}
