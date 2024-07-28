pub fn insertion_sort<T: Ord>(items: &mut [T]) {
    if items.len() < 2 {
        return;
    }
    for current in 1..items.len() {
        for i in (1..=current).rev() {
            if items[i] < items[i-1] {
                items.swap(i, i-1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::*;

    #[test]
    fn it_sorts_zero_i32() {
        it_sorts_zero_i32_helper(insertion_sort);
    }

    #[test]
    fn it_sorts_one_i32() {
        it_sorts_one_i32_helper(insertion_sort);
    }

    #[test]
    fn it_sorts_two_i32() {
        it_sorts_two_i32_helper(insertion_sort);
    }

    #[test]
    fn it_sorts_six_i32() {
        it_sorts_six_i32_helper(insertion_sort);
    }

    #[test]
    fn it_sorts_six_sorted_i32() {
        it_sorts_six_sorted_i32_helper(insertion_sort);
    }

    #[test]
    fn it_sorts_six_reverse_sorted_i32() {
        it_sorts_six_reverse_sorted_i32_helper(insertion_sort);
    }

}
