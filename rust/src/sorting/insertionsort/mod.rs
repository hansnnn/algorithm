fn insertionsort(mut array: Vec<i32>) -> Vec<i32> {
    for i in 1..array.len() {
        let key = array[i];
        let mut j = i as i32 - 1;
        while j >= 0 && array[j as usize] > key {
            array[j as usize + 1] = array[j as usize];
            j -= 1;
        }
        array[(j + 1) as usize] = key;
    }
    array
}

#[cfg(test)]
mod insertionsort {
    use crate::sorting::insertionsort::insertionsort;
    use rand::{thread_rng, Rng};
    use std::time::Instant;
    #[test]
    fn test_insertionsort() {
        let mut rng = thread_rng();
        for _ in 0..100 {
            let mut unsorted = (0..100)
                .map(|_| rng.gen_range(0..100))
                .collect::<Vec<i32>>();
            let mut sorted = unsorted.clone();
            let rust_now = Instant::now();
            unsorted.sort();
            let rust_elapsed = rust_now.elapsed();
            let our_now = Instant::now();
            sorted = insertionsort(sorted.clone());
            let our_elapsed = our_now.elapsed();
            assert_eq!(&sorted, &unsorted);
            println!(
                "quicksort: rust_elapsed: {:?}, our_elapsed: {:?}",
                rust_elapsed, our_elapsed,
            );
        }
    }
}
