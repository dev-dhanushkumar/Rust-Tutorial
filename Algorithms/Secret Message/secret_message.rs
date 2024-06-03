fn vector_length<T>(v: &[T]) -> usize {
    if v.is_empty() {
      0
    } else {
      1 + vector_length(&v[1..])
    }
  }
  
fn main() {
  let my_vec = vec![1, 2, 3, 4, 9];
  let length = vector_length(&my_vec);
  println!("Length of my_vec: {}", length);
}
  