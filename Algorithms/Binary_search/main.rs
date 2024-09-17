fn binary_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;
        if &arr[mid] == target {
            return Some(mid);
        } else if &arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    None
}

fn main() {
    let numbers = vec![1, 3, 4, 6, 8, 9, 11];
    let target = 6;

    match binary_search(&numbers, &target) {
        Some(index) => println!("Found {} at index {}", target, index),
        None => println!("{} not found in the array", target),
    }
}