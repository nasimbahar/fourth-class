pub fn starting_point(){

  
        let a: u32 = 100;
        let b: u32 = 50;
        let result = generic_add(&a, b);
        println!("The result of adding {} and {} is {}", a, b, result);
    
    
}

trait MyAdd<O> {
    type Output;
    fn my_add(&self, other: O) -> Self::Output;
}

impl MyAdd<u16> for u32 {
    type Output = u64;

    fn my_add(&self, other: u16) -> Self::Output {
        println!("return type will be 64");
        // Cast both to u64 to prevent overflow and return u64
        (*self as u64) + (other as u64)
    }
}
impl MyAdd<u32> for u32 {
    type Output = u32;

    fn my_add(&self, other: u32) -> Self::Output {
        println!("return type will be 32");
        *self + other
    }
}


fn generic_add<T, O>(first: &T, second: O) -> T::Output
where
    T: MyAdd<O>,
{
    first.my_add(second)
}
