fn main() {
    let mut arr = vec![98,56,12,132,45,23,86,67,92,13];
    println!("Before Sorting: {:?}", arr);

    for i in 0..arr.len()-1 {
        let mut min_idx = i;
        for j in i+1..arr.len() {
            if arr[j] < arr[min_idx] {
                min_idx = j;
            }
        }

        let temp = arr[i];
        arr[i] = arr[min_idx];
        arr[min_idx] = temp;
    }

    println!("After Sorting: {:?}", arr);
}