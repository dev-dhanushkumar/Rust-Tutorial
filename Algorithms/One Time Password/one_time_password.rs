fn find_prims(start: i32, end: i32) -> Vec<i32>{
    let mut primes = Vec::new();
    // Given range using the Sieve of Eratosthenes Algorithm
    let min_value = std::cmp::min(start, 0);
    let max_value = end - min_value;
    let mut  is_prime = vec![true; (max_value + 1) as usize];
    println!("{:?}", is_prime);

    let mut i = 2;
    while i * i <= end {
        if is_prime[(i - min_value) as usize] {
            let mut j = i* i;
            while j <= end {
                is_prime[(j - min_value) as usize] = false;
                j += i;
            }
        }
        i +=1;
    }

    for j in start..= end {
        if j > 1 && is_prime[(j - min_value) as usize] {
            primes.push(j)
        }
    }

    primes
}




fn main() {
    let first:i32 = 10;
    let second:i32 = 30;

    let vector: Vec<i32> = find_prims(first, second);
    println!("{:?}", vector);
    let pass:i32 = vector[0] + vector[vector.len() - 1];
    println!("Security code: {:?}", pass);
}