
pub fn starting_point(){

  
    example();
 
 
 }

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

impl From<Point2D> for Point3D {
    fn from(point: Point2D) -> Self {
        Point3D {
            x: point.x,
            y: point.y,
            z: 0.0, // Assuming the z coordinate is zero when converting from 2D to 3D
        }
    }
}
fn example() {
    let point2d = Point2D { x: 1.0, y: 2.0 };

    // Using From to convert Point2D to Point3D
    let point3d: Point3D = Point3D::from(point2d);
    println!("Converted using From: {:?}", point3d);

    // Alternatively, using Into to perform the same conversion
    // This works because we've implemented From<Point2D> for Point3D,
    // so Rust knows how to convert Point2D into Point3D automatically
    let point2d_new = Point2D { x: 3.0, y: 4.0 };
    let point3d_from_into: Point3D = point2d_new.into();
    println!("Converted using Into: {:?}", point3d_from_into);
}
