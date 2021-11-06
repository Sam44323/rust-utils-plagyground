pub mod Vectors {
  pub fn creating_vector() -> Vec<i32> {
    let mut v = Vec::new();
    v.push(5); // pushing a new value
    v.push(6);
    v.push(7);
    v.push(10);
    println!("Initial Vector Value: {:?}", v);
    v.pop(); // removing the last value
    println!("Vector after popping: {:?}", v);
    v
  }

  pub fn reading_vector() {
    // reading element using get method for vectors
    let vect: Vec<i32> = creating_vector();
    match vect.get(2) {
      Some(third) => println!("The third element is {}", third),
      None => println!("There is no third element."),
    }
  }

  pub fn method_callers() {
    creating_vector();
    reading_vector();
  }
}
