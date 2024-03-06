pub fn starting_point(){

  
    example();
 
 
 }


fn display_length<T: Sized>(item: &T) {
    println!("The size is: {}", std::mem::size_of_val(item));
}

fn example() {
    let number: u32 = 42;
    display_length(&number);
}
