pub fn starting_point(){

  
    example();
 
 
 }

struct MyResource {
    name: String, 
}

impl MyResource {
    // Constructor to create a new MyResource with a given name
    fn new(name: &str) -> MyResource {
        println!("Creating MyResource: {}", name);
        MyResource {
            name: name.to_string(),
        }
    }
}

impl Drop for MyResource {
    fn drop(&mut self) {
        // Cleanup code here
        println!("Cleaning up MyResource: {}", self.name);
    }
}

fn example() {
    {
        let _resource = MyResource::new("my_file.txt");
        // _resource is in scope and can be used here
        // When _resource goes out of scope at the end of this block, `drop` will be called automatically
    }

    // This line will be executed after the drop method has been called
    println!("MyResource 'my_file.txt' has been dropped.");
}
