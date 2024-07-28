pub mod selection_sort;
pub mod insertion_sort;
pub mod shell_sort;

pub use selection_sort::selection_sort;
pub use insertion_sort::insertion_sort;
pub use shell_sort::shell_sort;

#[cfg(test)]
fn it_sorts_zero_i32_helper(f: fn(&mut [i32])) {
    let mut vec= Vec::<i32>::new();
    f(&mut vec);
    assert_eq!(Vec::<i32>::new(), vec);
}

#[cfg(test)]
fn it_sorts_one_i32_helper(f: fn(&mut [i32])) {
    let mut vec = vec![1];
    f(&mut vec);
    assert_eq!(vec![1], vec);
}

#[cfg(test)]
fn it_sorts_two_i32_helper(f: fn(&mut [i32])) {
    let mut vec = vec![2, 1];
    f(&mut vec);
    assert_eq!(vec![1, 2], vec);
}

#[cfg(test)]
fn it_sorts_six_i32_helper(f: fn(&mut [i32])) {
    let mut vec = vec![3, 1, 5, 0, 2, 4];
    f(&mut vec);
    assert_eq!(vec![0, 1, 2, 3, 4, 5], vec);
}

#[cfg(test)]
fn it_sorts_six_sorted_i32_helper(f: fn(&mut [i32])) {
    let mut vec = vec![0, 1, 2, 3, 4, 5];
    f(&mut vec);
    assert_eq!(vec![0, 1, 2, 3, 4, 5], vec);
}

#[cfg(test)]
fn it_sorts_six_reverse_sorted_i32_helper(f: fn(&mut [i32])) {
    let mut vec = vec![5, 4, 3, 2, 1, 0];
    f(&mut vec);
    assert_eq!(vec![0, 1, 2, 3, 4, 5], vec);
}