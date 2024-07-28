pub fn shell_sort<T: Ord>(items: &mut [T]) {
    if items.len() < 2 {
        return;
    }

    let mut h = 1;
    while h < items.len() / 3 {
        h = 3*h +1;
    }

    while h >= 1 {
        for current in h..items.len() {
            let mut i = current;
            while i >= h {
                if items[i] < items[i-h] {
                    items.swap(i, i-h);
                }
                i -= h;
            }
        }
        h = h / 3;
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::*;

    #[test]
    fn it_sorts_zero_i32() {
        it_sorts_zero_i32_helper(shell_sort);
    }

    #[test]
    fn it_sorts_one_i32() {
        it_sorts_one_i32_helper(shell_sort);
    }

    #[test]
    fn it_sorts_two_i32() {
        it_sorts_two_i32_helper(shell_sort);
    }

    #[test]
    fn it_sorts_six_i32() {
        it_sorts_six_i32_helper(shell_sort);
    }

    #[test]
    fn it_sorts_six_sorted_i32() {
        it_sorts_six_sorted_i32_helper(shell_sort);
    }

    #[test]
    fn it_sorts_six_reverse_sorted_i32() {
        it_sorts_six_reverse_sorted_i32_helper(shell_sort);
    }

}

