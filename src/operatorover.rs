use std::ops::Add;


pub fn starting_point(){
    example();
}
#[derive(Debug)]
pub struct Multiplier(i32);

// Implement the Add trait for Multiplier. Normally, this would mean implementing addition,
// but we'll actually implement multiplication logic here.
impl Add for Multiplier {
    // Specify the output type of the operation. Here, it's still Multiplier.
    type Output = Self;

    // Define the addition (in our case, multiplication) operation for Multiplier.
    // This takes ownership of self and another Multiplier instance (rhs), and returns a new Multiplier.
    fn add(self, rhs: Self) -> Self::Output {
        // Multiply the inner values of self and rhs, then return a new Multiplier with the result.
        Multiplier(self.0 * rhs.0)
    }
}

fn example() {
    // Create two Multiplier instances.
    let multiplier_one = Multiplier(8);
    let multiplier_two = Multiplier(10);

    // Use the overloaded '+' operator to multiply the two Multiplier instances.
    let result = multiplier_one + multiplier_two;

    // Print the result, which is the product of 5 and 10.
    println!("The result of multiplying is: {:?}", result);
}
