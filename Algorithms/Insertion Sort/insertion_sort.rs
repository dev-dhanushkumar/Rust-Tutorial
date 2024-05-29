pub fn main(){
    let mut myvec = vec![44,22,75,11,90,78,45,68,19,63,35,64];
    println!("{:?}", myvec);

    for i in 0..myvec.len(){
        for j in i+1..myvec.len(){
            if myvec[i]>myvec[j] {
                let temp = myvec[i];
                myvec[i] = myvec[j];
                myvec[j] = temp;
            }
        }
    }

    println!("{:?}", myvec);
}