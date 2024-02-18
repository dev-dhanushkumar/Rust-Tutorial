fn main(){
    let mut arr = vec![45,23,11,98,65,52,25,33];
    println!("Before Sorting: {:?}", arr);

    for i in 0..arr.len() - 1 {
        for j in 0..arr.len()-i-1 {
            if arr[j]>arr[j+1]  {
                let temp = arr[j];
                arr[j] = arr[j+1];
                arr[j+1] = temp;
            }
        }
    }
    println!("After sorting:{:?}", arr);
    
}