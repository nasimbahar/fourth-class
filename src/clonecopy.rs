pub fn starting_point() {
    example();
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
}

fn example() {
    // Create an instance of Point
    let original_point = Point { x: 1.0, y: 2.0 };

    // Use the Copy trait to make a bitwise copy of the original point
    let copied_point = original_point;  // No explicit method call needed, thanks to Copy trait

    // Modify the original point to demonstrate they are indeed separate instances
    let modified_original_point = Point { x: 3.0, y: 4.0 };

    // Use the Clone trait to explicitly clone the modified point
    let cloned_point = modified_original_point.clone();  // Explicit clone() call

    println!("Original Point: {:?}", original_point);
    println!("Copied Point: {:?}", copied_point);
    println!("Modified Original Point: {:?}", modified_original_point);
    println!("Cloned Point: {:?}", cloned_point);
}
