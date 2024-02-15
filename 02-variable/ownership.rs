fn main(){
    /*let v = vec![23,86,33,11,87,36];
    //In above v variable contain ownership
    let v2 = v; //Now ownership are transfer to v2

    println!("{:?}", v);//This line getting error for ownweship.
    */



    //Ownership transfer to function
    /*let v = vec![233,12,67,467,234,7890];
    let v2 = v;
    display(v2);
    println!("{:?}", v2);//Here getting error for ownership v2 -> display()
    */


    //Ownership transfer to function to get that value
    let v = vec![43,54,234,676,224,89];
    let v2 = v;
    let ret_val = ret_display(v2);
    println!("Return value: {:?}", ret_val);

    let u1  = 10;
    let u2 = u1;

    println!("Primitive type only  copy elements: {}", u1);

}

// //
// fn display(val: Vec<i32>){
//     println!("v2 -> val value: {:?}", val);
// }

fn ret_display(val: Vec<i32>) ->Vec<i32>{
    println!("v2 -> val {:?}", val);
    val
}