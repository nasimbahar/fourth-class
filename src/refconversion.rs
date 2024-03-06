
pub fn starting_point(){

  
    //refexample();
    muteexample();
 
 
 }


fn print_length<T: AsRef<str>>(s: T) {
    let s_ref = s.as_ref();
    println!("Length: {}", s_ref.len());
}

fn refexample() {
    let string = "Hello, world!".to_string();
    let str_slice = "Hello, Rustaceans!";

    print_length(string); // Works with String
    print_length(str_slice); // Works with &str
}

fn make_uppercase<T: AsMut<str>>(s: &mut T) {
    s.as_mut().make_ascii_uppercase();
}

fn muteexample() {
    let mut string = "hello".to_string();
    make_uppercase(&mut string);
    println!("Uppercase: {}", string);
}
