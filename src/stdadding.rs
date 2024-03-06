use std::ops::Add;

pub fn starting_point(){
    example();
}
#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl Add for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

fn example() {
    let a = Complex { real: 1.0, imag: 2.0 };
    let b = Complex { real: 3.0, imag: 4.0 };
    let result = a + b;
    println!("The result of adding the complex numbers is: {:?}", result);
}




