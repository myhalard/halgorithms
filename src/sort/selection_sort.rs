pub fn selection_sort<T: Ord>(items: &mut [T]) {
    if items.len() < 2 {
        return;
    }
    for current in 0..items.len()-1 {
        let mut min = current;
        for i in current+1..items.len() {
            if items[i] < items[min] {
                min = i;
            }
        }
        items.swap(current, min);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::*;

    #[test]
    fn it_sorts_zero_i32() {
        it_sorts_zero_i32_helper(selection_sort);
    }

    #[test]
    fn it_sorts_one_i32() {
        it_sorts_one_i32_helper(selection_sort);
    }

    #[test]
    fn it_sorts_two_i32() {
        it_sorts_two_i32_helper(selection_sort);
    }

    #[test]
    fn it_sorts_six_i32() {
        it_sorts_six_i32_helper(selection_sort);
    }

    #[test]
    fn it_sorts_six_sorted_i32() {
        it_sorts_six_sorted_i32_helper(selection_sort);
    }

    #[test]
    fn it_sorts_six_reverse_sorted_i32() {
        it_sorts_six_reverse_sorted_i32_helper(selection_sort);
    }

}
