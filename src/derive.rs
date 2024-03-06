pub fn starting_point(){

  
   example();


}

#[derive(Debug, Clone, PartialEq)]
struct Book {
    title: String,
    author: String,
    pages: u32,
}

fn example() {
    let book1 = Book {
        title: "The Rust Programming Language".to_string(),
        author: "Steve Klabnik and Carol Nichols".to_string(),
        pages: 552,
    };

    // Clone the book
    let book2 = book1.clone();

    // Since we derived PartialEq, we can compare books for equality
    assert_eq!(book1, book2);

    // Since we derived Debug, we can print the book's information using {:?}
    println!("Book1: {:?}", book1);
    println!("Book2: {:?}", book2);
}
