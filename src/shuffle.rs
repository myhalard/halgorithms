use rand::Rng;

pub fn knuth_shuffle<T>(items: &mut [T]) {
    if items.len() < 2 {
        return;
    }

    let mut rng = rand::thread_rng();
    for current in 1..items.len() {
        let i = rng.gen_range(0..current+1);
        items.swap(current, i);
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, cmp::min, cmp::max};

    use super::*;
 
    fn factorial(value: usize) -> usize {
        if value < 2 {
            return 1;
        }
        let mut result = 1;
        for i in 2..=value {
            result *= i;
        }
        result
    }

    fn it_shuffles_uniformly_helper(slice: &[i32]) {
        let f = factorial(slice.len());
        let iter_len = f * f * 100_000;
        let mut distribution = HashMap::new();
        for _ in 0..iter_len {
            let mut clone = vec![0; slice.len()];
            clone.clone_from_slice(slice);
            knuth_shuffle(&mut clone);
            let count = distribution.entry(clone).or_insert(0);
            *count += 1;
        }

        let mut min_value = iter_len;
        let mut max_value = 0;
        for (k,v) in distribution {
            min_value = min(min_value, v);
            max_value = max(max_value, v);
           println!("vec={:?}, pct={}%", k, v as f64 / iter_len as f64 * 100f64);

        }
        let max_diff = (max_value - min_value) as f64 / iter_len as f64;
        assert!(max_diff < 0.01);
     }

    #[test]
    #[ignore]
    fn it_shuffles_uniformly_two_i32() {
        it_shuffles_uniformly_helper(&vec![0, 1]);
    }

    #[test]
    #[ignore]
    fn it_shuffles_uniformly_three_i32() {
        it_shuffles_uniformly_helper(&vec![0, 1, 2]);
    }

}