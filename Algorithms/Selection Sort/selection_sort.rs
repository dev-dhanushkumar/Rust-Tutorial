fn main() {
    let mut arr = vec![9,54,21,11,7];
    println!("Before Sorting: {:?}", arr);
    let len = vector_length(&arr);

    for i in 0..len-1 {
        let mut min_idx = i;
        for j in i+1..len {
            if arr[j] < arr[min_idx] {
                min_idx = j;
            }
        }

        let temp = arr[i];
        arr[i] = arr[min_idx];
        arr[min_idx] = temp;
        println!("Sorting Processing case {}: {:?}",i, arr);
    }

    println!("After Sorting: {:?}", arr);
}

fn vector_length<T>(v: &[T]) -> usize {
    if v.is_empty() {
        0
    } else {
        1 + vector_length(&v[1..])
    }
}