pub fn if_else() {
    //Biggest amoung thtee number using if-else
    let num1 = 58; 
    let num2 = 54;
    let num3 = 77;
    if num1 > num2 && num1 > num3 {
        println!("Big number is: {0}", num1);
    }else if num2 > num3 {
        println!("Big Number is: {0}", num2);
    }else{
        println!("Big Number is: {0}", num3);
    }
}